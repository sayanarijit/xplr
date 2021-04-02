use crate::config::Config;
use crate::config::Mode;
use crate::input::Key;
use anyhow::{bail, Result};
use mime_guess;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub const VERSION: &str = "v0.2.14"; // Update Cargo.toml

pub const TEMPLATE_TABLE_ROW: &str = "TEMPLATE_TABLE_ROW";

pub const UNSUPPORTED_STR: &str = "???";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipe {
    pub msg_in: String,
    pub focus_out: String,
    pub selection_out: String,
    pub mode_out: String,
}

impl Pipe {
    fn from_session_path(path: &String) -> Self {
        let pipesdir = PathBuf::from(path).join("pipe");

        fs::create_dir_all(&pipesdir).unwrap();

        let msg_in = pipesdir.join("msg_in").to_string_lossy().to_string();

        let focus_out = pipesdir.join("focus_out").to_string_lossy().to_string();

        let selection_out = pipesdir.join("selection_out").to_string_lossy().to_string();

        let mode_out = pipesdir.join("mode_out").to_string_lossy().to_string();

        fs::write(&msg_in, "").unwrap();
        fs::write(&focus_out, "").unwrap();
        fs::write(&selection_out, "").unwrap();
        fs::write(&mode_out, "").unwrap();

        Self {
            msg_in,
            focus_out,
            selection_out,
            mode_out,
        }
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
            .canonicalize()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let path = PathBuf::from(&absolute_path);

        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();

        let maybe_metadata = path.metadata().ok();

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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
    fn apply(&self, node: &Node, input: &String, case_sensitive: bool) -> bool {
        match self {
            Self::RelativePathIs => {
                if case_sensitive {
                    &node.relative_path == input
                } else {
                    node.relative_path.to_lowercase() == input.to_lowercase()
                }
            }

            Self::RelativePathIsNot => {
                if case_sensitive {
                    &node.relative_path != input
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
                    &node.absolute_path == input
                } else {
                    node.absolute_path.to_lowercase() == input.to_lowercase()
                }
            }

            Self::AbsolutePathIsNot => {
                if case_sensitive {
                    &node.absolute_path != input
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
pub struct NodeFilterFromInputString {
    filter: NodeFilter,
    #[serde(default)]
    case_sensitive: bool,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExplorerConfig {
    filters: Vec<NodeFilterApplicable>,
}

impl ExplorerConfig {
    pub fn apply(&self, node: &Node) -> bool {
        self.filters.iter().all(|f| f.apply(node))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExternalMsg {
    Explore,
    Refresh,
    ClearScreen,
    FocusNext,
    FocusNextByRelativeIndex(usize),
    FocusNextByRelativeIndexFromInput,
    FocusPrevious,
    FocusPreviousByRelativeIndex(usize),
    FocusPreviousByRelativeIndexFromInput,
    FocusFirst,
    FocusLast,
    FocusPath(String),
    FocusByIndex(usize),
    FocusByIndexFromInput,
    FocusByFileName(String),
    ChangeDirectory(String),
    Enter,
    Back,
    BufferString(String),
    BufferStringFromKey,
    ResetInputBuffer,
    SwitchMode(String),
    Call(Command),
    Select,
    UnSelect,
    ToggleSelection,
    ClearSelection,
    AddNodeFilter(NodeFilterApplicable),
    RemoveNodeFilter(NodeFilterApplicable),
    ToggleNodeFilter(NodeFilterApplicable),
    AddNodeFilterFromInputString(NodeFilterFromInputString),
    ResetNodeFilters,
    PrintResultAndQuit,
    PrintAppStateAndQuit,
    Debug(String),
    Terminate,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MsgIn {
    Internal(InternalMsg),
    External(ExternalMsg),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
    PrintResultAndQuit,
    PrintAppStateAndQuit,
    Debug(String),
    Call(Command),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Task {
    priority: usize,
    msg: MsgIn,
    key: Option<Key>,
}

impl Task {
    pub fn new(priority: usize, msg: MsgIn, key: Option<Key>) -> Self {
        Self { priority, msg, key }
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.priority.cmp(&self.priority)
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    config: Config,
    pwd: String,
    directory_buffers: HashMap<String, DirectoryBuffer>,
    tasks: BinaryHeap<Task>,
    selection: Vec<Node>,
    msg_out: VecDeque<MsgOut>,
    mode: Mode,
    input_buffer: Option<String>,
    pid: u32,
    session_path: String,
    pipe: Pipe,
    explorer_config: ExplorerConfig,
}

impl App {
    pub fn create() -> Result<Self> {
        let mut pwd = PathBuf::from(env::args().skip(1).next().unwrap_or(".".into()))
            .canonicalize()
            .unwrap_or_default();

        if pwd.is_file() {
            pwd = pwd.parent().map(|p| p.into()).unwrap_or_default();
        }

        let pwd = pwd.to_string_lossy().to_string();

        let config_dir = dirs::config_dir()
            .unwrap_or(PathBuf::from("."))
            .join("xplr");

        let config_file = config_dir.join("config.yml");

        let config: Config = if config_file.exists() {
            serde_yaml::from_reader(io::BufReader::new(&fs::File::open(&config_file)?))?
        } else {
            Config::default()
        };

        if config.version != VERSION {
            bail!(
                "incompatible config version {} in {}",
                config.version,
                config_file.to_string_lossy().to_string()
            )
        } else {
            let mode = config
                .modes
                .get(&"default".to_string())
                .map(|k| k.to_owned())
                .unwrap_or_default();

            let pid = std::process::id();
            let session_path = dirs::runtime_dir()
                .unwrap_or("/tmp".into())
                .join("xplr")
                .join("session")
                .join(&pid.to_string())
                .to_string_lossy()
                .to_string();

            let mut explorer_config = ExplorerConfig::default();
            if !config.general.show_hidden {
                explorer_config.filters.push(NodeFilterApplicable::new(
                    NodeFilter::RelativePathDoesNotStartWith,
                    ".".into(),
                    Default::default(),
                ));
            }

            Ok(Self {
                config,
                pwd,
                directory_buffers: Default::default(),
                tasks: Default::default(),
                selection: Default::default(),
                msg_out: Default::default(),
                mode,
                input_buffer: Default::default(),
                pid,
                session_path: session_path.clone(),
                pipe: Pipe::from_session_path(&session_path),
                explorer_config,
            })
        }
    }

    pub fn focused_node(&self) -> Option<&Node> {
        self.directory_buffer().and_then(|d| d.focused_node())
    }

    pub fn enqueue(mut self, task: Task) -> Self {
        self.tasks.push(task);
        self
    }

    pub fn possibly_mutate(mut self) -> Result<Self> {
        if let Some(task) = self.tasks.pop() {
            match task.msg {
                MsgIn::Internal(msg) => self.handle_internal(msg),
                MsgIn::External(msg) => self.handle_external(msg, task.key),
            }
        } else {
            Ok(self)
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
            ExternalMsg::FocusByIndex(i) => self.focus_by_index(i),
            ExternalMsg::FocusByIndexFromInput => self.focus_by_index_from_input(),
            ExternalMsg::FocusByFileName(n) => self.focus_by_file_name(&n),
            ExternalMsg::ChangeDirectory(dir) => self.change_directory(&dir),
            ExternalMsg::Enter => self.enter(),
            ExternalMsg::Back => self.back(),
            ExternalMsg::BufferString(input) => self.buffer_string(&input),
            ExternalMsg::BufferStringFromKey => self.buffer_string_from_key(key),
            ExternalMsg::ResetInputBuffer => self.reset_input_buffer(),
            ExternalMsg::SwitchMode(mode) => self.switch_mode(&mode),
            ExternalMsg::Call(cmd) => self.call(cmd),
            ExternalMsg::Select => self.select(),
            ExternalMsg::UnSelect => self.un_select(),
            ExternalMsg::ToggleSelection => self.toggle_selection(),
            ExternalMsg::ClearSelection => self.clear_selection(),
            ExternalMsg::AddNodeFilter(f) => self.add_node_filter(f),
            ExternalMsg::AddNodeFilterFromInputString(f) => {
                self.add_node_filter_from_input_string(f)
            }
            ExternalMsg::RemoveNodeFilter(f) => self.remove_node_filter(f),
            ExternalMsg::ToggleNodeFilter(f) => self.toggle_node_filter(f),
            ExternalMsg::ResetNodeFilters => self.reset_node_filters(),
            ExternalMsg::PrintResultAndQuit => self.print_result_and_quit(),
            ExternalMsg::PrintAppStateAndQuit => self.print_app_state_and_quit(),
            ExternalMsg::Debug(path) => self.debug(&path),
            ExternalMsg::Terminate => bail!("terminated"),
        }
    }

    fn handle_key(mut self, key: Key) -> Result<Self> {
        let kb = self.mode.key_bindings.clone();
        let default = kb.default.clone();
        let msgs = kb
            .on_key
            .get(&key.to_string())
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
            self = self.enqueue(Task::new(0, MsgIn::External(msg), Some(key)));
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

    fn change_directory(mut self, dir: &String) -> Result<Self> {
        if PathBuf::from(dir).is_dir() {
            self.pwd = dir.to_owned();
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

    fn buffer_string(mut self, input: &String) -> Result<Self> {
        if let Some(buf) = self.input_buffer.as_mut() {
            buf.extend(input.chars());
        } else {
            self.input_buffer = Some(input.to_owned());
        };
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn buffer_string_from_key(self, key: Option<Key>) -> Result<Self> {
        if let Some(c) = key.and_then(|k| k.to_char()) {
            self.buffer_string(&c.to_string())
        } else {
            Ok(self)
        }
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

    fn focus_by_file_name(mut self, name: &String) -> Result<Self> {
        if let Some(dir_buf) = self.directory_buffer_mut() {
            if let Some(focus) = dir_buf
                .clone()
                .nodes
                .iter()
                .enumerate()
                .find(|(_, n)| &n.relative_path == name)
                .map(|(i, _)| i)
            {
                dir_buf.focus = focus;
                self.msg_out.push_back(MsgOut::Refresh);
            };
        };
        Ok(self)
    }

    fn focus_path(self, path: &String) -> Result<Self> {
        let pathbuf = PathBuf::from(path);
        if let Some(parent) = pathbuf.parent() {
            if let Some(filename) = pathbuf.file_name() {
                self.change_directory(&parent.to_string_lossy().to_string())?
                    .focus_by_file_name(&filename.to_string_lossy().to_string())
            } else {
                Ok(self)
            }
        } else {
            Ok(self)
        }
    }

    fn switch_mode(mut self, mode: &String) -> Result<Self> {
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

    fn add_directory(mut self, parent: String, dir: DirectoryBuffer) -> Result<Self> {
        self.directory_buffers.insert(parent, dir);
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn select(mut self) -> Result<Self> {
        if let Some(n) = self.focused_node().map(|n| n.to_owned()) {
            self.selection.push(n.clone());
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
        self.explorer_config.filters.push(filter);
        self.msg_out.push_back(MsgOut::Explore);
        Ok(self)
    }

    fn add_node_filter_from_input_string(
        mut self,
        filter: NodeFilterFromInputString,
    ) -> Result<Self> {
        if let Some(input) = self.input_buffer() {
            self.explorer_config.filters.push(NodeFilterApplicable::new(
                filter.filter,
                input,
                filter.case_sensitive,
            ));
            self.msg_out.push_back(MsgOut::Explore);
        };
        Ok(self)
    }

    fn remove_node_filter(mut self, filter: NodeFilterApplicable) -> Result<Self> {
        self.explorer_config.filters = self
            .explorer_config
            .filters
            .into_iter()
            .filter(|f| f != &filter)
            .collect();
        self.msg_out.push_back(MsgOut::Explore);
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

        if !self.config.general.show_hidden {
            self.explorer_config.filters.push(NodeFilterApplicable::new(
                NodeFilter::RelativePathDoesNotStartWith,
                ".".into(),
                Default::default(),
            ));
        };
        self.msg_out.push_back(MsgOut::Explore);

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

    fn debug(mut self, path: &String) -> Result<Self> {
        self.msg_out.push_back(MsgOut::Debug(path.to_owned()));
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
    pub fn session_path(&self) -> &String {
        &self.session_path
    }

    pub fn refresh_selection(mut self) -> Result<Self> {
        self.selection = self
            .selection
            .clone()
            .into_iter()
            .filter(|n| PathBuf::from(&n.absolute_path).exists())
            .collect();
        Ok(self)
    }

    pub fn result(&self) -> Vec<&Node> {
        if self.selection.is_empty() {
            self.focused_node().map(|n| vec![n]).unwrap_or_default()
        } else {
            self.selection.iter().map(|n| n).collect()
        }
    }

    pub fn result_str(&self) -> String {
        self.result()
            .into_iter()
            .map(|n| n.absolute_path.clone())
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Get a reference to the app's explorer config.
    pub fn explorer_config(&self) -> &ExplorerConfig {
        &self.explorer_config
    }
}
