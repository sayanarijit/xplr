use crate::config::{
    Action, CommandConfig, Config, ExploreModeAction, GlobalAction, Mode, SelectModeAction,
};
use crate::error::Error;
use crate::input::Key;
use dirs;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

pub const VERSION: &str = "v0.1.7"; // Update Cargo.toml
pub const UNSUPPORTED_STR: &str = "???";
pub const TOTAL_ROWS: usize = 50;

pub const TEMPLATE_TABLE_ROW: &str = "TEMPLATE_TABLE_ROW";

fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryBuffer {
    pub pwd: PathBuf,
    pub focus: Option<usize>,
    pub items: Vec<(PathBuf, DirectoryItemMetadata)>,
    pub total: usize,
}

impl DirectoryBuffer {
    pub fn relative_focus(focus: usize) -> usize {
        focus.min(TOTAL_ROWS)
    }

    pub fn explore(
        path: &PathBuf,
        show_hidden: bool,
    ) -> Result<(usize, impl Iterator<Item = (PathBuf, String)>), Error> {
        let hide_hidden = !show_hidden;

        let total = fs::read_dir(&path)?
            .filter_map(|d| d.ok().map(|e| e.path()))
            .filter_map(|p| p.canonicalize().ok())
            .filter_map(|abs_path| {
                abs_path
                    .file_name()
                    .map(|rel_path| rel_path.to_str().unwrap_or(UNSUPPORTED_STR).to_string())
            })
            .filter(|rel_path| !(hide_hidden && rel_path.starts_with('.')))
            .count();

        let items = fs::read_dir(&path)?
            .filter_map(|d| d.ok().map(|e| e.path()))
            .filter_map(|p| p.canonicalize().ok())
            .filter_map(|abs_path| {
                abs_path.file_name().map(|rel_path| {
                    (
                        abs_path.to_path_buf(),
                        rel_path.to_str().unwrap_or(UNSUPPORTED_STR).to_string(),
                    )
                })
            })
            .filter(move |(_, rel_path)| !(hide_hidden && rel_path.starts_with('.')));
        Ok((total, items))
    }

    pub fn load(
        config: &Config,
        focus: Option<usize>,
        path: &PathBuf,
        show_hidden: bool,
        selected_paths: &HashSet<PathBuf>,
    ) -> Result<DirectoryBuffer, Error> {
        let offset = focus
            .map(|f| (f.max(TOTAL_ROWS) - TOTAL_ROWS, f.max(TOTAL_ROWS)))
            .unwrap_or((0, TOTAL_ROWS));

        let (total, items) = DirectoryBuffer::explore(&path, show_hidden)?;
        let visible: Vec<(PathBuf, DirectoryItemMetadata)> = items
            .enumerate()
            .skip_while(|(i, _)| *i < offset.0)
            .take_while(|(i, _)| *i <= offset.1)
            .enumerate()
            .map(|(rel_idx, (net_idx, (abs, rel)))| {
                let ext = abs
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                (net_idx, rel_idx, abs, rel, ext)
            })
            .map(|(net_idx, rel_idx, abs, rel, ext)| {
                let absolute_path: String =
                    abs.as_os_str().to_str().unwrap_or(UNSUPPORTED_STR).into();
                let relative_path = rel.to_string();
                let extension = ext.to_string();
                let is_dir = abs.is_dir();
                let is_file = abs.is_file();

                let maybe_meta = abs.metadata().ok();

                let is_symlink = maybe_meta
                    .clone()
                    .map(|m| m.file_type().is_symlink())
                    .unwrap_or(false);

                let is_readonly = maybe_meta
                    .clone()
                    .map(|m| m.permissions().readonly())
                    .unwrap_or(false);

                let (focus_idx, is_focused) =
                    focus.map(|f| (f, net_idx == f)).unwrap_or((0, false));
                let is_selected = selected_paths.contains(&abs);

                let ui = if is_focused {
                    &config.general.focused_ui
                } else if is_selected {
                    &config.general.selected_ui
                } else {
                    &config.general.normal_ui
                };

                let is_first_item = net_idx == 0;
                let is_last_item = net_idx == total.max(1) - 1;

                let tree = config
                    .general
                    .table
                    .tree
                    .clone()
                    .map(|t| {
                        if is_last_item {
                            t.2.format.clone()
                        } else if is_first_item {
                            t.0.format.clone()
                        } else {
                            t.1.format.clone()
                        }
                    })
                    .unwrap_or_default();

                let filetype = config
                    .filetypes
                    .special
                    .get(&relative_path)
                    .or_else(|| config.filetypes.extension.get(&extension))
                    .unwrap_or_else(|| {
                        if is_symlink {
                            &config.filetypes.symlink
                        } else if is_dir {
                            &config.filetypes.directory
                        } else {
                            &config.filetypes.file
                        }
                    });

                let focus_relative_index = if focus_idx <= net_idx {
                    format!(" {}", net_idx - focus_idx)
                } else {
                    format!("-{}", focus_idx - net_idx)
                };

                let m = DirectoryItemMetadata {
                    absolute_path,
                    relative_path,
                    extension,
                    icon: filetype.icon.clone(),
                    prefix: ui.prefix.clone(),
                    suffix: ui.suffix.clone(),
                    tree: tree.into(),
                    is_symlink,
                    is_first_item,
                    is_last_item,
                    is_dir,
                    is_file,
                    is_readonly,
                    is_selected,
                    is_focused,
                    index: net_idx + 1,
                    focus_relative_index,
                    buffer_relative_index: rel_idx + 1,
                    total_items: total,
                };
                (abs.to_owned(), m)
            })
            .collect();

        let focus = focus.map(|f| {
            if Self::relative_focus(f) >= visible.len() {
                visible.len().max(1) - 1
            } else {
                f
            }
        });

        Ok(Self {
            pwd: path.into(),
            total,
            items: visible,
            focus,
        })
    }

