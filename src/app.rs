use crate::config::Config;
use crate::config::Hooks;
use crate::config::Mode;
pub use crate::directory_buffer::DirectoryBuffer;
use crate::dirs;
use crate::explorer;
use crate::input::{InputOperation, Key};
use crate::lua;
pub use crate::msg::in_::external::Command;
pub use crate::msg::in_::external::ExplorerConfig;
pub use crate::msg::in_::external::NodeFilter;
pub use crate::msg::in_::external::NodeFilterApplicable;
use crate::msg::in_::external::NodeSearcherApplicable;
pub use crate::msg::in_::external::NodeSorter;
pub use crate::msg::in_::external::NodeSorterApplicable;
pub use crate::msg::in_::ExternalMsg;
pub use crate::msg::in_::InternalMsg;
pub use crate::msg::in_::MsgIn;
pub use crate::msg::out::MsgOut;
pub use crate::node::Node;
pub use crate::node::ResolvedNode;
pub use crate::pipe::Pipe;
use crate::search::SearchAlgorithm;
use crate::ui::Layout;
use anyhow::{bail, Result};
use gethostname::gethostname;
use indexmap::set::IndexSet;
use path_absolutize::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;
use time::OffsetDateTime;
use tui_input::{Input, InputRequest};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TEMPLATE_TABLE_ROW: &str = "TEMPLATE_TABLE_ROW";
pub const UNSUPPORTED_STR: &str = "???";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub msg: MsgIn,
    pub key: Option<Key>,
}

