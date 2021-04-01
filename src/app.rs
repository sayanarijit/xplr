use crate::config::Config;
use crate::config::Mode;
use crate::error::Error;
use crate::input::Key;
use mime_guess;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;

pub const VERSION: &str = "v0.2.1"; // Update Cargo.toml

pub const TEMPLATE_TABLE_ROW: &str = "TEMPLATE_TABLE_ROW";

pub const UNSUPPORTED_STR: &str = "???";

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
pub enum ExternalMsg {
    Refresh,
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
    ToggleSelection,
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
    Refresh,
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
    selected: Vec<Node>,
    msg_out: VecDeque<MsgOut>,
    mode: Mode,
    input_buffer: Option<String>,
}

impl App {
    pub fn new(config: Config, pwd: String) -> Self {
        let mode = config
            .modes
            .get(&"default".to_string())
            .map(|k| k.to_owned())
            .unwrap_or_default();

        Self {
            config,
            pwd,
            directory_buffers: Default::default(),
            tasks: Default::default(),
            selected: Default::default(),
            msg_out: Default::default(),
            mode,
            input_buffer: Default::default(),
        }
    }

    pub fn focused_node(&self) -> Option<&Node> {
        self.directory_buffer().and_then(|d| d.focused_node())
    }

    pub fn enqueue(mut self, task: Task) -> Self {
        self.tasks.push(task);
        self
    }

    pub fn possibly_mutate(mut self) -> Result<Self, Error> {
        if let Some(task) = self.tasks.pop() {
            match task.msg {
                MsgIn::Internal(msg) => self.handle_internal(msg),
                MsgIn::External(msg) => self.handle_external(msg, task.key),
            }
        } else {
            Ok(self)
        }
    }

    fn handle_internal(self, msg: InternalMsg) -> Result<Self, Error> {
        match msg {
            InternalMsg::AddDirectory(parent, dir) => self.add_directory(parent, dir),
            InternalMsg::HandleKey(key) => self.handle_key(key),
        }
    }

    fn handle_external(self, msg: ExternalMsg, key: Option<Key>) -> Result<Self, Error> {
        match msg {
            ExternalMsg::Refresh => self.refresh(),
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
            ExternalMsg::ToggleSelection => self.toggle_selection(),
            ExternalMsg::PrintResultAndQuit => self.print_result_and_quit(),
            ExternalMsg::PrintAppStateAndQuit => self.print_app_state_and_quit(),
            ExternalMsg::Debug(path) => self.debug(&path),
            ExternalMsg::Terminate => Err(Error::Terminated),
        }
    }

    fn handle_key(mut self, key: Key) -> Result<Self, Error> {
        let kb = self.mode.key_bindings.clone();
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
                    kb.default.map(|a| a.messages)
                }
            });

        if let Some(msgs) = msgs.to_owned() {
            for msg in msgs {
                self = self.enqueue(Task::new(0, MsgIn::External(msg), Some(key)));
            }
        };

        Ok(self)
    }

    fn refresh(mut self) -> Result<Self, Error> {
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn focus_first(mut self) -> Result<Self, Error> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = 0;
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_last(mut self) -> Result<Self, Error> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = dir.total.max(1) - 1;
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_previous(mut self) -> Result<Self, Error> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = dir.focus.max(1) - 1;
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_previous_by_relative_index(mut self, index: usize) -> Result<Self, Error> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = dir.focus.max(index) - index;
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_previous_by_relative_index_from_input(self) -> Result<Self, Error> {
        if let Some(index) = self.input_buffer().and_then(|i| i.parse::<usize>().ok()) {
            self.focus_previous_by_relative_index(index)
        } else {
            Ok(self)
        }
    }

    fn focus_next(mut self) -> Result<Self, Error> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = (dir.focus + 1).min(dir.total.max(1) - 1);
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_next_by_relative_index(mut self, index: usize) -> Result<Self, Error> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = (dir.focus + index).min(dir.total.max(1) - 1);
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_next_by_relative_index_from_input(self) -> Result<Self, Error> {
        if let Some(index) = self.input_buffer().and_then(|i| i.parse::<usize>().ok()) {
            self.focus_next_by_relative_index(index)
        } else {
            Ok(self)
        }
    }

    fn change_directory(mut self, dir: &String) -> Result<Self, Error> {
        if PathBuf::from(dir).is_dir() {
            self.pwd = dir.to_owned();
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn enter(self) -> Result<Self, Error> {
        self.focused_node()
            .map(|n| n.absolute_path.clone())
            .map(|p| self.clone().change_directory(&p))
            .unwrap_or(Ok(self))
    }

    fn back(self) -> Result<Self, Error> {
        PathBuf::from(self.pwd())
            .parent()
            .map(|p| {
                self.clone()
                    .change_directory(&p.to_string_lossy().to_string())
            })
            .unwrap_or(Ok(self))
    }

    fn buffer_string(mut self, input: &String) -> Result<Self, Error> {
        if let Some(buf) = self.input_buffer.as_mut() {
            buf.extend(input.chars());
        } else {
            self.input_buffer = Some(input.to_owned());
        };
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn buffer_string_from_key(self, key: Option<Key>) -> Result<Self, Error> {
        if let Some(c) = key.and_then(|k| k.to_char()) {
            self.buffer_string(&c.to_string())
        } else {
            Ok(self)
        }
    }

    fn reset_input_buffer(mut self) -> Result<Self, Error> {
        self.input_buffer = None;
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn focus_by_index(mut self, index: usize) -> Result<Self, Error> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = index.min(dir.total.max(1) - 1);
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn focus_by_index_from_input(self) -> Result<Self, Error> {
        if let Some(index) = self.input_buffer().and_then(|i| i.parse::<usize>().ok()) {
            self.focus_by_index(index)
        } else {
            Ok(self)
        }
    }

    fn focus_by_file_name(mut self, name: &String) -> Result<Self, Error> {
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

    fn focus_path(self, path: &String) -> Result<Self, Error> {
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

    fn switch_mode(mut self, mode: &String) -> Result<Self, Error> {
        if let Some(mode) = self.config.modes.get(mode) {
            self.mode = mode.to_owned();
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn call(mut self, command: Command) -> Result<Self, Error> {
        self.msg_out.push_back(MsgOut::Call(command));
        Ok(self)
    }

    fn add_directory(mut self, parent: String, dir: DirectoryBuffer) -> Result<Self, Error> {
        // TODO: Optimize
        self.directory_buffers.insert(parent, dir);
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn toggle_selection(mut self) -> Result<Self, Error> {
        self.clone()
            .focused_node()
            .map(|n| {
                if self.selected().contains(n) {
                    self.selected = self
                        .clone()
                        .selected
                        .into_iter()
                        .filter(|s| s != n)
                        .collect();
                    Ok(self.clone())
                } else {
                    self.selected.push(n.to_owned());
                    Ok(self.clone())
                }
            })
            .unwrap_or(Ok(self))
    }

    fn print_result_and_quit(mut self) -> Result<Self, Error> {
        self.msg_out.push_back(MsgOut::PrintResultAndQuit);
        Ok(self)
    }

    fn print_app_state_and_quit(mut self) -> Result<Self, Error> {
        self.msg_out.push_back(MsgOut::PrintAppStateAndQuit);
        Ok(self)
    }

    fn debug(mut self, path: &String) -> Result<Self, Error> {
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

    /// Get a reference to the app's selected.
    pub fn selected(&self) -> &Vec<Node> {
        &self.selected
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
    pub fn input_buffer(&self) -> Option<&String> {
        self.input_buffer.as_ref()
    }
}