    pub fn focused_item(&self) -> Option<(PathBuf, DirectoryItemMetadata)> {
        self.focus.and_then(|f| {
            self.items
                .get(Self::relative_focus(f))
                .map(|f| f.to_owned())
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryItemMetadata {
    pub absolute_path: String,
    pub relative_path: String,
    pub extension: String,
    pub icon: String,
    pub prefix: String,
    pub suffix: String,
    pub tree: String,
    pub is_first_item: bool,
    pub is_last_item: bool,
    pub is_symlink: bool,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_readonly: bool,
    pub is_selected: bool,
    pub is_focused: bool,
    pub index: usize,
    pub focus_relative_index: String,
    pub buffer_relative_index: usize,
    pub total_items: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub version: String,
    pub config: Config,
    pub directory_buffer: DirectoryBuffer,
    pub saved_buffers: HashMap<PathBuf, Option<usize>>,
    pub selected_paths: HashSet<PathBuf>,
    pub mode: Mode,
    pub show_hidden: bool,
    pub call: Option<CommandConfig>,
    pub result: Option<String>,
}

impl App {
    pub fn new(
        config: &Config,
        pwd: &PathBuf,
        saved_buffers: &HashMap<PathBuf, Option<usize>>,
        selected_paths: &HashSet<PathBuf>,
        mode: Mode,
        show_hidden: bool,
        focus: Option<usize>,
    ) -> Result<Self, Error> {
        let directory_buffer =
            DirectoryBuffer::load(config, focus.or(Some(0)), &pwd, show_hidden, selected_paths)?;

        let mut saved_buffers = saved_buffers.clone();
        saved_buffers.insert(
            directory_buffer.pwd.clone().into(),
            directory_buffer.focus.clone(),
        );

        Ok(Self {
            version: VERSION.into(),
            config: config.to_owned(),
            directory_buffer,
            saved_buffers,
            selected_paths: selected_paths.to_owned(),
            mode,
            show_hidden,
            result: None,
            call: None,
        })
    }

    pub fn exit_submode(self) -> Result<Self, Error> {
        let mut app = self;
        let mode = match app.mode {
            Mode::ExploreSubmode(_) => Mode::Explore,
            Mode::SelectSubmode(_) => Mode::Select,
            m => m,
        };
        app.mode = mode;
        Ok(app)
    }

    pub fn toggle_hidden(self) -> Result<Self, Error> {
        Self::new(
            &self.config,
            &self.directory_buffer.pwd,
            &self.saved_buffers,
            &self.selected_paths,
            self.mode,
            !self.show_hidden,
            self.directory_buffer.focus,
        )
    }

    pub fn focus_first_item(self) -> Result<Self, Error> {
        let focus = if self.directory_buffer.total == 0 {
            None
        } else {
            Some(0)
        };

        Self::new(
            &self.config,
            &self.directory_buffer.pwd,
            &self.saved_buffers,
            &self.selected_paths,
            self.mode,
            self.show_hidden,
            focus,
        )
    }

    pub fn focus_last_item(self) -> Result<Self, Error> {
        let focus = if self.directory_buffer.total == 0 {
            None
        } else {
            Some(self.directory_buffer.total - 1)
        };

        Self::new(
            &self.config,
            &self.directory_buffer.pwd,
            &self.saved_buffers,
            &self.selected_paths,
            self.mode,
            self.show_hidden,
            focus,
        )
    }

    pub fn change_directory(self, dir: &String) -> Result<Self, Error> {
        self.focus_path(&PathBuf::from(dir))?.enter()
    }

    pub fn call(mut self, cmd: &CommandConfig) -> Result<Self, Error> {
        self.call = Some(cmd.clone());
        Ok(self)
    }

    pub fn focus_next_item(self) -> Result<Self, Error> {
        let len = self.directory_buffer.total;
        let focus = self
            .directory_buffer
            .focus
            .map(|f| (len - 1).min(f + 1))
            .or(Some(0));

        Self::new(
            &self.config,
            &self.directory_buffer.pwd,
            &self.saved_buffers,
            &self.selected_paths,
            self.mode,
            self.show_hidden,
            focus,
        )
    }

    pub fn focus_previous_item(self) -> Result<Self, Error> {
        let len = self.directory_buffer.total;
        let focus = if len == 0 {
            None
        } else {
            self.directory_buffer
                .focus
                .map(|f| Some(1.max(f) - 1))
                .unwrap_or(Some(len - 1))
        };

        Self::new(
            &self.config,
            &self.directory_buffer.pwd,
            &self.saved_buffers,
            &self.selected_paths,
            self.mode,
            self.show_hidden,
            focus,
        )
    }

    pub fn focus_path(self, path: &PathBuf) -> Result<Self, Error> {
        expand_tilde(path)
            .unwrap_or(path.into())
            .parent()
            .map(|pwd| {
                let (_, items) = DirectoryBuffer::explore(&pwd.into(), self.show_hidden)?;
                let focus = items
                    .enumerate()
                    .find_map(|(i, (p, _))| {
                        if p.as_path() == path.as_path() {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .or(Some(0));

                Self::new(
                    &self.config,
                    &pwd.into(),
                    &self.saved_buffers,
                    &self.selected_paths,
                    self.mode.clone(),
                    self.show_hidden,
                    focus,
                )
            })
            .unwrap_or_else(|| Ok(self.to_owned()))
    }

    pub fn focus_by_index(self, idx: &usize) -> Result<Self, Error> {
        Self::new(
            &self.config,
            &self.directory_buffer.pwd,
            &self.saved_buffers,
            &self.selected_paths,
            self.mode.clone(),
            self.show_hidden,
            Some(idx.clone()),
        )
    }

    pub fn focus_by_buffer_relative_index(self, idx: &usize) -> Result<Self, Error> {
        Self::new(
            &self.config,
            &self.directory_buffer.pwd,
            &self.saved_buffers,
            &self.selected_paths,
            self.mode.clone(),
            self.show_hidden,
            Some(DirectoryBuffer::relative_focus(idx.clone())),
        )
    }

    pub fn focus_by_focus_relative_index(self, idx: &isize) -> Result<Self, Error> {
        Self::new(
            &self.config,
            &self.directory_buffer.pwd,
            &self.saved_buffers,
            &self.selected_paths,
            self.mode.clone(),
            self.show_hidden,
            self.directory_buffer
                .focus
                .map(|f| ((f as isize) + idx).min(0) as usize), // TODO: make it safer
        )
    }

    pub fn enter(self) -> Result<Self, Error> {
        let pwd = self
            .directory_buffer
            .focused_item()
            .map(|(p, _)| p)
            .map(|p| {
                if p.is_dir() {
                    p
                } else {
                    self.directory_buffer.pwd.clone()
                }
            })
            .unwrap_or_else(|| self.directory_buffer.pwd.clone());

        let focus = self.saved_buffers.get(&pwd).unwrap_or(&None);

        Self::new(
            &self.config,
            &pwd,
            &self.saved_buffers,
            &self.selected_paths,
            self.mode,
            self.show_hidden,
            focus.clone(),
        )
    }

    pub fn back(self) -> Result<Self, Error> {
        let app = self.clone();
        self.focus_path(&app.directory_buffer.pwd)
    }

    pub fn select(self) -> Result<Self, Error> {
        let selected_paths = self
            .directory_buffer
            .focused_item()
            .map(|(p, _)| {
                let mut selected_paths = self.selected_paths.clone();
                selected_paths.insert(p);
                selected_paths
            })
            .unwrap_or_else(|| self.selected_paths.clone());

        let mut app = self;
        app.selected_paths = selected_paths;
        app.mode = Mode::Select;
        Ok(app)
    }

    pub fn toggle_selection(self) -> Result<Self, Error> {
        let selected_paths = self
            .directory_buffer
            .focused_item()
            .map(|(p, _)| {
                let mut selected_paths = self.selected_paths.clone();
                if selected_paths.contains(&p) {
                    selected_paths.remove(&p);
                } else {
                    selected_paths.insert(p);
                }
                selected_paths
            })
            .unwrap_or_else(|| self.selected_paths.clone());

        let mode = if selected_paths.len() == 0 {
            Mode::Explore
        } else {
            Mode::Select
        };

        let mut app = self;
        app.selected_paths = selected_paths;
        app.mode = mode;
        Ok(app)
    }

    pub fn enter_submode(self, submode: &String) -> Result<Self, Error> {
        let mut app = self;
        app.mode = Mode::ExploreSubmode(submode.clone());
        Ok(app)
    }

    pub fn print_focused(self) -> Result<Self, Error> {
        let mut app = self;
        app.result = app
            .directory_buffer
            .focused_item()
            .and_then(|(p, _)| p.to_str().map(|s| s.to_string()));
        Ok(app)
    }

    pub fn print_pwd(self) -> Result<Self, Error> {
        let mut app = self;
        app.result = app.directory_buffer.pwd.to_str().map(|s| s.to_string());
        Ok(app)
    }

    pub fn print_selected(self) -> Result<Self, Error> {
        let mut app = self;
        app.result = Some(
            app.selected_paths
                .clone()
                .iter()
                .filter_map(|p| p.to_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        );
        Ok(app)
    }

    pub fn print_app_state(self) -> Result<Self, Error> {
        let state = serde_yaml::to_string(&self)?;
        let mut app = self;
        app.result = Some(state);
        Ok(app)
    }

    pub fn quit(mut self) -> Result<Self, Error> {
        self.result = Some("".into());
        Ok(self)
    }

    pub fn terminate(self) -> Result<Self, Error> {
        Err(Error::Terminated)
    }

    pub fn actions_from_key(&self, key: Key) -> Option<Vec<Action>> {
        self.config
            .key_bindings
            .global
            .get(&key)
            .map(|m| {
                m.actions
                    .iter()
                    .map(|a| Action::Global(a.clone()))
                    .collect()
            })
            .or_else(|| match &self.mode {
                Mode::Explore => self.config.key_bindings.explore_mode.get(&key).map(|m| {
                    m.actions
                        .iter()
                        .map(|a| Action::ExploreMode(a.clone()))
                        .collect()
                }),

                Mode::Select => self.config.key_bindings.select_mode.get(&key).map(|m| {
                    m.actions
                        .iter()
                        .map(|a| Action::SelectMode(a.clone()))
                        .collect()
                }),

                Mode::ExploreSubmode(sub) => self
                    .config
                    .key_bindings
                    .explore_submodes
                    .get(sub)
                    .and_then(|kb| {
                        kb.get(&key).map(|m| {
                            m.actions
                                .iter()
                                .map(|a| Action::ExploreMode(a.clone()))
                                .collect()
                        })
                    }),

                Mode::SelectSubmode(sub) => self
                    .config
                    .key_bindings
                    .select_submodes
                    .get(sub)
                    .and_then(|kb| {
                        kb.get(&key).map(|m| {
                            m.actions
                                .iter()
                                .map(|a| Action::SelectMode(a.clone()))
                                .collect()
                        })
                    }),
            })
    }

    pub fn handle(self, action: &Action) -> Result<Self, Error> {
        match action {
            // Global actions
            Action::Global(GlobalAction::ToggleShowHidden) => self.toggle_hidden(),
            Action::Global(GlobalAction::Back) => self.back(),
            Action::Global(GlobalAction::Enter) => self.enter(),
            Action::Global(GlobalAction::FocusNext) => self.focus_next_item(),
            Action::Global(GlobalAction::FocusPrevious) => self.focus_previous_item(),
            Action::Global(GlobalAction::FocusFirst) => self.focus_first_item(),
            Action::Global(GlobalAction::FocusLast) => self.focus_last_item(),
            Action::Global(GlobalAction::FocusPath(path)) => self.focus_path(&path.into()),
            Action::Global(GlobalAction::FocusPathByIndex(n)) => self.focus_by_index(n),
            Action::Global(GlobalAction::FocusPathByBufferRelativeIndex(n)) => {
                self.focus_by_buffer_relative_index(&n)
            }
            Action::Global(GlobalAction::FocusPathByFocusRelativeIndex(n)) => {
                self.focus_by_focus_relative_index(&n)
            }
            Action::Global(GlobalAction::ChangeDirectory(dir)) => self.change_directory(&dir),
            Action::Global(GlobalAction::Call(cmd)) => self.call(&cmd),
            Action::Global(GlobalAction::PrintFocused) => self.print_focused(),
            Action::Global(GlobalAction::PrintPwd) => self.print_pwd(),
            Action::Global(GlobalAction::PrintAppState) => self.print_app_state(),
            Action::Global(GlobalAction::Quit) => self.quit(),
            Action::Global(GlobalAction::Terminate) => self.terminate(),

            // Explore mode
            Action::ExploreMode(ExploreModeAction::ToggleShowHidden) => self.toggle_hidden(),
            Action::ExploreMode(ExploreModeAction::Back) => self.back(),
            Action::ExploreMode(ExploreModeAction::Enter) => self.enter(),
            Action::ExploreMode(ExploreModeAction::FocusNext) => self.focus_next_item(),
            Action::ExploreMode(ExploreModeAction::FocusPrevious) => self.focus_previous_item(),
            Action::ExploreMode(ExploreModeAction::FocusFirst) => self.focus_first_item(),
            Action::ExploreMode(ExploreModeAction::FocusLast) => self.focus_last_item(),
            Action::ExploreMode(ExploreModeAction::FocusPath(path)) => {
                self.focus_path(&path.into())
            }
            Action::ExploreMode(ExploreModeAction::FocusPathByIndex(n)) => self.focus_by_index(n),
            Action::ExploreMode(ExploreModeAction::FocusPathByBufferRelativeIndex(n)) => {
                self.focus_by_buffer_relative_index(&n)
            }
            Action::ExploreMode(ExploreModeAction::FocusPathByFocusRelativeIndex(n)) => {
                self.focus_by_focus_relative_index(&n)
            }
            Action::ExploreMode(ExploreModeAction::ChangeDirectory(dir)) => {
                self.change_directory(&dir)
            }
            Action::ExploreMode(ExploreModeAction::Call(cmd)) => self.call(&cmd),
            Action::ExploreMode(ExploreModeAction::Select) => self.select(),
            Action::ExploreMode(ExploreModeAction::EnterSubmode(submode)) => {
                self.enter_submode(submode)
            }
            Action::ExploreMode(ExploreModeAction::ExitSubmode) => self.exit_submode(),
            Action::ExploreMode(ExploreModeAction::PrintFocused) => self.print_focused(),
            Action::ExploreMode(ExploreModeAction::PrintPwd) => self.print_pwd(),
            Action::ExploreMode(ExploreModeAction::PrintAppState) => self.print_app_state(),
            Action::ExploreMode(ExploreModeAction::Quit) => self.quit(),

            // Select mode
            Action::SelectMode(SelectModeAction::ToggleShowHidden) => self.toggle_hidden(),
            Action::SelectMode(SelectModeAction::Back) => self.back(),
            Action::SelectMode(SelectModeAction::Enter) => self.enter(),
            Action::SelectMode(SelectModeAction::FocusNext) => self.focus_next_item(),
            Action::SelectMode(SelectModeAction::FocusPrevious) => self.focus_previous_item(),
            Action::SelectMode(SelectModeAction::FocusFirst) => self.focus_first_item(),
            Action::SelectMode(SelectModeAction::FocusLast) => self.focus_last_item(),
            Action::SelectMode(SelectModeAction::FocusPath(path)) => self.focus_path(&path.into()),
            Action::SelectMode(SelectModeAction::FocusPathByIndex(n)) => self.focus_by_index(n),
            Action::SelectMode(SelectModeAction::FocusPathByBufferRelativeIndex(n)) => {
                self.focus_by_buffer_relative_index(&n)
            }
            Action::SelectMode(SelectModeAction::FocusPathByFocusRelativeIndex(n)) => {
                self.focus_by_focus_relative_index(&n)
            }
            Action::SelectMode(SelectModeAction::ChangeDirectory(dir)) => {
                self.change_directory(&dir)
            }
            Action::SelectMode(SelectModeAction::Call(cmd)) => self.call(&cmd),
            Action::SelectMode(SelectModeAction::ToggleSelection) => self.toggle_selection(),
            Action::SelectMode(SelectModeAction::EnterSubmode(submode)) => {
                self.enter_submode(submode)
            }
            Action::SelectMode(SelectModeAction::ExitSubmode) => self.exit_submode(),
            Action::SelectMode(SelectModeAction::PrintSelected) => self.print_selected(),
            Action::SelectMode(SelectModeAction::PrintAppState) => self.print_app_state(),
            Action::SelectMode(SelectModeAction::Quit) => self.quit(),
        }
    }
}

pub fn create() -> Result<App, Error> {
    let config_dir = dirs::config_dir()
        .unwrap_or(PathBuf::from("."))
        .join("xplr");

    let config_file = config_dir.join("config.yml");

    let config: Config = if config_file.exists() {
        serde_yaml::from_reader(BufReader::new(&File::open(&config_file)?))?
    } else {
        Config::default()
    };

    if !config.version.eq(VERSION) {
        return Err(Error::IncompatibleVersion(
            format!("Config file {} is outdated", config_file.to_string_lossy()),
        ));
    }

    let root = Path::new("/");
    let pwd = PathBuf::from(std::env::args().skip(1).next().unwrap_or("./".into()))
        .canonicalize()
        .unwrap_or(root.into());

    let (pwd, file_to_focus) = if pwd.is_file() {
        (pwd.parent().unwrap_or(root).into(), Some(pwd))
    } else {
        (pwd, None)
    };

    let app = App::new(
        &config,
        &pwd,
        &Default::default(),
        &Default::default(),
        Mode::Explore,
        config.general.show_hidden,
        None,
    )?;

    if let Some(file) = file_to_focus {
        app.focus_path(&file)
    } else {
        Ok(app)
    }
}