impl Task {
    pub fn new(msg: MsgIn, key: Option<Key>) -> Self {
        Self { msg, key }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Info,
    Warning,
    Success,
    Error,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Log {
    pub level: LogLevel,
    pub message: String,
    pub created_at: OffsetDateTime,
}

impl Log {
    pub fn new(level: LogLevel, message: String) -> Self {
        Self {
            level,
            message,
            created_at: OffsetDateTime::now_local()
                .ok()
                .unwrap_or_else(OffsetDateTime::now_utc),
        }
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level_str = match self.level {
            LogLevel::Info => "INFO   ",
            LogLevel::Warning => "WARNING",
            LogLevel::Success => "SUCCESS",
            LogLevel::Error => "ERROR  ",
        };
        write!(f, "[{0}] {level_str} {1}", &self.created_at, &self.message)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum HelpMenuLine {
    KeyMap(String, Vec<String>, String),
    Paragraph(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct History {
    pub loc: usize,
    pub paths: Vec<String>,
}

impl History {
    fn loc_exists(&self) -> bool {
        self.peek()
            .map(|p| PathBuf::from(p).exists())
            .unwrap_or(false)
    }

    fn cleanup(mut self) -> Self {
        while self.loc > 0
            && self
                .paths
                .get(self.loc.saturating_sub(1))
                .and_then(|p1| self.peek().map(|p2| p1 == p2))
                .unwrap_or(false)
        {
            self.paths.remove(self.loc);
            self.loc = self.loc.saturating_sub(1);
        }

        while self.loc < self.paths.len().saturating_sub(1)
            && self
                .paths
                .get(self.loc.saturating_add(1))
                .and_then(|p1| self.peek().map(|p2| p1 == p2))
                .unwrap_or(false)
        {
            self.paths.remove(self.loc.saturating_add(1));
        }

        self
    }

    fn peek(&self) -> Option<&String> {
        self.paths.get(self.loc)
    }

    fn push(mut self, path: String) -> Self {
        if self.peek() != Some(&path) {
            self.paths = self.paths.into_iter().take(self.loc + 1).collect();
            self.paths.push(path);
            self.loc = self.paths.len().saturating_sub(1);
        }
        self
    }

    fn visit_last(mut self) -> Self {
        self.loc = self.loc.saturating_sub(1);

        while self.loc > 0 && !self.loc_exists() {
            self.paths.remove(self.loc);
            self.loc = self.loc.saturating_sub(1);
        }
        self.cleanup()
    }

    fn visit_next(mut self) -> Self {
        self.loc = self
            .loc
            .saturating_add(1)
            .min(self.paths.len().saturating_sub(1));

        while self.loc < self.paths.len().saturating_sub(1) && !self.loc_exists() {
            self.paths.remove(self.loc);
        }

        self.cleanup()
    }

    fn _is_deepest_dir(&self, path: &str) -> bool {
        !self
            .paths
            .iter()
            .any(|p| p.ends_with('/') && p.starts_with(path) && path != p)
    }

    fn _uniq_deep_dirs(&self) -> IndexSet<String> {
        self.paths
            .clone()
            .into_iter()
            .filter(|p| p.ends_with('/') && self._is_deepest_dir(p))
            .collect::<IndexSet<String>>()
    }

    fn visit_next_deep_branch(self, pwd: &str) -> Self {
        let uniq_deep_dirs = self._uniq_deep_dirs();

        if let Some(path) = uniq_deep_dirs
            .iter()
            .skip_while(|p| p.trim_end_matches('/') != pwd)
            .nth(1)
        {
            self.push(path.to_string())
        } else {
            self
        }
    }

    fn visit_previous_deep_branch(self, pwd: &str) -> Self {
        let uniq_deep_dirs = self._uniq_deep_dirs();
        if let Some(path) = uniq_deep_dirs
            .iter()
            .rev()
            .skip_while(|p| p.trim_end_matches('/') != pwd)
            .nth(1)
        {
            self.push(path.to_string())
        } else {
            self
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LuaContextHeavy {
    pub version: String,
    pub pwd: String,
    pub initial_pwd: String,
    pub vroot: Option<String>,
    pub focused_node: Option<Node>,
    pub directory_buffer: Option<DirectoryBuffer>,
    pub selection: IndexSet<Node>,
    pub mode: Mode,
    pub layout: Layout,
    pub input_buffer: Option<String>,
    pub pid: u32,
    pub session_path: String,
    pub explorer_config: ExplorerConfig,
    pub history: History,
    pub last_modes: Vec<Mode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LuaContextLight {
    pub version: String,
    pub pwd: String,
    pub initial_pwd: String,
    pub vroot: Option<String>,
    pub focused_node: Option<Node>,
    pub selection: IndexSet<Node>,
    pub mode: Mode,
    pub layout: Layout,
    pub input_buffer: Option<String>,
    pub pid: u32,
    pub session_path: String,
    pub explorer_config: ExplorerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputBuffer {
    pub buffer: Option<Input>,
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    pub bin: String,
    pub version: String,
    pub config: Config,
    pub hooks: Hooks,
    pub vroot: Option<String>,
    pub initial_vroot: Option<String>,
    pub pwd: String,
    pub initial_pwd: String,
    pub directory_buffer: Option<DirectoryBuffer>,
    pub last_focus: HashMap<String, Option<String>>,
    pub selection: IndexSet<Node>,
    pub msg_out: VecDeque<MsgOut>,
    pub mode: Mode,
    pub layout: Layout,
    pub input: InputBuffer,
    pub pid: u32,
    pub session_path: String,
    pub pipe: Pipe,
    pub explorer_config: ExplorerConfig,
    pub logs: Vec<Log>,
    pub logs_hidden: bool,
    pub history: History,
    pub last_modes: Vec<Mode>,
    pub hostname: String,
}

impl App {
    pub fn create(
        bin: String,
        vroot: Option<PathBuf>,
        pwd: PathBuf,
        lua: &mlua::Lua,
        config_file: Option<PathBuf>,
        extra_config_files: Vec<PathBuf>,
    ) -> Result<Self> {
        let (mut config, hooks) = lua::init(lua)?;
        let mut hooks = hooks.unwrap_or_default();

        let config_file = if let Some(path) = config_file {
            Some(path)
        } else if let Some(dir) = dirs::config_dir() {
            let path = dir.join("xplr/init.lua");
            if path.exists() {
                Some(path)
            } else {
                None
            }
        } else {
            let path = PathBuf::from("/etc/xplr/init.lua");
            if path.exists() {
                Some(path)
            } else {
                None
            }
        };

        let config_files = config_file.into_iter().chain(extra_config_files);

        let mut load_errs = vec![];
        for config_file in config_files {
            match lua::extend(lua, &config_file.to_string_lossy()) {
                Ok((c, maybe_hooks)) => {
                    config = c;
                    if let Some(h) = maybe_hooks {
                        hooks = hooks.extend(h);
                    }
                }
                Err(e) => {
                    load_errs.push(e.to_string());
                }
            }
        }

        let mode = match config.modes.get(
            &config
                .general
                .initial_mode
                .clone()
                .unwrap_or_else(|| "default".into()),
        ) {
            Some(m) => m.clone().sanitized(
                config.general.read_only,
                config.general.global_key_bindings.clone(),
            ),
            None => {
                bail!("'default' mode is missing")
            }
        };

        let layout = match config.layouts.get(
            &config
                .general
                .initial_layout
                .clone()
                .unwrap_or_else(|| "default".into()),
        ) {
            Some(l) => l.clone(),
            None => {
                bail!("'default' layout is missing")
            }
        };

        let pid = std::process::id();
        let mut session_path = dirs::runtime_dir()
            .join("xplr")
            .join("session")
            .join(pid.to_string())
            .to_string_lossy()
            .to_string();

        if fs::create_dir_all(&session_path).is_err() {
            session_path = env::temp_dir()
                .join("xplr")
                .join("session")
                .join(pid.to_string())
                .to_string_lossy()
                .to_string();
            fs::create_dir_all(&session_path)?;
        }

        let mut explorer_config = ExplorerConfig::default();
        if !config.general.show_hidden {
            explorer_config.filters.replace(NodeFilterApplicable::new(
                NodeFilter::RelativePathDoesNotStartWith,
                ".".into(),
            ));
        }

        if let Some(sorters) = &config.general.initial_sorting {
            explorer_config.sorters.clone_from(sorters);
        };

        let hostname = gethostname().to_string_lossy().to_string();

        if let Some(vroot) = vroot.as_ref() {
            if !pwd.starts_with(vroot) {
                bail!(
                    "{:?} is outside of virtual root {:?}",
                    pwd.to_string_lossy(),
                    vroot.to_string_lossy()
                )
            }
        }

        let pwd = pwd.to_string_lossy().to_string();
        let vroot = vroot.map(|v| v.to_string_lossy().to_string());
        let initial_vroot = vroot.clone();
        env::set_current_dir(&pwd)?;

        let initial_pwd = pwd.clone();

        let input = InputBuffer {
            buffer: Default::default(),
            prompt: config.general.prompt.format.clone().unwrap_or_default(),
        };

        let hist = if &pwd == "/" {
            pwd.clone()
        } else {
            format!("{0}/", &pwd)
        };

        let mut app = Self {
            bin,
            version: VERSION.to_string(),
            config,
            vroot,
            initial_vroot,
            pwd,
            initial_pwd,
            directory_buffer: Default::default(),
            last_focus: Default::default(),
            selection: Default::default(),
            msg_out: Default::default(),
            mode,
            layout,
            input,
            pid,
            session_path: session_path.clone(),
            pipe: Pipe::from_session_path(&session_path)?,
            explorer_config,
            logs: Default::default(),
            logs_hidden: Default::default(),
            history: History::default().push(hist),
            last_modes: Default::default(),
            hostname,
            hooks,
        };

        let has_errs = !load_errs.is_empty();
        for err in load_errs {
            app = app.log_error(err)?
        }

        if has_errs && !app.config.general.disable_debug_error_mode {
            app = app.switch_mode_builtin("debug_error")?;
        }

        Ok(app)
    }

    pub fn focused_node(&self) -> Option<&Node> {
        self.directory_buffer
            .as_ref()
            .and_then(|d| d.focused_node())
    }

    pub fn focused_node_str(&self) -> String {
        self.focused_node()
            .map(|n| n.absolute_path.clone())
            .unwrap_or_default()
    }

    pub fn handle_batch_external_msgs(mut self, msgs: Vec<ExternalMsg>) -> Result<Self> {
        for task in msgs
            .into_iter()
            .map(|msg| Task::new(MsgIn::External(msg), None))
        {
            self = match task.msg {
                MsgIn::Internal(msg) => self.handle_internal(msg)?,
                MsgIn::External(msg) => self.handle_external(msg, task.key)?,
            };
        }
        self.refresh()
    }

    pub fn handle_task(self, task: Task) -> Result<Self> {
        let app = match task.msg {
            MsgIn::Internal(msg) => self.handle_internal(msg)?,
            MsgIn::External(msg) => self.handle_external(msg, task.key)?,
        };
        app.refresh()
    }

    fn handle_internal(self, msg: InternalMsg) -> Result<Self> {
        match msg {
            InternalMsg::SetDirectory(dir) => self.set_directory(dir),
            InternalMsg::AddLastFocus(parent, focus_path) => {
                self.add_last_focus(parent, focus_path)
            }
            InternalMsg::HandleKey(key) => self.handle_key(key),
            InternalMsg::RefreshSelection => self.refresh_selection(),
        }
    }

    fn handle_external(mut self, msg: ExternalMsg, key: Option<Key>) -> Result<Self> {
        let is_msg_read_only = msg.is_read_only();
        if self.config.general.read_only && !is_msg_read_only {
            self.log_error("could not execute code in read-only mode.".into())
        } else {
            use ExternalMsg::*;

            if !is_msg_read_only {
                // We don't want to operate on imaginary paths.
                self = self.refresh_selection()?;
            }

            self = match msg {
                ExplorePwd => self.explore_pwd(),
                ExplorePwdAsync => self.explore_pwd_async(),
                ExploreParentsAsync => self.explore_parents_async(),
                TryCompletePath => self.try_complete_path(),
                Refresh => self.refresh(),
                ClearScreen => self.clear_screen(),
                FocusFirst => self.focus_first(true),
                FocusLast => self.focus_last(),
                FocusPrevious => self.focus_previous(),
                FocusPreviousSelection => self.focus_previous_selection(),
                FocusPreviousByRelativeIndex(i) => {
                    self.focus_previous_by_relative_index(i)
                }

                FocusPreviousByRelativeIndexFromInput => {
                    self.focus_previous_by_relative_index_from_input()
                }
                FocusNext => self.focus_next(),
                FocusNextSelection => self.focus_next_selection(),
                FocusNextByRelativeIndex(i) => self.focus_next_by_relative_index(i),
                FocusNextByRelativeIndexFromInput => {
                    self.focus_next_by_relative_index_from_input()
                }
                FocusPath(p) => self.focus_path(&p, true),
                FocusPathFromInput => self.focus_path_from_input(),
                FocusByIndex(i) => self.focus_by_index(i),
                FocusByIndexFromInput => self.focus_by_index_from_input(),
                FocusByFileName(n) => self.focus_by_file_name(&n, true),
                ScrollUp => self.scroll_up(),
                ScrollDown => self.scroll_down(),
                ScrollUpHalf => self.scroll_up_half(),
                ScrollDownHalf => self.scroll_down_half(),
                ChangeDirectory(dir) => self.change_directory(&dir, true),
                Enter => self.enter(),
                Back => self.back(),
                LastVisitedPath => self.last_visited_path(),
                NextVisitedPath => self.next_visited_path(),
                PreviousVisitedDeepBranch => self.previous_visited_deep_branch(),
                NextVisitedDeepBranch => self.next_visited_deep_branch(),
                FollowSymlink => self.follow_symlink(),
                SetVroot(p) => self.set_vroot(&p),
                UnsetVroot => self.unset_vroot(),
                ToggleVroot => self.toggle_vroot(),
                ResetVroot => self.reset_vroot(),
                SetInputPrompt(p) => self.set_input_prompt(p),
                UpdateInputBuffer(op) => self.update_input_buffer(op),
                UpdateInputBufferFromKey => self.update_input_buffer_from_key(key),
                BufferInput(input) => self.buffer_input(&input),
                BufferInputFromKey => self.buffer_input_from_key(key),
                SetInputBuffer(input) => self.set_input_buffer(input),
                RemoveInputBufferLastCharacter => {
                    self.remove_input_buffer_last_character()
                }
                RemoveInputBufferLastWord => self.remove_input_buffer_last_word(),
                ResetInputBuffer => self.reset_input_buffer(),
                SwitchMode(mode) => self.switch_mode(&mode),
                SwitchModeKeepingInputBuffer(mode) => {
                    self.switch_mode_keeping_input_buffer(&mode)
                }
                SwitchModeBuiltin(mode) => self.switch_mode_builtin(&mode),
                SwitchModeBuiltinKeepingInputBuffer(mode) => {
                    self.switch_mode_builtin_keeping_input_buffer(&mode)
                }
                SwitchModeCustom(mode) => self.switch_mode_custom(&mode),
                SwitchModeCustomKeepingInputBuffer(mode) => {
                    self.switch_mode_custom_keeping_input_buffer(&mode)
                }
                PopMode => self.pop_mode(),
                PopModeKeepingInputBuffer => self.pop_mode_keeping_input_buffer(),
                SwitchLayout(mode) => self.switch_layout(&mode),
                SwitchLayoutBuiltin(mode) => self.switch_layout_builtin(&mode),
                SwitchLayoutCustom(mode) => self.switch_layout_custom(&mode),
                Call(cmd) => self.call(cmd),
                Call0(cmd) => self.call0(cmd),
                CallSilently(cmd) => self.call_silently(cmd),
                CallSilently0(cmd) => self.call_silently0(cmd),
                BashExec(cmd) => self.bash_exec(cmd),
                BashExec0(cmd) => self.bash_exec0(cmd),
                BashExecSilently(cmd) => self.bash_exec_silently(cmd),
                BashExecSilently0(cmd) => self.bash_exec_silently0(cmd),
                CallLua(func) => self.call_lua(func),
                CallLuaSilently(func) => self.call_lua_silently(func),
                LuaEval(code) => self.lua_eval(code),
                LuaEvalSilently(code) => self.lua_eval_silently(code),
                Select => self.select(),
                SelectAll => self.select_all(),
                SelectPath(p) => self.select_path(p),
                UnSelect => self.un_select(),
                UnSelectAll => self.un_select_all(),
                UnSelectPath(p) => self.un_select_path(p),
                ToggleSelection => self.toggle_selection(),
                ToggleSelectAll => self.toggle_select_all(),
                ToggleSelectionByPath(p) => self.toggle_selection_by_path(p),
                ClearSelection => self.clear_selection(),
                AddNodeFilter(f) => self.add_node_filter(f),
                AddNodeFilterFromInput(f) => self.add_node_filter_from_input(f),
                RemoveNodeFilter(f) => self.remove_node_filter(f),
                RemoveNodeFilterFromInput(f) => self.remove_node_filter_from_input(f),
                ToggleNodeFilter(f) => self.toggle_node_filter(f),
                RemoveLastNodeFilter => self.remove_last_node_filter(),
                ResetNodeFilters => self.reset_node_filters(),
                ClearNodeFilters => self.clear_node_filters(),
                AddNodeSorter(f) => self.add_node_sorter(f),
                RemoveNodeSorter(f) => self.remove_node_sorter(f),
                ReverseNodeSorter(f) => self.reverse_node_sorter(f),
                ToggleNodeSorter(f) => self.toggle_node_sorter(f),
                RemoveLastNodeSorter => self.remove_last_node_sorter(),
                ReverseNodeSorters => self.reverse_node_sorters(),
                ResetNodeSorters => self.reset_node_sorters(),
                ClearNodeSorters => self.clear_node_sorters(),
                Search(p) => self.search(p),
                SearchFromInput => self.search_from_input(),
                SearchFuzzy(p) => self.search_with(p, SearchAlgorithm::Fuzzy, false),
                SearchFuzzyFromInput => {
                    self.search_from_input_with(SearchAlgorithm::Fuzzy, false)
                }
                SearchRegex(p) => self.search_with(p, SearchAlgorithm::Regex, false),
                SearchRegexFromInput => {
                    self.search_from_input_with(SearchAlgorithm::Regex, false)
                }
                SearchFuzzyUnordered(p) => {
                    self.search_with(p, SearchAlgorithm::Fuzzy, true)
                }
                SearchFuzzyUnorderedFromInput => {
                    self.search_from_input_with(SearchAlgorithm::Fuzzy, true)
                }
                SearchRegexUnordered(p) => {
                    self.search_with(p, SearchAlgorithm::Regex, true)
                }
                SearchRegexUnorderedFromInput => {
                    self.search_from_input_with(SearchAlgorithm::Regex, true)
                }
                EnableSearchOrder => self.enable_search_order(),
                DisableSearchOrder => self.disable_search_order(),
                ToggleSearchOrder => self.toggle_search_order(),
                ToggleSearchAlgorithm => self.toggle_search_algorithm(),
                AcceptSearch => self.accept_search(),
                CancelSearch => self.cancel_search(),
                EnableMouse => self.enable_mouse(),
                DisableMouse => self.disable_mouse(),
                ToggleMouse => self.toggle_mouse(),
                StartFifo(f) => self.start_fifo(f),
                StopFifo => self.stop_fifo(),
                ToggleFifo(f) => self.toggle_fifo(f),
                LogInfo(l) => self.log_info(l),
                LogSuccess(l) => self.log_success(l),
                LogWarning(l) => self.log_warning(l),
                LogError(l) => self.log_error(l),
                Quit => self.quit(),
                PrintPwdAndQuit => self.print_pwd_and_quit(),
                PrintFocusPathAndQuit => self.print_focus_path_and_quit(),
                PrintSelectionAndQuit => self.print_selection_and_quit(),
                PrintResultAndQuit => self.print_result_and_quit(),
                PrintAppStateAndQuit => self.print_app_state_and_quit(),
                Debug(path) => self.debug(path),
                Terminate => bail!(""),
            }?;

            if !is_msg_read_only {
                // We don't want to keep imaginary paths in the selection.
                // But the write action is probably still in queue.
                // So we need to refresh selection after the write action.
                let msg = InternalMsg::RefreshSelection;
                let msg = MsgIn::Internal(msg);
                let task = Task::new(msg, None);
                self.msg_out.push_back(MsgOut::Enqueue(task));
            }

            Ok(self)
        }
    }

    fn handle_key(mut self, key: Key) -> Result<Self> {
        let kb = self.mode.key_bindings.clone();
        let key_str = key.to_string();
        let msgs = kb
            .on_key
            .get(&key_str)
            .map(|a| a.messages.clone())
            .or_else(|| {
                if key.is_alphabet() {
                    kb.on_alphabet.as_ref().map(|a| a.messages.clone())
                } else if key.is_number() {
                    kb.on_number.as_ref().map(|a| a.messages.clone())
                } else if key.is_special_character() {
                    kb.on_special_character.as_ref().map(|a| a.messages.clone())
                } else if key.is_navigation() {
                    kb.on_navigation.as_ref().map(|a| a.messages.clone())
                } else if key.is_function() {
                    kb.on_function.as_ref().map(|a| a.messages.clone())
                } else {
                    None
                }
            })
            .or_else(|| {
                if key.is_alphanumeric() {
                    kb.on_alphanumeric.as_ref().map(|a| a.messages.clone())
                } else {
                    None
                }
            })
            .or_else(|| {
                if key.is_character() {
                    kb.on_character.as_ref().map(|a| a.messages.clone())
                } else {
                    None
                }
            })
            .or_else(|| kb.default.as_ref().map(|a| a.messages.clone()))
            .unwrap_or_else(|| {
                if self.config.general.enable_recover_mode {
                    vec![ExternalMsg::SwitchModeBuiltin("recover".into())]
                } else {
                    vec![ExternalMsg::LogWarning("key map not found.".into())]
                }
            });

        for msg in msgs {
            // Rename breaks without enqueue
            let external = MsgIn::External(msg);
            let task = Task::new(external, Some(key));
            let msg_out = MsgOut::Enqueue(task);
            self.msg_out.push_back(msg_out);
        }

        Ok(self)
    }

    pub fn explore_pwd(mut self) -> Result<Self> {
        let focus = &self.last_focus.get(&self.pwd).cloned().unwrap_or(None);
        let pwd = self.pwd.clone();
        self = self.add_last_focus(pwd.clone(), focus.clone())?;

        match explorer::explore_sync(
            self.explorer_config.clone(),
            self.pwd.clone().into(),
            focus.as_ref().map(PathBuf::from),
            self.directory_buffer.as_ref().map(|d| d.focus).unwrap_or(0),
        ) {
            Ok(dir) => self.set_directory(dir),
            Err(e) => {
                self.directory_buffer = None;
                self.log_error(format!("could not explore {pwd:?}: {e}"))
            }
        }
    }

    fn explore_pwd_async(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ExplorePwdAsync);
        Ok(self)
    }

    fn explore_parents_async(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ExploreParentsAsync);
        Ok(self)
    }

    fn try_complete_path(self) -> Result<Self> {
        match self.input.buffer.as_ref().map(|b| b.value()) {
            None => Ok(self),
            Some("") => Ok(self),
            Some(p) => match explorer::try_complete_path(&self.pwd, p) {
                Ok(Some(completed_path)) => self.set_input_buffer(completed_path),
                Ok(None) => Ok(self),
                Err(e) => self.log_error(e.to_string()),
            },
        }
    }

    fn refresh(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn clear_screen(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ClearScreen);
        Ok(self)
    }

    pub fn focus_first(mut self, save_history: bool) -> Result<Self> {
        let mut history = self.history.clone();
        if let Some(dir) = self.directory_buffer_mut() {
            if save_history {
                if let Some(n) = dir.focused_node() {
                    history = history.push(n.absolute_path.clone());
                }
            }

            dir.focus = 0;

            if save_history {
                if let Some(n) = self.focused_node() {
                    self.history = history.push(n.absolute_path.clone())
                }
            }
        };
        Ok(self)
    }

    fn focus_last(mut self) -> Result<Self> {
        let mut history = self.history.clone();
        if let Some(dir) = self.directory_buffer_mut() {
            if let Some(n) = dir.focused_node() {
                history = history.push(n.absolute_path.clone());
            }

            dir.focus = dir.total.saturating_sub(1);

            if let Some(n) = dir.focused_node() {
                self.history = history.push(n.absolute_path.clone());
            }
        };
        Ok(self)
    }

    fn focus_previous(mut self) -> Result<Self> {
        let bounded = self.config.general.enforce_bounded_index_navigation;
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = if dir.focus == 0 {
                if bounded {
                    dir.focus
                } else {
                    dir.total.saturating_sub(1)
                }
            } else {
                dir.focus.saturating_sub(1)
            };
        };
        Ok(self)
    }

    fn focus_previous_selection(mut self) -> Result<Self> {
        let total = self.selection.len();
        if total == 0 {
            return Ok(self);
        }

        let bounded = self.config.general.enforce_bounded_index_navigation;

        if let Some(n) = self
            .directory_buffer
            .as_ref()
            .and_then(|d| d.focused_node())
        {
            if let Some(idx) = self.selection.get_index_of(n) {
                let idx = if idx == 0 {
                    if bounded {
                        idx
                    } else {
                        total.saturating_sub(1)
                    }
                } else {
                    idx.saturating_sub(1)
                };
                if let Some(p) = self
                    .selection
                    .get_index(idx)
                    .map(|n| n.absolute_path.clone())
                {
                    self = self.focus_path(&p, true)?;
                }
            } else if let Some(p) =
                self.selection.last().map(|n| n.absolute_path.clone())
            {
                self = self.focus_path(&p, true)?;
            }
        }

        Ok(self)
    }

    pub fn focus_previous_by_relative_index(mut self, index: usize) -> Result<Self> {
        let mut history = self.history.clone();
        if let Some(dir) = self.directory_buffer_mut() {
            if let Some(n) = dir.focused_node() {
                history = history.push(n.absolute_path.clone());
            }

            dir.focus = dir.focus.saturating_sub(index);
            if let Some(n) = self.focused_node() {
                self.history = history.push(n.absolute_path.clone());
            }
        };
        Ok(self)
    }

    fn focus_previous_by_relative_index_from_input(self) -> Result<Self> {
        if let Some(index) = self
            .input
            .buffer
            .as_ref()
            .and_then(|i| i.value().parse::<usize>().ok())
        {
            self.focus_previous_by_relative_index(index)
        } else {
            Ok(self)
        }
    }

    fn focus_next(mut self) -> Result<Self> {
        let bounded = self.config.general.enforce_bounded_index_navigation;
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = if (dir.focus + 1) == dir.total {
                if bounded {
                    dir.focus
                } else {
                    0
                }
            } else {
                dir.focus + 1
            }
        };

        Ok(self)
    }

    fn focus_next_selection(mut self) -> Result<Self> {
        let total = self.selection.len();
        if total == 0 {
            return Ok(self);
        }

        let bounded = self.config.general.enforce_bounded_index_navigation;

        if let Some(n) = self
            .directory_buffer
            .as_ref()
            .and_then(|d| d.focused_node())
        {
            if let Some(idx) = self.selection.get_index_of(n) {
                let idx = if idx + 1 == total {
                    if bounded {
                        idx
                    } else {
                        0
                    }
                } else {
                    idx + 1
                };
                if let Some(p) = self
                    .selection
                    .get_index(idx)
                    .map(|n| n.absolute_path.clone())
                {
                    self = self.focus_path(&p, true)?;
                }
            } else if let Some(p) =
                self.selection.first().map(|n| n.absolute_path.clone())
            {
                self = self.focus_path(&p, true)?;
            }
        }

        Ok(self)
    }

    pub fn focus_next_by_relative_index(mut self, index: usize) -> Result<Self> {
        let mut history = self.history.clone();
        if let Some(dir) = self.directory_buffer_mut() {
            if let Some(n) = dir.focused_node() {
                history = history.push(n.absolute_path.clone());
            }

            dir.focus = dir
                .focus
                .saturating_add(index)
                .min(dir.total.saturating_sub(1));

            if let Some(n) = self.focused_node() {
                self.history = history.push(n.absolute_path.clone());
            }
        };
        Ok(self)
    }

    fn focus_next_by_relative_index_from_input(self) -> Result<Self> {
        if let Some(index) = self
            .input
            .buffer
            .as_ref()
            .and_then(|i| i.value().parse::<usize>().ok())
        {
            self.focus_next_by_relative_index(index)
        } else {
            Ok(self)
        }
    }

    fn follow_symlink(self) -> Result<Self> {
        if let Some(pth) = self
            .focused_node()
            .and_then(|n| n.symlink.clone().map(|s| s.absolute_path))
        {
            self.focus_path(&pth, true)
        } else {
            Ok(self)
        }
    }

    fn set_vroot(mut self, path: &String) -> Result<Self> {
        let vroot = PathBuf::from(path).absolutize()?.to_path_buf();

        if vroot.is_dir() {
            self.vroot = Some(vroot.to_string_lossy().to_string());
            if !PathBuf::from(&self.pwd).starts_with(&vroot) {
                self.change_directory(path, true)
            } else {
                Ok(self)
            }
        } else {
            self.log_error(format!(
                "not a valid directory: {:?}",
                vroot.to_string_lossy()
            ))
        }
    }

    fn unset_vroot(mut self) -> Result<Self> {
        self.vroot = None;
        Ok(self)
    }

    fn toggle_vroot(self) -> Result<Self> {
        if self.vroot.is_some() && self.vroot == self.initial_vroot {
            self.unset_vroot()
        } else if self.vroot.is_some() && self.initial_vroot.is_some() {
            self.reset_vroot()
        } else if self.vroot.is_some() {
            self.unset_vroot()
        } else {
            let vroot = self.pwd.clone();
            self.set_vroot(&vroot)
        }
    }

    fn reset_vroot(mut self) -> Result<Self> {
        if let Some(vroot) = self.initial_vroot.clone() {
            self.set_vroot(&vroot)
        } else {
            self.vroot = None;
            Ok(self)
        }
    }

    fn change_directory(mut self, dir: &str, save_history: bool) -> Result<Self> {
        let dir = PathBuf::from(dir).absolutize()?.to_path_buf();

        if let Some(vroot) = &self.vroot.clone() {
            if !dir.starts_with(vroot) {
                return self.log_error(format!(
                    "{:?} is outside of virtual root {:?}",
                    dir.to_string_lossy(),
                    vroot,
                ));
            }
        }

        match env::set_current_dir(&dir) {
            Ok(()) => {
                let lwd = self.pwd.clone();
                let focus = self.focused_node().map(|n| n.relative_path.clone());
                self = self.add_last_focus(lwd, focus)?;
                self.pwd = dir.to_string_lossy().to_string();
                self.explorer_config.searcher = None;
                if save_history {
                    let hist = if &self.pwd == "/" {
                        self.pwd.clone()
                    } else {
                        format!("{0}/", &self.pwd)
                    };
                    self.history = self.history.push(hist);
                }
                self.explore_pwd()
            }
            Err(e) => self.log_error(format!("could not enter {dir:?}: {e}")),
        }
    }

    fn enter(self) -> Result<Self> {
        if let Some(node) = self.focused_node() {
            if node.is_dir || node.symlink.as_ref().map(|s| s.is_dir).unwrap_or(false) {
                let path = node.absolute_path.clone();
                self.change_directory(&path, true)
            } else {
                Ok(self)
            }
        } else {
            Ok(self)
        }
    }

    fn back(self) -> Result<Self> {
        let pwd = self.pwd.clone();
        if let Some(p) = PathBuf::from(&pwd).parent().and_then(|p| p.to_str()) {
            self.change_directory(p, false)
                .and_then(|a| a.focus_path(&pwd, true))
        } else {
            Ok(self)
        }
    }

    fn last_visited_path(mut self) -> Result<Self> {
        self.history = self.history.visit_last();
        if let Some(target) = self.history.peek().cloned() {
            if let Some(path) = target.strip_suffix('/') {
                self.change_directory(path, false)
            } else {
                self.focus_path(&target, false)
            }
        } else {
            Ok(self)
        }
    }

    fn next_visited_path(mut self) -> Result<Self> {
        self.history = self.history.visit_next();
        if let Some(target) = self.history.peek().cloned() {
            if let Some(path) = target.strip_suffix('/') {
                self.change_directory(path, false)
            } else {
                self.focus_path(&target, false)
            }
        } else {
            Ok(self)
        }
    }

    fn previous_visited_deep_branch(mut self) -> Result<Self> {
        self.history = self.history.visit_previous_deep_branch(&self.pwd);
        if let Some(path) = self.history.peek().cloned() {
            self.change_directory(path.trim_end_matches('/'), false)
        } else {
            Ok(self)
        }
    }

    fn next_visited_deep_branch(mut self) -> Result<Self> {
        self.history = self.history.visit_next_deep_branch(&self.pwd);
        if let Some(path) = self.history.peek().cloned() {
            self.change_directory(path.trim_end_matches('/'), false)
        } else {
            Ok(self)
        }
    }

    fn set_input_prompt(mut self, p: String) -> Result<Self> {
        self.input.prompt = p;
        Ok(self)
    }

    fn update_input_buffer(mut self, op: InputOperation) -> Result<Self> {
        if let Some(buf) = self.input.buffer.as_mut() {
            buf.handle(op.into());
            self.logs_hidden = true;
        } else {
            let mut buf = Input::default();
            if buf.handle(op.into()).is_some() {
                self.input.buffer = Some(buf);
                self.logs_hidden = true;
            };
        }
        Ok(self)
    }

    fn update_input_buffer_from_key(self, key: Option<Key>) -> Result<Self> {
        if let Some(op) = key.and_then(|k| k.to_input_operation()) {
            self.update_input_buffer(op)
        } else {
            Ok(self)
        }
    }

    fn buffer_input(mut self, input: &str) -> Result<Self> {
        if let Some(buf) = self.input.buffer.as_mut() {
            buf.handle(InputRequest::GoToEnd);
            for c in input.chars() {
                buf.handle(InputRequest::InsertChar(c));
            }
        } else {
            self.input.buffer = Some(Input::default().with_value(input.into()));
        };
        self.logs_hidden = true;
        Ok(self)
    }

    fn buffer_input_from_key(self, key: Option<Key>) -> Result<Self> {
        if let Some(c) = key.and_then(|k| k.to_char()) {
            self.buffer_input(&c.to_string())
        } else {
            Ok(self)
        }
    }

    fn set_input_buffer(mut self, string: String) -> Result<Self> {
        self.input.buffer = Some(Input::default().with_value(string));
        self.logs_hidden = true;
        Ok(self)
    }

    fn remove_input_buffer_last_character(mut self) -> Result<Self> {
        if let Some(buf) = self.input.buffer.as_mut() {
            buf.handle(InputRequest::GoToEnd);
            buf.handle(InputRequest::DeletePrevChar);
            self.logs_hidden = true;
        };
        Ok(self)
    }

    fn remove_input_buffer_last_word(mut self) -> Result<Self> {
        if let Some(buf) = self.input.buffer.as_mut() {
            buf.handle(InputRequest::GoToEnd);
            buf.handle(InputRequest::DeletePrevWord);
            self.logs_hidden = true;
        };
        Ok(self)
    }

    fn reset_input_buffer(mut self) -> Result<Self> {
        self.input = InputBuffer {
            buffer: Default::default(),
            prompt: self
                .mode
                .prompt
                .as_ref()
                .or(self.config.general.prompt.format.as_ref())
                .cloned()
                .unwrap_or_default(),
        };
        Ok(self)
    }

    fn focus_by_index(mut self, index: usize) -> Result<Self> {
        let history = self.history.clone();
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = index.min(dir.total.saturating_sub(1));
            if let Some(n) = self.focused_node() {
                self.history = history.push(n.absolute_path.clone());
            }
        };
        Ok(self)
    }

    fn focus_by_index_from_input(self) -> Result<Self> {
        if let Some(index) = self
            .input
            .buffer
            .as_ref()
            .and_then(|i| i.value().parse::<usize>().ok())
        {
            self.focus_by_index(index)
        } else {
            Ok(self)
        }
    }

    pub fn focus_by_file_name(mut self, name: &str, save_history: bool) -> Result<Self> {
        let mut history = self.history.clone();
        if let Some(dir_buf) = self.directory_buffer_mut() {
            if let Some(focus) = dir_buf
                .clone()
                .nodes
                .iter()
                .enumerate()
                .find(|(_, n)| n.relative_path == name)
                .map(|(i, _)| i)
            {
                if save_history {
                    if let Some(n) = dir_buf.focused_node() {
                        history = history.push(n.absolute_path.clone());
                    }
                }
                dir_buf.focus = focus;
                if save_history {
                    if let Some(n) = dir_buf.focused_node() {
                        self.history = history.push(n.absolute_path.clone());
                    }
                }
                Ok(self)
            } else {
                self.log_error(format!("{name:?} not found in $PWD"))
            }
        } else {
            Ok(self)
        }
    }

    pub fn scroll_up(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ScrollUp);
        Ok(self)
    }

    pub fn scroll_down(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ScrollDown);
        Ok(self)
    }

    pub fn scroll_up_half(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ScrollUpHalf);
        Ok(self)
    }

    pub fn scroll_down_half(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ScrollDownHalf);
        Ok(self)
    }

    pub fn focus_path(self, path: &str, save_history: bool) -> Result<Self> {
        let pathbuf = PathBuf::from(path).absolutize()?.to_path_buf();
        if let Some(parent) = pathbuf.parent() {
            if let Some(filename) = pathbuf.file_name() {
                self.change_directory(&parent.to_string_lossy(), false)?
                    .focus_by_file_name(&filename.to_string_lossy(), save_history)
            } else {
                self.log_error(format!("{path:?} not found"))
            }
        } else {
            self.log_error(format!("could not focus on {path:?}"))
        }
    }

    fn focus_path_from_input(self) -> Result<Self> {
        if let Some(p) = self.input.buffer.clone() {
            self.focus_path(p.value(), true)
        } else {
            Ok(self)
        }
    }

    fn push_mode(mut self) -> Self {
        if Some(&self.mode) != self.config.modes.builtin.get("recover")
            && self
                .last_modes
                .last()
                .map(|m| m != &self.mode)
                .unwrap_or(true)
        {
            self.last_modes.push(self.mode.clone())
        }
        self
    }

    fn pop_mode(self) -> Result<Self> {
        self.pop_mode_keeping_input_buffer()
            .and_then(App::reset_input_buffer)
    }

    fn pop_mode_keeping_input_buffer(mut self) -> Result<Self> {
        if let Some(mode) = self.last_modes.pop() {
            self.mode = mode;
        };
        Ok(self)
    }

    fn switch_mode(self, mode: &str) -> Result<Self> {
        self.switch_mode_keeping_input_buffer(mode)
            .and_then(App::reset_input_buffer)
    }

    fn switch_mode_keeping_input_buffer(self, mode: &str) -> Result<Self> {
        if self.config.modes.builtin.contains_key(mode) {
            self.switch_mode_builtin_keeping_input_buffer(mode)
        } else if self.config.modes.custom.contains_key(mode) {
            self.switch_mode_custom_keeping_input_buffer(mode)
        } else {
            self.log_error(format!("mode not found: {mode:?}"))
        }
    }

    fn switch_mode_builtin(self, mode: &str) -> Result<Self> {
        self.switch_mode_builtin_keeping_input_buffer(mode)
            .and_then(App::reset_input_buffer)
    }

    fn switch_mode_builtin_keeping_input_buffer(mut self, mode: &str) -> Result<Self> {
        if let Some(mode) = self.config.modes.builtin.get(mode).cloned() {
            self = self.push_mode();
            self.mode = mode.sanitized(
                self.config.general.read_only,
                self.config.general.global_key_bindings.clone(),
            );

            // Hooks
            if !self.hooks.on_mode_switch.is_empty() {
                let msgs = self.hooks.on_mode_switch.clone();
                self = self.handle_batch_external_msgs(msgs)?
            }

            Ok(self)
        } else {
            self.log_error(format!("builtin mode not found: {mode:?}"))
        }
    }

    fn switch_mode_custom(self, mode: &str) -> Result<Self> {
        self.switch_mode_custom_keeping_input_buffer(mode)
            .and_then(App::reset_input_buffer)
    }

    fn switch_mode_custom_keeping_input_buffer(mut self, mode: &str) -> Result<Self> {
        if let Some(mode) = self.config.modes.custom.get(mode).cloned() {
            self = self.push_mode();
            self.mode = mode.sanitized(
                self.config.general.read_only,
                self.config.general.global_key_bindings.clone(),
            );

            // Hooks
            if !self.hooks.on_mode_switch.is_empty() {
                let msgs = self.hooks.on_mode_switch.clone();
                self = self.handle_batch_external_msgs(msgs)?
            }

            Ok(self)
        } else {
            self.log_error(format!("custom mode not found: {mode:?}"))
        }
    }

    fn switch_layout(self, layout: &str) -> Result<Self> {
        if self.config.layouts.builtin.contains_key(layout) {
            self.switch_layout_builtin(layout)
        } else if self.config.layouts.custom.contains_key(layout) {
            self.switch_layout_custom(layout)
        } else {
            self.log_error(format!("layout not found: {layout:?}"))
        }
    }

    fn switch_layout_builtin(mut self, layout: &str) -> Result<Self> {
        if let Some(l) = self.config.layouts.builtin.get(layout) {
            self.layout = l.clone();

            // Hooks
            if !self.hooks.on_layout_switch.is_empty() {
                let msgs = self.hooks.on_layout_switch.clone();
                self = self.handle_batch_external_msgs(msgs)?
            }

            Ok(self)
        } else {
            self.log_error(format!("builtin layout not found: {layout:?}"))
        }
    }

    fn switch_layout_custom(mut self, layout: &str) -> Result<Self> {
        if let Some(l) = self.config.layouts.get_custom(layout) {
            self.layout = l.clone();

            // Hooks
            if !self.hooks.on_layout_switch.is_empty() {
                let msgs = self.hooks.on_layout_switch.clone();
                self = self.handle_batch_external_msgs(msgs)?
            }

            Ok(self)
        } else {
            self.log_error(format!("custom layout not found: {layout:?}"))
        }
    }

    fn call(mut self, command: Command) -> Result<Self> {
        self.logs_hidden = true;
        self.msg_out.push_back(MsgOut::Call(command));
        Ok(self)
    }

    fn call0(mut self, command: Command) -> Result<Self> {
        self.logs_hidden = true;
        self.msg_out.push_back(MsgOut::Call0(command));
        Ok(self)
    }

    fn call_silently(mut self, command: Command) -> Result<Self> {
        self.logs_hidden = true;
        self.msg_out.push_back(MsgOut::CallSilently(command));
        Ok(self)
    }

    fn call_silently0(mut self, command: Command) -> Result<Self> {
        self.logs_hidden = true;
        self.msg_out.push_back(MsgOut::CallSilently0(command));
        Ok(self)
    }

    fn bash_exec(self, script: String) -> Result<Self> {
        self.call(Command {
            command: "bash".into(),
            args: vec!["-c".into(), script],
        })
    }

    fn bash_exec0(self, script: String) -> Result<Self> {
        self.call0(Command {
            command: "bash".into(),
            args: vec!["-c".into(), script],
        })
    }

    fn bash_exec_silently(self, script: String) -> Result<Self> {
        self.call_silently(Command {
            command: "bash".into(),
            args: vec!["-c".into(), script],
        })
    }

    fn bash_exec_silently0(self, script: String) -> Result<Self> {
        self.call_silently0(Command {
            command: "bash".into(),
            args: vec!["-c".into(), script],
        })
    }

    fn call_lua(mut self, func: String) -> Result<Self> {
        self.logs_hidden = true;
        self.msg_out.push_back(MsgOut::CallLua(func));
        Ok(self)
    }

    fn call_lua_silently(mut self, func: String) -> Result<Self> {
        self.logs_hidden = true;
        self.msg_out.push_back(MsgOut::CallLuaSilently(func));
        Ok(self)
    }

    fn lua_eval(mut self, code: String) -> Result<Self> {
        self.logs_hidden = true;
        self.msg_out.push_back(MsgOut::LuaEval(code));
        Ok(self)
    }

    fn lua_eval_silently(mut self, code: String) -> Result<Self> {
        self.logs_hidden = true;
        self.msg_out.push_back(MsgOut::LuaEvalSilently(code));
        Ok(self)
    }

    pub fn set_directory(mut self, dir: DirectoryBuffer) -> Result<Self> {
        if self
            .directory_buffer
            .as_ref()
            .map(|d| d.explored_at >= dir.explored_at)
            .unwrap_or(false)
        {
            return Ok(self);
        };

        self = self.add_last_focus(
            dir.parent.clone(),
            dir.focused_node().map(|n| n.relative_path.clone()),
        )?;

        if dir.parent == self.pwd {
            self.directory_buffer = Some(dir);
            // Might as well refresh the selection
            self = self.refresh_selection()?;
        };

        Ok(self)
    }

    pub fn add_last_focus(
        mut self,
        parent: String,
        focused_path: Option<String>,
    ) -> Result<Self> {
        self.last_focus.insert(parent, focused_path);
        Ok(self)
    }

    pub fn select(mut self) -> Result<Self> {
        let count = self.selection.len();
        if let Some(n) = self.focused_node().cloned() {
            self.selection.insert(n);
        }

        if self.selection.len() != count {
            self = self.on_selection_change()?;
        }

        Ok(self)
    }

    pub fn select_path(mut self, path: String) -> Result<Self> {
        let path = PathBuf::from(path).absolutize()?.to_path_buf();
        let parent = path.parent().map(|p| p.to_string_lossy().to_string());
        let filename = path.file_name().map(|p| p.to_string_lossy().to_string());
        let count = self.selection.len();

        if let (Some(p), Some(n)) = (parent, filename) {
            self.selection.insert(Node::new(p, n));
        }

        if self.selection.len() != count {
            self = self.on_selection_change()?;
        }
        Ok(self)
    }

    pub fn select_all(mut self) -> Result<Self> {
        let count = self.selection.len();
        if let Some(d) = self.directory_buffer.as_ref() {
            self.selection.extend(d.nodes.clone());
        };

        if self.selection.len() != count {
            self = self.on_selection_change()?;
        }

        Ok(self)
    }

    pub fn un_select_path(mut self, path: String) -> Result<Self> {
        let pathbuf = PathBuf::from(path).absolutize()?.to_path_buf();
        let count = self.selection.len();
        self.selection
            .retain(|n| PathBuf::from(&n.absolute_path) != pathbuf);

        if self.selection.len() != count {
            self = self.on_selection_change()?;
        }

        Ok(self)
    }

    pub fn un_select(mut self) -> Result<Self> {
        let count = self.selection.len();
        if let Some(n) = self.focused_node().cloned() {
            self.selection
                .retain(|s| s.absolute_path != n.absolute_path);
        }

        if self.selection.len() != count {
            self = self.on_selection_change()?;
        }
        Ok(self)
    }

    pub fn un_select_all(mut self) -> Result<Self> {
        let count = self.selection.len();
        if let Some(d) = self.directory_buffer.as_ref() {
            d.nodes.clone().into_iter().for_each(|n| {
                self.selection
                    .retain(|s| s.absolute_path != n.absolute_path);
            });
        };

        if self.selection.len() != count {
            self = self.on_selection_change()?;
        }

        Ok(self)
    }

    fn toggle_selection(self) -> Result<Self> {
        if let Some(p) = self.focused_node().map(|n| n.absolute_path.clone()) {
            self.toggle_selection_by_path(p)
        } else {
            Ok(self)
        }
    }

    fn toggle_select_all(self) -> Result<Self> {
        if let Some(d) = self.directory_buffer.as_ref() {
            if d.nodes.iter().all(|n| self.selection.contains(n)) {
                self.un_select_all()
            } else {
                self.select_all()
            }
        } else {
            Ok(self)
        }
    }

    fn toggle_selection_by_path(self, path: String) -> Result<Self> {
        let pathbuf = PathBuf::from(&path).absolutize()?.to_path_buf();
        if self
            .selection
            .iter()
            .any(|n| PathBuf::from(&n.absolute_path) == pathbuf)
        {
            self.un_select_path(path)
        } else {
            self.select_path(path)
        }
    }

    fn clear_selection(mut self) -> Result<Self> {
        let count = self.selection.len();
        self.selection.clear();
        if self.selection.len() != count {
            self = self.on_selection_change()?;
        }
        Ok(self)
    }

    fn add_node_filter(mut self, filter: NodeFilterApplicable) -> Result<Self> {
        self.explorer_config.filters.replace(filter);
        Ok(self)
    }

    fn add_node_filter_from_input(mut self, filter: NodeFilter) -> Result<Self> {
        if let Some(input) = self.input.buffer.as_ref() {
            self.explorer_config
                .filters
                .insert(NodeFilterApplicable::new(filter, input.value().into()));
        };
        Ok(self)
    }

    fn remove_node_filter(mut self, filter: NodeFilterApplicable) -> Result<Self> {
        self.explorer_config.filters.retain(|f| f != &filter);
        Ok(self)
    }

    fn remove_node_filter_from_input(mut self, filter: NodeFilter) -> Result<Self> {
        if let Some(input) = self.input.buffer.as_ref() {
            let nfa = NodeFilterApplicable::new(filter, input.value().into());
            self.explorer_config.filters.retain(|f| f != &nfa);
        };
        Ok(self)
    }

    fn toggle_node_filter(self, filter: NodeFilterApplicable) -> Result<Self> {
        if self.explorer_config.filters.contains(&filter) {
            self.remove_node_filter(filter)
        } else {
            self.add_node_filter(filter)
        }
    }

    fn remove_last_node_filter(mut self) -> Result<Self> {
        self.explorer_config.filters.pop();
        Ok(self)
    }

    fn reset_node_filters(mut self) -> Result<Self> {
        self.explorer_config.filters.clear();

        if !self.config.general.show_hidden {
            self.add_node_filter(NodeFilterApplicable::new(
                NodeFilter::RelativePathDoesNotStartWith,
                ".".into(),
            ))
        } else {
            Ok(self)
        }
    }
    fn clear_node_filters(mut self) -> Result<Self> {
        self.explorer_config.filters.clear();
        Ok(self)
    }

    fn add_node_sorter(mut self, sorter: NodeSorterApplicable) -> Result<Self> {
        self.explorer_config.sorters.replace(sorter);
        Ok(self)
    }

    fn remove_node_sorter(mut self, sorter: NodeSorter) -> Result<Self> {
        self.explorer_config.sorters.retain(|s| s.sorter != sorter);
        Ok(self)
    }

    fn reverse_node_sorter(mut self, sorter: NodeSorter) -> Result<Self> {
        self.explorer_config.sorters = self
            .explorer_config
            .sorters
            .into_iter()
            .map(|s| if s.sorter == sorter { s.reversed() } else { s })
            .collect();
        Ok(self)
    }

    fn toggle_node_sorter(self, sorter: NodeSorterApplicable) -> Result<Self> {
        if self.explorer_config.sorters.contains(&sorter) {
            self.remove_node_sorter(sorter.sorter)
        } else {
            self.add_node_sorter(sorter)
        }
    }

    fn remove_last_node_sorter(mut self) -> Result<Self> {
        self.explorer_config.sorters.pop();
        Ok(self)
    }

    fn reverse_node_sorters(mut self) -> Result<Self> {
        self.explorer_config.sorters = self
            .explorer_config
            .sorters
            .into_iter()
            .map(|s| s.reversed())
            .collect();
        Ok(self)
    }

    fn reset_node_sorters(mut self) -> Result<Self> {
        self.explorer_config.sorters = self
            .config
            .general
            .initial_sorting
            .clone()
            .unwrap_or_default();
        Ok(self)
    }

    fn clear_node_sorters(mut self) -> Result<Self> {
        self.explorer_config.sorters.clear();
        Ok(self)
    }

    pub fn search(self, pattern: String) -> Result<Self> {
        let (algorithm, unordered) = self
            .explorer_config
            .searcher
            .as_ref()
            .map(|s| (s.algorithm, s.unordered))
            .unwrap_or((
                self.config.general.search.algorithm,
                self.config.general.search.unordered,
            ));

        self.search_with(pattern, algorithm, unordered)
    }

    fn search_from_input(self) -> Result<Self> {
        if let Some(pattern) = self.input.buffer.as_ref().map(Input::to_string) {
            self.search(pattern)
        } else {
            Ok(self)
        }
    }

    pub fn search_with(
        mut self,
        pattern: String,
        algorithm: SearchAlgorithm,
        unordered: bool,
    ) -> Result<Self> {
        let rf = self
            .explorer_config
            .searcher
            .as_ref()
            .map(|s| s.recoverable_focus.clone())
            .unwrap_or_else(|| self.focused_node().map(|n| n.absolute_path.clone()));

        self.explorer_config.searcher = Some(NodeSearcherApplicable::new(
            pattern,
            rf,
            algorithm,
            unordered,
            self.config.general.search.exact_mode,
            self.config.general.search.rank_criteria.clone(),
        ));
        Ok(self)
    }

    fn search_from_input_with(
        self,
        algorithm: SearchAlgorithm,
        unordered: bool,
    ) -> Result<Self> {
        if let Some(pattern) = self.input.buffer.as_ref().map(Input::to_string) {
            self.search_with(pattern, algorithm, unordered)
        } else {
            Ok(self)
        }
    }

    fn enable_search_order(mut self) -> Result<Self> {
        self.explorer_config.searcher = self
            .explorer_config
            .searcher
            .map(|s| s.enable_search_order());
        Ok(self)
    }

    fn disable_search_order(mut self) -> Result<Self> {
        self.explorer_config.searcher = self
            .explorer_config
            .searcher
            .map(|s| s.disable_search_order());
        Ok(self)
    }

    fn toggle_search_order(mut self) -> Result<Self> {
        self.explorer_config.searcher = self
            .explorer_config
            .searcher
            .map(|s| s.toggle_search_order());
        Ok(self)
    }

    fn toggle_search_algorithm(mut self) -> Result<Self> {
        self.explorer_config.searcher =
            self.explorer_config.searcher.map(|s| s.toggle_algorithm());
        Ok(self)
    }

    fn accept_search(mut self) -> Result<Self> {
        let focus = self
            .directory_buffer
            .as_ref()
            .and_then(|dir| dir.focused_node())
            .map(|n| n.relative_path.clone());

        self.explorer_config.searcher = None;
        self = self.explore_pwd()?;

        if let Some(f) = focus {
            self = self.focus_path(&f, true)?;
        }
        Ok(self)
    }

    fn cancel_search(mut self) -> Result<Self> {
        let focus = self
            .explorer_config
            .searcher
            .as_ref()
            .and_then(|s| s.recoverable_focus.clone());

        self.explorer_config.searcher = None;
        self = self.explore_pwd()?;

        if let Some(f) = focus {
            self = self.focus_path(&f, true)?;
        }
        Ok(self)
    }

    fn enable_mouse(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::EnableMouse);
        Ok(self)
    }

    fn disable_mouse(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::DisableMouse);
        Ok(self)
    }

