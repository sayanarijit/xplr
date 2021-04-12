use crate::config::Config;
use crate::config::Mode;
use crate::input::Key;
use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::PathBuf;

pub const TEMPLATE_TABLE_ROW: &str = "TEMPLATE_TABLE_ROW";
pub const UNSUPPORTED_STR: &str = "???";
pub const UPGRADE_GUIDE_LINK: &str = "https://github.com/sayanarijit/xplr/wiki/Upgrade-Guide";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipe {
    pub msg_in: String,
    pub focus_out: String,
    pub selection_out: String,
    pub result_out: String,
    pub mode_out: String,
    pub directory_nodes_out: String,
    pub global_help_menu_out: String,
    pub logs_out: String,
}

impl Pipe {
    fn from_session_path(path: &str) -> Result<Self> {
        let pipesdir = PathBuf::from(path).join("pipe");

        fs::create_dir_all(&pipesdir).unwrap();

        let msg_in = pipesdir.join("msg_in").to_string_lossy().to_string();

        let focus_out = pipesdir.join("focus_out").to_string_lossy().to_string();

        let selection_out = pipesdir.join("selection_out").to_string_lossy().to_string();

        let result_out = pipesdir.join("result_out").to_string_lossy().to_string();

        let mode_out = pipesdir.join("mode_out").to_string_lossy().to_string();

        let directory_nodes_out = pipesdir
            .join("directory_nodes_out")
            .to_string_lossy()
            .to_string();

        let global_help_menu_out = pipesdir
            .join("global_help_menu_out")
            .to_string_lossy()
            .to_string();

        let logs_out = pipesdir.join("logs_out").to_string_lossy().to_string();

        fs::write(&msg_in, "")?;
        fs::write(&focus_out, "")?;
        fs::write(&selection_out, "")?;
        fs::write(&mode_out, "")?;
        fs::write(&directory_nodes_out, "")?;
        fs::write(&global_help_menu_out, "")?;
        fs::write(&result_out, "")?;
        fs::write(&logs_out, "")?;

        Ok(Self {
            msg_in,
            focus_out,
            selection_out,
            result_out,
            mode_out,
            directory_nodes_out,
            global_help_menu_out,
            logs_out,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub parent: String,
    pub relative_path: String,
    pub absolute_path: String,
    pub extension: String,
    pub is_symlink: bool,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_readonly: bool,
    pub mime_essence: String,
}

impl Node {
    pub fn new(parent: String, relative_path: String) -> Self {
        let absolute_path = PathBuf::from(&parent)
            .join(&relative_path)
            .to_string_lossy()
            .to_string();

        let path = PathBuf::from(&absolute_path);

        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();

        let maybe_metadata = path.symlink_metadata().ok();

        let is_symlink = maybe_metadata
            .clone()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false);

        let is_dir = maybe_metadata.clone().map(|m| m.is_dir()).unwrap_or(false);

        let is_file = maybe_metadata.clone().map(|m| m.is_file()).unwrap_or(false);

        let is_readonly = maybe_metadata
            .map(|m| m.permissions().readonly())
            .unwrap_or(false);

        let mime_essence = mime_guess::from_path(&path)
            .first()
            .map(|m| m.essence_str().to_string())
            .unwrap_or_default();

        Self {
            parent,
            relative_path,
            absolute_path,
            extension,
            is_symlink,
            is_dir,
            is_file,
            is_readonly,
            mime_essence,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.relative_path.cmp(&self.relative_path)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DirectoryBuffer {
    pub parent: String,
    pub nodes: Vec<Node>,
    pub total: usize,
    pub focus: usize,
}

impl DirectoryBuffer {
    pub fn new(parent: String, nodes: Vec<Node>, focus: usize) -> Self {
        let total = nodes.len();
        Self {
            parent,
            nodes,
            total,
            focus,
        }
    }

    pub fn focused_node(&self) -> Option<&Node> {
        self.nodes.get(self.focus)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum InternalMsg {
    AddDirectory(String, DirectoryBuffer),
    HandleKey(Key),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum NodeFilter {
    RelativePathIs,
    RelativePathIsNot,

    RelativePathDoesStartWith,
    RelativePathDoesNotStartWith,

    RelativePathDoesContain,
    RelativePathDoesNotContain,

    RelativePathDoesEndWith,
    RelativePathDoesNotEndWith,

    AbsolutePathIs,
    AbsolutePathIsNot,

    AbsolutePathDoesStartWith,
    AbsolutePathDoesNotStartWith,

    AbsolutePathDoesContain,
    AbsolutePathDoesNotContain,

    AbsolutePathDoesEndWith,
    AbsolutePathDoesNotEndWith,
}

impl NodeFilter {
    fn apply(&self, node: &Node, input: &str, case_sensitive: bool) -> bool {
        match self {
            Self::RelativePathIs => {
                if case_sensitive {
                    node.relative_path == input
                } else {
                    node.relative_path.to_lowercase() == input.to_lowercase()
                }
            }

            Self::RelativePathIsNot => {
                if case_sensitive {
                    node.relative_path != input
                } else {
                    node.relative_path.to_lowercase() != input.to_lowercase()
                }
            }

            Self::RelativePathDoesStartWith => {
                if case_sensitive {
                    node.relative_path.starts_with(input)
                } else {
                    node.relative_path
                        .to_lowercase()
                        .starts_with(&input.to_lowercase())
                }
            }

            Self::RelativePathDoesNotStartWith => {
                if case_sensitive {
                    !node.relative_path.starts_with(input)
                } else {
                    !node
                        .relative_path
                        .to_lowercase()
                        .starts_with(&input.to_lowercase())
                }
            }

            Self::RelativePathDoesContain => {
                if case_sensitive {
                    node.relative_path.contains(input)
                } else {
                    node.relative_path
                        .to_lowercase()
                        .contains(&input.to_lowercase())
                }
            }

            Self::RelativePathDoesNotContain => {
                if case_sensitive {
                    !node.relative_path.contains(input)
                } else {
                    !node
                        .relative_path
                        .to_lowercase()
                        .contains(&input.to_lowercase())
                }
            }

            Self::RelativePathDoesEndWith => {
                if case_sensitive {
                    node.relative_path.ends_with(input)
                } else {
                    node.relative_path
                        .to_lowercase()
                        .ends_with(&input.to_lowercase())
                }
            }

            Self::RelativePathDoesNotEndWith => {
                if case_sensitive {
                    !node.relative_path.ends_with(input)
                } else {
                    !node
                        .relative_path
                        .to_lowercase()
                        .ends_with(&input.to_lowercase())
                }
            }

            Self::AbsolutePathIs => {
                if case_sensitive {
                    node.absolute_path == input
                } else {
                    node.absolute_path.to_lowercase() == input.to_lowercase()
                }
            }

            Self::AbsolutePathIsNot => {
                if case_sensitive {
                    node.absolute_path != input
                } else {
                    node.absolute_path.to_lowercase() != input.to_lowercase()
                }
            }

            Self::AbsolutePathDoesStartWith => {
                if case_sensitive {
                    node.absolute_path.starts_with(input)
                } else {
                    node.absolute_path
                        .to_lowercase()
                        .starts_with(&input.to_lowercase())
                }
            }

            Self::AbsolutePathDoesNotStartWith => {
                if case_sensitive {
                    !node.absolute_path.starts_with(input)
                } else {
                    !node
                        .absolute_path
                        .to_lowercase()
                        .starts_with(&input.to_lowercase())
                }
            }

            Self::AbsolutePathDoesContain => {
                if case_sensitive {
                    node.absolute_path.contains(input)
                } else {
                    node.absolute_path
                        .to_lowercase()
                        .contains(&input.to_lowercase())
                }
            }

            Self::AbsolutePathDoesNotContain => {
                if case_sensitive {
                    !node.absolute_path.contains(input)
                } else {
                    !node
                        .absolute_path
                        .to_lowercase()
                        .contains(&input.to_lowercase())
                }
            }

            Self::AbsolutePathDoesEndWith => {
                if case_sensitive {
                    node.absolute_path.ends_with(input)
                } else {
                    node.absolute_path
                        .to_lowercase()
                        .ends_with(&input.to_lowercase())
                }
            }

            Self::AbsolutePathDoesNotEndWith => {
                if case_sensitive {
                    !node.absolute_path.ends_with(input)
                } else {
                    !node
                        .absolute_path
                        .to_lowercase()
                        .ends_with(&input.to_lowercase())
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeFilterApplicable {
    filter: NodeFilter,
    input: String,
    #[serde(default)]
    case_sensitive: bool,
}

impl NodeFilterApplicable {
    pub fn new(filter: NodeFilter, input: String, case_sensitive: bool) -> Self {
        Self {
            filter,
            input,
            case_sensitive,
        }
    }

    fn apply(&self, node: &Node) -> bool {
        self.filter.apply(node, &self.input, self.case_sensitive)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeFilterFromInput {
    filter: NodeFilter,
    #[serde(default)]
    case_sensitive: bool,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExplorerConfig {
    filters: HashSet<NodeFilterApplicable>,
}

impl ExplorerConfig {
    pub fn filter(&self, node: &Node) -> bool {
        self.filters.iter().all(|f| f.apply(node))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExternalMsg {
    /// Explore the present working directory and register the filtered nodes.
    /// This operation is expensive. So, try avoiding using it too often.
    /// Once exploration is done, it will auto `Refresh` the state.
    Explore,

    /// Refresh the app state (uncluding UI).
    /// But it will not re-explore the directory if the working directory is the same.
    /// If there is some change in the working directory and you want to re-explore it,
    /// use the `Explore` message instead.
    Refresh,

    /// Clears the screen.
    ClearScreen,

    /// Focus next node.
    FocusNext,

    /// Focus on the `n`th node relative to the current focus where `n` is a given value.
    ///
    /// Example: `FocusNextByRelativeIndex: 2`
    FocusNextByRelativeIndex(usize),

    /// Focus on the `n`th node relative to the current focus where `n` is read from
    /// the input buffer.
    FocusNextByRelativeIndexFromInput,

    /// Focus on the previous item.
    FocusPrevious,

    /// Focus on the `-n`th node relative to the current focus where `n` is a given value.
    ///
    /// Example: `FocusPreviousByRelativeIndex: 2`
    FocusPreviousByRelativeIndex(usize),

    /// Focus on the `-n`th node relative to the current focus where `n` is read from
    /// the input buffer.
    FocusPreviousByRelativeIndexFromInput,

    /// Focus on the first node.
    FocusFirst,

    /// Focus on the last node.
    FocusLast,

    /// Focus on the given path.
    ///
    /// Example: `FocusPath: /tmp`
    FocusPath(String),

    /// Focus on the path read from input buffer.
    FocusPathFromInput,

    /// Focus on the absolute `n`th node where `n` is a given value.
    ///
    /// Example: `FocusByIndex: 2`
    FocusByIndex(usize),

    /// Focus on the absolute `n`th node where `n` is read from the input buffer.
    FocusByIndexFromInput,

    /// Focus on the file by name from the present working directory.
    ///
    /// Example: `FocusByFileName: README.md`
    FocusByFileName(String),

    /// Change the present working directory ($PWD)
    ///
    /// Example: `ChangeDirectory: /tmp`
    ChangeDirectory(String),

    /// Enter into the currently focused path if it's a directory.
    Enter,

    /// Go back to the parent directory.
    Back,

    /// Go to the last path visited.
    LastVisitedPath,

    /// Go to the next path visited.
    NextVisitedPath,

    /// Append/buffer the given string into the input buffer.
    ///
    /// Example: `BufferInput: foo`
    BufferInput(String),

    /// Append/buffer the characted read from a keyboard input into the
    /// input buffer.
    BufferInputFromKey,

    /// Set/rewrite the input buffer with the given string.
    /// When the input buffer is not-null (even if empty string)
    /// it will show in the UI.
    ///
    /// Example: `SetInputBuffer: foo`
    SetInputBuffer(String),

    /// Reset the input buffer back to null. It will not show in the UI.
    ResetInputBuffer,

    /// Switch input mode.
    /// This will reset the input buffer and call `Refresh` automatically.
    ///
    /// Example: `SwitchMode: default`
    SwitchMode(String),

    /// Call a shell command with the given arguments.
    /// Note that the arguments will be shell-escaped.
    /// So to read the variables, the `-c` option of the shell
    /// can be used.
    /// You may need to pass `Refresh` or `Explore` depening on the expectation.
    ///
    /// Example: `Call: {command: bash, args: ["-c", "read -p test"]}`
    Call(Command),

    /// Like `Call` but without the flicker. The stdin, stdout
    /// stderr will be piped to null. So it's non-interactive.
    ///
    /// Example: `CallSilently: {command: tput, args: ["bell"]}`
    CallSilently(Command),

    /// An alias to `Call: {command: bash, args: ["-c", "${command}"], silent: false}`
    /// where ${command} is the given value.
    ///
    /// Example: `BashExec: "read -p test"`
    BashExec(String),

    /// Like `BashExec` but without the flicker. The stdin, stdout
    /// stderr will be piped to null. So it's non-interactive.
    ///
    /// Example: `BashExecSilently: "tput bell"`
    BashExecSilently(String),

    /// Select the focused node.
    Select,

    /// Unselect the focused node.
    UnSelect,

    /// Toggle selection on the focused node.
    ToggleSelection,

    /// Clear the selection
    ClearSelection,

    /// Add a filter to explude nodes while exploring directories.
    ///
    /// Example: `AddNodeFilter: {filter: RelativePathDoesStartWith, input: foo}`
    AddNodeFilter(NodeFilterApplicable),

    /// Remove an existing filter.
    ///
    /// Example: `RemoveNodeFilter: {filter: RelativePathDoesStartWith, input: foo}`
    RemoveNodeFilter(NodeFilterApplicable),

    /// Remove a filter if it exists, else, add a it.
    ///
    /// Example: `ToggleNodeFilter: {filter: RelativePathDoesStartWith, input: foo}`
    ToggleNodeFilter(NodeFilterApplicable),

    /// Add a node filter reading the input from the buffer.
    ///
    /// Example: `AddNodeFilterFromInput: {filter: RelativePathDoesStartWith}`
    AddNodeFilterFromInput(NodeFilterFromInput),

    /// Remove a node filter reading the input from the buffer.
    ///
    /// Example: `RemoveNodeFilterFromInput: {filter: RelativePathDoesStartWith}`
    RemoveNodeFilterFromInput(NodeFilterFromInput),

    /// Reset the node filters back to the default configuration.
    ResetNodeFilters,

    /// Log information message.
    ///
    /// Example: `LogInfo: launching satellite`
    LogInfo(String),

    /// Log a success message.
    ///
    /// Example: `LogSuccess: satellite reached destination`.
    LogSuccess(String),

    /// Log an error message.
    ///
    /// Example: `LogError: satellite crashed`
    LogError(String),

    /// Quit with returncode zero (success).
    Quit,

    /// Print selected paths if it's not empty, else, print the focused node's path.
    PrintResultAndQuit,

    /// Print the state of application in YAML format. Helpful for debugging or generating
    /// the default configuration file.
    PrintAppStateAndQuit,

    /// Write the application state to a file, without quitting. Also helpful for debugging.
    Debug(String),

    /// Terminate the application with a non-zero return code.
    Terminate,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MsgIn {
    Internal(InternalMsg),
    External(ExternalMsg),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Command {
    pub command: String,

    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MsgOut {
    Explore,
    Refresh,
    ClearScreen,
    Quit,
    PrintResultAndQuit,
    PrintAppStateAndQuit,
    Debug(String),
    Call(Command),
    CallSilently(Command),
    Enque(Task),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Task {
    msg: MsgIn,
    key: Option<Key>,
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
    Success,
    Error,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Log {
    pub level: LogLevel,
    pub message: String,
    pub created_at: DateTime<Local>,
}

impl Log {
    pub fn new(level: LogLevel, message: String) -> Self {
        Self {
            level,
            message,
            created_at: Local::now(),
        }
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level_str = match self.level {
            LogLevel::Info => "INFO   ",
            LogLevel::Success => "SUCCESS",
            LogLevel::Error => "ERROR  ",
        };
        write!(f, "[{}] {} {}", &self.created_at, level_str, &self.message)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum HelpMenuLine {
    KeyMap(String, String),
    Paragraph(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct History {
    loc: usize,
    paths: Vec<String>,
}

impl History {
    pub fn push(mut self, path: String) -> Self {
        self.paths = self.paths.into_iter().take(self.loc + 1).collect();
        self.paths.push(path);
        self.loc = self.paths.len().max(1) - 1;
        self
    }

    pub fn visit_last(mut self) -> Self {
        self.loc = self.loc.max(1) - 1;
        self
    }

    pub fn visit_next(mut self) -> Self {
        self.loc = (self.loc + 1).min(self.paths.len().max(1) - 1);
        self
    }

    pub fn peek(&self) -> Option<&String> {
        self.paths.get(self.loc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    version: String,
    config: Config,
    pwd: String,
    directory_buffers: HashMap<String, DirectoryBuffer>,
    selection: Vec<Node>,
    msg_out: VecDeque<MsgOut>,
    mode: Mode,
    input_buffer: Option<String>,
    pid: u32,
    session_path: String,
    pipe: Pipe,
    explorer_config: ExplorerConfig,
    logs: Vec<Log>,
    history: History,
}

impl App {
    pub fn create(pwd: PathBuf) -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("xplr");

        let config_file = config_dir.join("config.yml");
        let default_config = Config::default();
        let default_config_version = default_config.version.clone();

        let config: Config = if config_file.exists() {
            let c: Config =
                serde_yaml::from_reader(io::BufReader::new(&fs::File::open(&config_file)?))?;
            c.extended()
        } else {
            default_config
        };

        if !config.is_compatible()? {
            bail!(
                "incompatible configuration version in {}
                You config version is : {}
                Required version is   : {}
                Visit {}",
                config_file.to_string_lossy().to_string(),
                config.version,
                default_config_version,
                UPGRADE_GUIDE_LINK,
            )
        };

        let mode = match config.modes.builtin.get(&"default".to_string()) {
            Some(m) => m.clone(),
            None => {
                bail!("'default' mode is missing")
            }
        };

        let pid = std::process::id();
        let session_path = dirs::runtime_dir()
            .unwrap_or_else(|| "/tmp".into())
            .join("xplr")
            .join("session")
            .join(&pid.to_string())
            .to_string_lossy()
            .to_string();

        let mut explorer_config = ExplorerConfig::default();
        if !config.general.show_hidden.unwrap_or_default() {
            explorer_config.filters.insert(NodeFilterApplicable::new(
                NodeFilter::RelativePathDoesNotStartWith,
                ".".into(),
                Default::default(),
            ));
        }

        let mut history = History::default();
        history = history.push(pwd.to_string_lossy().to_string());

        let mut app = Self {
            version: Config::default().version,
            config: config.clone(),
            pwd: pwd.to_string_lossy().to_string(),
            directory_buffers: Default::default(),
            selection: Default::default(),
            msg_out: Default::default(),
            mode,
            input_buffer: Default::default(),
            pid,
            session_path: session_path.clone(),
            pipe: Pipe::from_session_path(&session_path)?,
            explorer_config,
            logs: Default::default(),
            history,
        };

        if let Some(notif) = config.upgrade_notification()? {
            let notif = format!(
                "{}. To stop seeing this log, update your config version from {} to {}.",
                &notif, &config.version, &app.version
            );
            app = app.enqueue(Task::new(
                MsgIn::External(ExternalMsg::LogInfo(notif)),
                None,
            ));
        }

        Ok(app)
    }

    pub fn focused_node(&self) -> Option<&Node> {
        self.directory_buffer().and_then(|d| d.focused_node())
    }

    pub fn focused_node_str(&self) -> String {
        self.focused_node()
            .map(|n| n.absolute_path.clone())
            .unwrap_or_default()
    }

    pub fn enqueue(mut self, task: Task) -> Self {
        self.msg_out.push_back(MsgOut::Enque(task));
        self
    }

    pub fn handle_task(self, task: Task) -> Result<Self> {
        match task.msg {
            MsgIn::Internal(msg) => self.handle_internal(msg),
            MsgIn::External(msg) => self.handle_external(msg, task.key),
        }
    }

    fn handle_internal(self, msg: InternalMsg) -> Result<Self> {
        match msg {
            InternalMsg::AddDirectory(parent, dir) => self.add_directory(parent, dir),
            InternalMsg::HandleKey(key) => self.handle_key(key),
        }
    }

    fn handle_external(self, msg: ExternalMsg, key: Option<Key>) -> Result<Self> {
        match msg {
            ExternalMsg::Explore => self.explore(),
            ExternalMsg::Refresh => self.refresh(),
            ExternalMsg::ClearScreen => self.clear_screen(),
            ExternalMsg::FocusFirst => self.focus_first(),
            ExternalMsg::FocusLast => self.focus_last(),
            ExternalMsg::FocusPrevious => self.focus_previous(),
            ExternalMsg::FocusPreviousByRelativeIndex(i) => {
                self.focus_previous_by_relative_index(i)
            }

            ExternalMsg::FocusPreviousByRelativeIndexFromInput => {
                self.focus_previous_by_relative_index_from_input()
            }
            ExternalMsg::FocusNext => self.focus_next(),
            ExternalMsg::FocusNextByRelativeIndex(i) => self.focus_next_by_relative_index(i),
            ExternalMsg::FocusNextByRelativeIndexFromInput => {
                self.focus_next_by_relative_index_from_input()
            }
            ExternalMsg::FocusPath(p) => self.focus_path(&p),
            ExternalMsg::FocusPathFromInput => self.focus_path_from_input(),
            ExternalMsg::FocusByIndex(i) => self.focus_by_index(i),
            ExternalMsg::FocusByIndexFromInput => self.focus_by_index_from_input(),
            ExternalMsg::FocusByFileName(n) => self.focus_by_file_name(&n),
            ExternalMsg::ChangeDirectory(dir) => self.change_directory(&dir),
            ExternalMsg::Enter => self.enter(),
            ExternalMsg::Back => self.back(),
            ExternalMsg::LastVisitedPath => self.last_visited_path(),
            ExternalMsg::NextVisitedPath => self.next_visited_path(),
            ExternalMsg::BufferInput(input) => self.buffer_input(&input),
            ExternalMsg::BufferInputFromKey => self.buffer_input_from_key(key),
            ExternalMsg::SetInputBuffer(input) => self.set_input_buffer(input),
            ExternalMsg::ResetInputBuffer => self.reset_input_buffer(),
            ExternalMsg::SwitchMode(mode) => self.switch_mode(&mode),
            ExternalMsg::Call(cmd) => self.call(cmd),
            ExternalMsg::CallSilently(cmd) => self.call_silently(cmd),
            ExternalMsg::BashExec(cmd) => self.bash_exec(cmd),
            ExternalMsg::BashExecSilently(cmd) => self.bash_exec_silently(cmd),
            ExternalMsg::Select => self.select(),
            ExternalMsg::UnSelect => self.un_select(),
            ExternalMsg::ToggleSelection => self.toggle_selection(),
            ExternalMsg::ClearSelection => self.clear_selection(),
            ExternalMsg::AddNodeFilter(f) => self.add_node_filter(f),
            ExternalMsg::AddNodeFilterFromInput(f) => self.add_node_filter_from_input(f),
            ExternalMsg::RemoveNodeFilter(f) => self.remove_node_filter(f),
            ExternalMsg::RemoveNodeFilterFromInput(f) => self.remove_node_filter_from_input(f),
            ExternalMsg::ToggleNodeFilter(f) => self.toggle_node_filter(f),
            ExternalMsg::ResetNodeFilters => self.reset_node_filters(),
            ExternalMsg::LogInfo(l) => self.log_info(l),
            ExternalMsg::LogSuccess(l) => self.log_success(l),
            ExternalMsg::LogError(l) => self.log_error(l),
            ExternalMsg::Quit => self.quit(),
            ExternalMsg::PrintResultAndQuit => self.print_result_and_quit(),
            ExternalMsg::PrintAppStateAndQuit => self.print_app_state_and_quit(),
            ExternalMsg::Debug(path) => self.debug(path),
            ExternalMsg::Terminate => bail!(""),
        }
    }

    fn handle_key(mut self, key: Key) -> Result<Self> {
        let kb = self.mode.key_bindings.clone();
        let key_str = key.to_string();
        let default = kb.default.clone();
        let msgs = kb
            .remaps
            .get(&key_str)
            .and_then(|k| kb.on_key.get(k))
            .or_else(|| kb.on_key.get(&key_str))
            .map(|a| Some(a.messages.clone()))
            .unwrap_or_else(|| {
                if key.is_alphabet() {
                    kb.on_alphabet.map(|a| a.messages)
                } else if key.is_number() {
                    kb.on_number.map(|a| a.messages)
                } else if key.is_special_character() {
                    kb.on_special_character.map(|a| a.messages)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| default.map(|a| a.messages).unwrap_or_default());

        for msg in msgs {
            self = self.enqueue(Task::new(MsgIn::External(msg), Some(key)));
        }

        Ok(self)
    }

    fn explore(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::Explore);
        Ok(self)
    }

    fn refresh(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn clear_screen(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ClearScreen);
        Ok(self)
    }

    fn focus_first(mut self) -> Result<Self> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = 0;
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_last(mut self) -> Result<Self> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = dir.total.max(1) - 1;
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_previous(mut self) -> Result<Self> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = dir.focus.max(1) - 1;
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_previous_by_relative_index(mut self, index: usize) -> Result<Self> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = dir.focus.max(index) - index;
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_previous_by_relative_index_from_input(self) -> Result<Self> {
        if let Some(index) = self.input_buffer().and_then(|i| i.parse::<usize>().ok()) {
            self.focus_previous_by_relative_index(index)
        } else {
            Ok(self)
        }
    }

    fn focus_next(mut self) -> Result<Self> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = (dir.focus + 1).min(dir.total.max(1) - 1);
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_next_by_relative_index(mut self, index: usize) -> Result<Self> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = (dir.focus + index).min(dir.total.max(1) - 1);
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_next_by_relative_index_from_input(self) -> Result<Self> {
        if let Some(index) = self.input_buffer().and_then(|i| i.parse::<usize>().ok()) {
            self.focus_next_by_relative_index(index)
        } else {
            Ok(self)
        }
    }

    fn change_directory(mut self, dir: &str) -> Result<Self> {
        if PathBuf::from(dir).is_dir() {
            self.pwd = dir.to_owned();
            self.history = self.history.push(self.pwd.clone());
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn enter(self) -> Result<Self> {
        self.focused_node()
            .map(|n| n.absolute_path.clone())
            .map(|p| self.clone().change_directory(&p))
            .unwrap_or(Ok(self))
    }

    fn back(self) -> Result<Self> {
        PathBuf::from(self.pwd())
            .parent()
            .map(|p| {
                self.clone()
                    .change_directory(&p.to_string_lossy().to_string())
            })
            .unwrap_or(Ok(self))
    }

    fn last_visited_path(mut self) -> Result<Self> {
        self.history = self.history.visit_last();
        self.pwd = self
            .history
            .peek()
            .map(|p| p.to_owned())
            .unwrap_or(self.pwd);
        self.refresh()
    }

    fn next_visited_path(mut self) -> Result<Self> {
        self.history = self.history.visit_next();
        self.pwd = self
            .history
            .peek()
            .map(|p| p.to_owned())
            .unwrap_or(self.pwd);
        self.refresh()
    }

    fn buffer_input(mut self, input: &str) -> Result<Self> {
        if let Some(buf) = self.input_buffer.as_mut() {
            buf.push_str(input)
        } else {
            self.input_buffer = Some(input.to_owned());
        };
        self.refresh()
    }

    fn buffer_input_from_key(self, key: Option<Key>) -> Result<Self> {
        if let Some(c) = key.and_then(|k| k.to_char()) {
            self.buffer_input(&c.to_string())
        } else {
            Ok(self)
        }
    }

    fn set_input_buffer(mut self, string: String) -> Result<Self> {
        self.input_buffer = Some(string);
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn reset_input_buffer(mut self) -> Result<Self> {
        self.input_buffer = None;
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn focus_by_index(mut self, index: usize) -> Result<Self> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = index.min(dir.total.max(1) - 1);
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_by_index_from_input(self) -> Result<Self> {
        if let Some(index) = self.input_buffer().and_then(|i| i.parse::<usize>().ok()) {
            self.focus_by_index(index)
        } else {
            Ok(self)
        }
    }

    fn focus_by_file_name(mut self, name: &str) -> Result<Self> {
        if let Some(dir_buf) = self.directory_buffer_mut() {
            if let Some(focus) = dir_buf
                .clone()
                .nodes
                .iter()
                .enumerate()
                .find(|(_, n)| n.relative_path == name)
                .map(|(i, _)| i)
            {
                dir_buf.focus = focus;
                self.msg_out.push_back(MsgOut::Refresh);
            };
        };
        Ok(self)
    }

    fn focus_path(self, path: &str) -> Result<Self> {
        let pathbuf = PathBuf::from(path);
        if let Some(parent) = pathbuf.parent() {
            if let Some(filename) = pathbuf.file_name() {
                self.change_directory(&parent.to_string_lossy().to_string())?
                    .focus_by_file_name(&filename.to_string_lossy().to_string())
            } else {
                bail!("invalid path {}", path)
            }
        } else {
            self.change_directory("/")
        }
    }

    fn focus_path_from_input(self) -> Result<Self> {
        if let Some(p) = self.input_buffer() {
            self.focus_path(&p)
        } else {
            Ok(self)
        }
    }

    fn switch_mode(mut self, mode: &str) -> Result<Self> {
        if let Some(mode) = self.config.modes.get(mode) {
            self.input_buffer = None;
            self.mode = mode.to_owned();
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn call(mut self, command: Command) -> Result<Self> {
        self.msg_out.push_back(MsgOut::Call(command));
        Ok(self)
    }

    fn call_silently(mut self, command: Command) -> Result<Self> {
        self.msg_out.push_back(MsgOut::CallSilently(command));
        Ok(self)
    }

    fn bash_exec(self, script: String) -> Result<Self> {
        self.call(Command {
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

    fn add_directory(mut self, parent: String, dir: DirectoryBuffer) -> Result<Self> {
        self.directory_buffers.insert(parent, dir);
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn select(mut self) -> Result<Self> {
        if let Some(n) = self.focused_node().map(|n| n.to_owned()) {
            self.selection.push(n);
            self.msg_out.push_back(MsgOut::Refresh);
        }
        Ok(self)
    }

    fn un_select(mut self) -> Result<Self> {
        if let Some(n) = self.focused_node().map(|n| n.to_owned()) {
            self.selection = self
                .selection
                .clone()
                .into_iter()
                .filter(|s| s != &n)
                .collect();
            self.msg_out.push_back(MsgOut::Refresh);
        }
        Ok(self)
    }

    fn toggle_selection(mut self) -> Result<Self> {
        if let Some(n) = self.focused_node() {
            if self.selection().contains(n) {
                self = self.un_select()?;
            } else {
                self = self.select()?;
            }
        }
        Ok(self)
    }

    fn clear_selection(mut self) -> Result<Self> {
        self.selection.clear();
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn add_node_filter(mut self, filter: NodeFilterApplicable) -> Result<Self> {
        self.explorer_config.filters.insert(filter);
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn add_node_filter_from_input(mut self, filter: NodeFilterFromInput) -> Result<Self> {
        if let Some(input) = self.input_buffer() {
            self.explorer_config
                .filters
                .insert(NodeFilterApplicable::new(
                    filter.filter,
                    input,
                    filter.case_sensitive,
                ));
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn remove_node_filter(mut self, filter: NodeFilterApplicable) -> Result<Self> {
        self.explorer_config.filters.remove(&filter);
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn remove_node_filter_from_input(mut self, filter: NodeFilterFromInput) -> Result<Self> {
        if let Some(input) = self.input_buffer() {
            self.explorer_config
                .filters
                .remove(&NodeFilterApplicable::new(
                    filter.filter,
                    input,
                    filter.case_sensitive,
                ));
            self.msg_out.push_back(MsgOut::Refresh);
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

    fn reset_node_filters(mut self) -> Result<Self> {
        self.explorer_config.filters.clear();

        if !self.config.general.show_hidden.unwrap_or_default() {
            self.add_node_filter(NodeFilterApplicable::new(
                NodeFilter::RelativePathDoesNotStartWith,
                ".".into(),
                Default::default(),
            ))
        } else {
            self.msg_out.push_back(MsgOut::Refresh);
            Ok(self)
        }
    }

    fn log_info(mut self, message: String) -> Result<Self> {
        self.logs.push(Log::new(LogLevel::Info, message));
        Ok(self)
    }

    fn log_success(mut self, message: String) -> Result<Self> {
        self.logs.push(Log::new(LogLevel::Success, message));
        Ok(self)
    }

    fn log_error(mut self, message: String) -> Result<Self> {
        self.logs.push(Log::new(LogLevel::Error, message));
        Ok(self)
    }

    fn quit(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::Quit);
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
        self.directory_buffers.get_mut(&self.pwd)
    }

    /// Get a reference to the app's pwd.
    pub fn pwd(&self) -> &String {
        &self.pwd
    }

    /// Get a reference to the app's current directory buffer.
    pub fn directory_buffer(&self) -> Option<&DirectoryBuffer> {
        self.directory_buffers.get(&self.pwd)
    }

    /// Get a reference to the app's config.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get a reference to the app's selection.
    pub fn selection(&self) -> &Vec<Node> {
        &self.selection
    }

    pub fn pop_msg_out(&mut self) -> Option<MsgOut> {
        self.msg_out.pop_front()
    }

    /// Get a reference to the app's mode.
    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn mode_str(&self) -> String {
        format!("{}\n", &self.mode.name)
    }

    /// Get a reference to the app's directory buffers.
    pub fn directory_buffers(&self) -> &HashMap<String, DirectoryBuffer> {
        &self.directory_buffers
    }

    /// Get a reference to the app's input buffer.
    pub fn input_buffer(&self) -> Option<String> {
        self.input_buffer.clone()
    }

    /// Get a reference to the app's pipes.
    pub fn pipe(&self) -> &Pipe {
        &self.pipe
    }

    /// Get a reference to the app's pid.
    pub fn pid(&self) -> &u32 {
        &self.pid
    }

    /// Get a reference to the app's runtime path.
    pub fn session_path(&self) -> &str {
        &self.session_path
    }

    pub fn refresh_selection(mut self) -> Result<Self> {
        self.selection = self
            .selection
            .into_iter()
            .filter(|n| PathBuf::from(&n.absolute_path).exists())
            .collect();
        Ok(self)
    }

    pub fn result(&self) -> Vec<&Node> {
        if self.selection.is_empty() {
            self.focused_node().map(|n| vec![n]).unwrap_or_default()
        } else {
            self.selection.iter().collect()
        }
    }

    pub fn directory_nodes_str(&self) -> String {
        self.directory_buffer()
            .map(|d| {
                d.nodes
                    .iter()
                    .map(|n| format!("{}\n", n.absolute_path))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .unwrap_or_default()
    }

    pub fn selection_str(&self) -> String {
        self.selection
            .iter()
            .map(|n| format!("{}\n", n.absolute_path))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn result_str(&self) -> String {
        self.result()
            .into_iter()
            .map(|n| format!("{}\n", n.absolute_path))
            .collect::<Vec<String>>()
            .join("")
    }

    /// Get a reference to the app's explorer config.
    pub fn explorer_config(&self) -> &ExplorerConfig {
        &self.explorer_config
    }

    /// Get a reference to the app's logs.
    pub fn logs(&self) -> &Vec<Log> {
        &self.logs
    }

    pub fn global_help_menu_str(&self) -> String {
        let builtin = self.config().modes.builtin.clone();
        let custom = self.config().modes.custom.clone();

        [
            (builtin.default.name.clone(), builtin.default),
            (builtin.number.name.clone(), builtin.number),
            (builtin.go_to.name.clone(), builtin.go_to),
            (builtin.search.name.clone(), builtin.search),
            (builtin.selection_ops.name.clone(), builtin.selection_ops),
            (builtin.action.name.clone(), builtin.action),
            (builtin.create.name.clone(), builtin.create),
            (builtin.create_file.name.clone(), builtin.create_file),
            (
                builtin.create_directory.name.clone(),
                builtin.create_directory,
            ),
            (builtin.rename.name.clone(), builtin.rename),
            (builtin.delete.name.clone(), builtin.delete),
        ]
        .iter()
        .chain(custom.into_iter().collect::<Vec<(String, Mode)>>().iter())
        .map(|(name, mode)| {
            let help = mode
                .help_menu()
                .iter()
                .map(|l| match l {
                    HelpMenuLine::Paragraph(p) => format!("\t{}\n", p),
                    HelpMenuLine::KeyMap(k, h) => {
                        let remaps = self
                            .mode()
                            .key_bindings
                            .remaps
                            .iter()
                            .filter(|(_, t)| t == &k)
                            .map(|(f, _)| f.clone())
                            .collect::<Vec<String>>()
                            .join(", ");
                        format!(" {:15} | {:25} | {}\n", k, remaps, h)
                    }
                })
                .collect::<Vec<String>>()
                .join("");

            format!(
                "### {}\n\n key             | remaps                    | action\n --------------- | ------------------------- |------\n{}\n",
                name, help
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
    }

    /// Get a reference to the app's version.
    pub fn version(&self) -> &String {
        &self.version
    }
}