    fn toggle_mouse(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ToggleMouse);
        Ok(self)
    }

    fn start_fifo(mut self, path: String) -> Result<Self> {
        self.msg_out.push_back(MsgOut::StartFifo(path));
        Ok(self)
    }

    fn stop_fifo(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::StopFifo);
        Ok(self)
    }

    fn toggle_fifo(mut self, path: String) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ToggleFifo(path));
        Ok(self)
    }

    pub fn log_info(mut self, message: String) -> Result<Self> {
        self.logs_hidden = false;
        self.logs.push(Log::new(LogLevel::Info, message));
        Ok(self)
    }

    pub fn log_success(mut self, message: String) -> Result<Self> {
        self.logs_hidden = false;
        self.logs.push(Log::new(LogLevel::Success, message));
        Ok(self)
    }

    pub fn log_warning(mut self, message: String) -> Result<Self> {
        self.logs_hidden = false;
        self.logs.push(Log::new(LogLevel::Warning, message));
        Ok(self)
    }

    pub fn log_error(mut self, message: String) -> Result<Self> {
        self.logs_hidden = false;
        self.logs.push(Log::new(LogLevel::Error, message));
        Ok(self)
    }

    fn quit(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::Quit);
        Ok(self)
    }

    fn print_pwd_and_quit(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::PrintPwdAndQuit);
        Ok(self)
    }

    fn print_focus_path_and_quit(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::PrintFocusPathAndQuit);
        Ok(self)
    }

    fn print_selection_and_quit(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::PrintSelectionAndQuit);
        Ok(self)
    }

    fn print_result_and_quit(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::PrintResultAndQuit);
        Ok(self)
    }

    fn print_app_state_and_quit(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::PrintAppStateAndQuit);
        Ok(self)
    }

    fn debug(mut self, path: String) -> Result<Self> {
        self.msg_out.push_back(MsgOut::Debug(path));
        Ok(self)
    }

    fn directory_buffer_mut(&mut self) -> Option<&mut DirectoryBuffer> {
        self.directory_buffer.as_mut()
    }

    pub fn mode_str(&self) -> String {
        format!("{0}\n", &self.mode.name)
    }

    // This is a performance heavy function. Use it only when necessary.
    fn refresh_selection(mut self) -> Result<Self> {
        let count = self.selection.len();
        self.selection.retain(|n| {
            let p = PathBuf::from(&n.absolute_path);
            // Should be able to retain broken symlink
            p.exists() || p.symlink_metadata().is_ok()
        });

        if count != self.selection.len() {
            self = self.on_selection_change()?;
        }

        Ok(self)
    }

    fn on_selection_change(mut self) -> Result<Self> {
        if !self.hooks.on_selection_change.is_empty() {
            let msgs = self.hooks.on_selection_change.clone();
            self = self.handle_batch_external_msgs(msgs)?
        }

        Ok(self)
    }

    pub fn result(&self) -> Vec<&Node> {
        if self.selection.is_empty() {
            self.focused_node().map(|n| vec![n]).unwrap_or_default()
        } else {
            self.selection.iter().collect()
        }
    }

    pub fn directory_nodes_str(&self, delimiter: char) -> String {
        self.directory_buffer
            .as_ref()
            .map(|d| {
                d.nodes
                    .iter()
                    .map(|n| format!("{0}{delimiter}", n.absolute_path))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .unwrap_or_default()
    }

    pub fn pwd_str(&self, delimiter: char) -> String {
        format!("{0}{delimiter}", &self.pwd)
    }

    pub fn selection_str(&self, delimiter: char) -> String {
        self.selection
            .iter()
            .map(|n| format!("{0}{delimiter}", n.absolute_path))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn result_str(&self, delimiter: char) -> String {
        self.result()
            .into_iter()
            .map(|n| format!("{0}{delimiter}", n.absolute_path))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn logs_str(&self, delimiter: char) -> String {
        self.logs
            .iter()
            .map(|l| format!("{l}{delimiter}"))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn global_help_menu_str(&self, delimiter: char) -> String {
        let builtin = self.config.modes.builtin.clone();
        let custom = self.config.modes.custom.clone();
        let read_only = self.config.general.read_only;
        let global_kb = &self.config.general.global_key_bindings;

        let modes = builtin.into_iter().chain(custom);

        std::iter::once((self.mode.name.clone(), self.mode.clone()))
        .chain(modes)
        .map(|(name, mode)| {
            (name, mode.sanitized(read_only, global_kb.clone()))
        })
        .map(|(name, mode)| {
            let help = mode
                .help_menu()
                .iter()
                .map(|l| match l {
                    HelpMenuLine::Paragraph(p) => format!("\t{p}{delimiter}"),
                    HelpMenuLine::KeyMap(k, remaps, h) => {
                        let remaps = remaps.join(", ");
                        format!(" {k:15} | {remaps:25} | {h}{delimiter}")
                    }
                })
                .collect::<Vec<String>>()
                .join("");

            format!(
                "### {name}{delimiter}{delimiter} key             | remaps                    | action\n --------------- | ------------------------- | ------{delimiter}{help}{delimiter}"
            )
        })
        .collect::<Vec<String>>()
        .join(&delimiter.to_string())
    }

    pub fn history_str(&self, delimiter: char) -> String {
        self.history
            .paths
            .iter()
            .map(|p| format!("{p}{delimiter}"))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn write_pipes(&self, delimiter: char) -> Result<()> {
        fs::create_dir_all(self.pipe.path.clone())?;
        fs::write(&self.pipe.msg_in, [delimiter as u8])?;

        let selection_str = self.selection_str(delimiter);
        fs::write(&self.pipe.selection_out, selection_str)?;

        let history_str = self.history_str(delimiter);
        fs::write(&self.pipe.history_out, history_str)?;

        let directory_nodes_str = self.directory_nodes_str(delimiter);
        fs::write(&self.pipe.directory_nodes_out, directory_nodes_str)?;

        let logs_str = self.logs_str(delimiter);
        fs::write(&self.pipe.logs_out, logs_str)?;

        let result_str = self.result_str(delimiter);
        fs::write(&self.pipe.result_out, result_str)?;

        let global_help_menu_str = self.global_help_menu_str(delimiter);
        fs::write(&self.pipe.global_help_menu_out, global_help_menu_str)?;

        Ok(())
    }

    pub fn cleanup_pipes(&self) -> Result<()> {
        while !fs::read_to_string(&self.pipe.msg_in)?.is_empty() {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        fs::remove_file(&self.pipe.msg_in)?;
        fs::remove_file(&self.pipe.selection_out)?;
        fs::remove_file(&self.pipe.result_out)?;
        fs::remove_file(&self.pipe.directory_nodes_out)?;
        fs::remove_file(&self.pipe.global_help_menu_out)?;
        fs::remove_file(&self.pipe.logs_out)?;
        fs::remove_file(&self.pipe.history_out)?;

        fs::remove_dir(&self.pipe.path)?;
        Ok(())
    }

    pub fn to_lua_ctx_heavy(&self) -> LuaContextHeavy {
        LuaContextHeavy {
            version: self.version.clone(),
            pwd: self.pwd.clone(),
            initial_pwd: self.initial_pwd.clone(),
            vroot: self.vroot.clone(),
            focused_node: self.focused_node().cloned(),
            directory_buffer: self.directory_buffer.clone(),
            selection: self.selection.clone(),
            mode: self.mode.clone(),
            layout: self.layout.clone(),
            input_buffer: self.input.buffer.as_ref().map(|i| i.value().into()),
            pid: self.pid,
            session_path: self.session_path.clone(),
            explorer_config: self.explorer_config.clone(),
            history: self.history.clone(),
            last_modes: self.last_modes.clone(),
        }
    }

    pub fn to_lua_ctx_light(&self) -> LuaContextLight {
        LuaContextLight {
            version: self.version.clone(),
            pwd: self.pwd.clone(),
            initial_pwd: self.initial_pwd.clone(),
            vroot: self.vroot.clone(),
            focused_node: self.focused_node().cloned(),
            selection: self.selection.clone(),
            mode: self.mode.clone(),
            layout: self.layout.clone(),
            input_buffer: self.input.buffer.as_ref().map(|i| i.value().into()),
            pid: self.pid,
            session_path: self.session_path.clone(),
            explorer_config: self.explorer_config.clone(),
        }
    }
}
