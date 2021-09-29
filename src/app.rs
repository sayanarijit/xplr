use crate::cli::Cli;
use crate::config::Config;
use crate::config::Mode;
use crate::explorer;
use crate::input::Key;
use crate::lua;
use crate::permissions::Permissions;
use crate::runner::Runner;
use crate::ui::Layout;
use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use humansize::{file_size_opts as options, FileSize};
use indexmap::set::IndexSet;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TEMPLATE_TABLE_ROW: &str = "TEMPLATE_TABLE_ROW";
pub const UNSUPPORTED_STR: &str = "???";

fn to_humansize(size: u64) -> String {
    size.file_size(options::CONVENTIONAL)
        .unwrap_or_else(|_| format!("{} B", size))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipe {
    pub path: String,
    pub msg_in: String,
    pub selection_out: String,
    pub result_out: String,
    pub directory_nodes_out: String,
    pub global_help_menu_out: String,
    pub logs_out: String,
    pub history_out: String,
}

impl Pipe {
    fn from_session_path(path: &str) -> Result<Self> {
        let path = PathBuf::from(path).join("pipe");

        let msg_in = path.join("msg_in").to_string_lossy().to_string();

        let selection_out = path.join("selection_out").to_string_lossy().to_string();

        let result_out = path.join("result_out").to_string_lossy().to_string();

        let directory_nodes_out = path
            .join("directory_nodes_out")
            .to_string_lossy()
            .to_string();

        let global_help_menu_out = path
            .join("global_help_menu_out")
            .to_string_lossy()
            .to_string();

        let logs_out = path.join("logs_out").to_string_lossy().to_string();

        let history_out = path.join("history_out").to_string_lossy().to_string();

        Ok(Self {
            path: path.to_string_lossy().to_string(),
            msg_in,
            selection_out,
            result_out,
            directory_nodes_out,
            global_help_menu_out,
            logs_out,
            history_out,
        })
    }

    /// Get a reference to the pipe's msg in.
    pub fn msg_in(&self) -> &String {
        &self.msg_in
    }

    /// Get a reference to the pipe's selection out.
    pub fn selection_out(&self) -> &String {
        &self.selection_out
    }

    /// Get a reference to the pipe's result out.
    pub fn result_out(&self) -> &String {
        &self.result_out
    }

    /// Get a reference to the pipe's directory nodes out.
    pub fn directory_nodes_out(&self) -> &String {
        &self.directory_nodes_out
    }

    /// Get a reference to the pipe's global help menu out.
    pub fn global_help_menu_out(&self) -> &String {
        &self.global_help_menu_out
    }

    /// Get a reference to the pipe's logs out.
    pub fn logs_out(&self) -> &String {
        &self.logs_out
    }

    /// Get a reference to the pipe's history out.
    pub fn history_out(&self) -> &String {
        &self.history_out
    }

    /// Get a reference to the pipe's path.
    pub fn path(&self) -> &String {
        &self.path
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ResolvedNode {
    pub absolute_path: String,
    pub extension: String,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_readonly: bool,
    pub mime_essence: String,
    pub size: u64,
    pub human_size: String,
}

impl ResolvedNode {
    pub fn from(path: PathBuf) -> Self {
        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();

        let (is_dir, is_file, is_readonly, size) = path
            .metadata()
            .map(|m| (m.is_dir(), m.is_file(), m.permissions().readonly(), m.len()))
            .unwrap_or((false, false, false, 0));

        let mime_essence = mime_guess::from_path(&path)
            .first()
            .map(|m| m.essence_str().to_string())
            .unwrap_or_default();

        let human_size = to_humansize(size);

        Self {
            absolute_path: path.to_string_lossy().to_string(),
            extension,
            is_dir,
            is_file,
            is_readonly,
            mime_essence,
            size,
            human_size,
        }
    }

    /// Get a reference to the resolved node's absolute path.
    pub fn absolute_path(&self) -> &String {
        &self.absolute_path
    }

    /// Get a reference to the resolved node's extension.
    pub fn extension(&self) -> &String {
        &self.extension
    }

    /// Get a reference to the resolved node's is dir.
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    /// Get a reference to the resolved node's is file.
    pub fn is_file(&self) -> bool {
        self.is_file
    }

    /// Get a reference to the resolved node's is readonly.
    pub fn is_readonly(&self) -> bool {
        self.is_readonly
    }

    /// Get a reference to the resolved node's mime essence.
    pub fn mime_essence(&self) -> &String {
        &self.mime_essence
    }

    /// Get a reference to the resolved node's size.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Get a reference to the resolved node's human size.
    pub fn human_size(&self) -> &String {
        &self.human_size
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub parent: String,
    pub relative_path: String,
    pub absolute_path: String,
    pub extension: String,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub is_broken: bool,
    pub is_readonly: bool,
    pub mime_essence: String,
    pub size: u64,
    pub human_size: String,
    pub permissions: Permissions,
    pub canonical: Option<ResolvedNode>,
    pub symlink: Option<ResolvedNode>,
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

        let (is_broken, maybe_canonical_meta) = path
            .canonicalize()
            .map(|p| (false, Some(ResolvedNode::from(p))))
            .unwrap_or_else(|_| (true, None));

        let (is_symlink, is_dir, is_file, is_readonly, size, permissions) = path
            .symlink_metadata()
            .map(|m| {
                (
                    m.file_type().is_symlink(),
                    m.is_dir(),
                    m.is_file(),
                    m.permissions().readonly(),
                    m.len(),
                    Permissions::from(&m),
                )
            })
            .unwrap_or_else(|_| (false, false, false, false, 0, Permissions::default()));

        let mime_essence = mime_guess::from_path(&path)
            .first()
            .map(|m| m.essence_str().to_string())
            .unwrap_or_default();

        let human_size = to_humansize(size);

        Self {
            parent,
            relative_path,
            absolute_path,
            extension,
            is_dir,
            is_file,
            is_symlink,
            is_broken,
            is_readonly,
            mime_essence,
            size,
            human_size,
            permissions,
            canonical: maybe_canonical_meta.clone(),
            symlink: if is_symlink {
                maybe_canonical_meta
            } else {
                None
            },
        }
    }

    /// Get a reference to the node's parent.
    pub fn parent(&self) -> &String {
        &self.parent
    }

    /// Get a reference to the node's relative path.
    pub fn relative_path(&self) -> &String {
        &self.relative_path
    }

    /// Get a reference to the node's extension.
    pub fn extension(&self) -> &String {
        &self.extension
    }

    /// Get a reference to the node's is dir.
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    /// Get a reference to the node's is file.
    pub fn is_file(&self) -> bool {
        self.is_file
    }

    /// Get a reference to the node's is symlink.
    pub fn is_symlink(&self) -> bool {
        self.is_symlink
    }

    /// Get a reference to the node's is broken.
    pub fn is_broken(&self) -> bool {
        self.is_broken
    }

    /// Get a reference to the node's is readonly.
    pub fn is_readonly(&self) -> bool {
        self.is_readonly
    }

    /// Get a reference to the node's mime essence.
    pub fn mime_essence(&self) -> &String {
        &self.mime_essence
    }

    /// Get a reference to the node's size.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Get a reference to the node's canonical.
    pub fn canonical(&self) -> &Option<ResolvedNode> {
        &self.canonical
    }

    /// Get a reference to the node's symlink.
    pub fn symlink(&self) -> &Option<ResolvedNode> {
        &self.symlink
    }

    /// Get a reference to the node's absolute path.
    pub fn absolute_path(&self) -> &String {
        &self.absolute_path
    }

    /// Get a reference to the node's human size.
    pub fn human_size(&self) -> &String {
        &self.human_size
    }

    /// Get a reference to the node's permissions.
    pub fn permissions(&self) -> &Permissions {
        &self.permissions
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

    /// Get a reference to the directory buffer's parent.
    pub fn parent(&self) -> &String {
        &self.parent
    }

    /// Get a reference to the directory buffer's nodes.
    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    /// Get a reference to the directory buffer's total.
    pub fn total(&self) -> usize {
        self.total
    }

    /// Get a reference to the directory buffer's focus.
    pub fn focus(&self) -> usize {
        self.focus
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum InternalMsg {
    AddLastFocus(String, Option<String>),
    SetDirectory(DirectoryBuffer),
    HandleKey(Key),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum NodeSorter {
    ByRelativePath,
    ByIRelativePath,
    ByExtension,
    ByIsDir,
    ByIsFile,
    ByIsSymlink,
    ByIsBroken,
    ByIsReadonly,
    ByMimeEssence,
    BySize,

    ByCanonicalAbsolutePath,
    ByICanonicalAbsolutePath,
    ByCanonicalExtension,
    ByCanonicalIsDir,
    ByCanonicalIsFile,
    ByCanonicalIsReadonly,
    ByCanonicalMimeEssence,
    ByCanonicalSize,

    BySymlinkAbsolutePath,
    ByISymlinkAbsolutePath,
    BySymlinkExtension,
    BySymlinkIsDir,
    BySymlinkIsFile,
    BySymlinkIsReadonly,
    BySymlinkMimeEssence,
    BySymlinkSize,
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeSorterApplicable {
    pub sorter: NodeSorter,
    #[serde(default)]
    pub reverse: bool,
}

impl PartialEq for NodeSorterApplicable {
    fn eq(&self, other: &NodeSorterApplicable) -> bool {
        self.sorter == other.sorter
    }
}

impl std::hash::Hash for NodeSorterApplicable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sorter.hash(state);
    }
}

impl NodeSorterApplicable {
    fn reversed(mut self) -> Self {
        self.reverse = !self.reverse;
        self
    }

    fn apply(&self, a: &Node, b: &Node) -> Ordering {
        match (self.sorter, self.reverse) {
            (NodeSorter::ByRelativePath, false) => {
                natord::compare(&a.relative_path, &b.relative_path)
            }
            (NodeSorter::ByIRelativePath, false) => {
                natord::compare_ignore_case(&a.relative_path, &b.relative_path)
            }
            (NodeSorter::ByRelativePath, true) => {
                natord::compare(&b.relative_path, &a.relative_path)
            }
            (NodeSorter::ByIRelativePath, true) => {
                natord::compare_ignore_case(&b.relative_path, &a.relative_path)
            }
            (NodeSorter::ByExtension, false) => a.extension.cmp(&b.extension),
            (NodeSorter::ByExtension, true) => b.extension.cmp(&a.extension),
            (NodeSorter::ByIsDir, false) => a.is_dir.cmp(&b.is_dir),
            (NodeSorter::ByIsDir, true) => b.is_dir.cmp(&a.is_dir),
            (NodeSorter::ByIsFile, false) => a.is_file.cmp(&b.is_file),
            (NodeSorter::ByIsFile, true) => b.is_file.cmp(&a.is_file),
            (NodeSorter::ByIsSymlink, false) => a.is_symlink.cmp(&b.is_symlink),
            (NodeSorter::ByIsSymlink, true) => b.is_symlink.cmp(&a.is_symlink),
            (NodeSorter::ByIsBroken, false) => a.is_broken.cmp(&b.is_broken),
            (NodeSorter::ByIsBroken, true) => b.is_broken.cmp(&a.is_broken),
            (NodeSorter::ByIsReadonly, false) => a.is_readonly.cmp(&b.is_readonly),
            (NodeSorter::ByIsReadonly, true) => b.is_readonly.cmp(&a.is_readonly),
            (NodeSorter::ByMimeEssence, false) => a.mime_essence.cmp(&b.mime_essence),
            (NodeSorter::ByMimeEssence, true) => b.mime_essence.cmp(&a.mime_essence),
            (NodeSorter::BySize, false) => a.size.cmp(&b.size),
            (NodeSorter::BySize, true) => b.size.cmp(&a.size),

            (NodeSorter::ByCanonicalAbsolutePath, false) => natord::compare(
                &a.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &b.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            (NodeSorter::ByCanonicalAbsolutePath, true) => natord::compare(
                &b.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &a.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            (NodeSorter::ByICanonicalAbsolutePath, false) => natord::compare_ignore_case(
                &a.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &b.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            (NodeSorter::ByICanonicalAbsolutePath, true) => natord::compare_ignore_case(
                &b.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &a.canonical
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            (NodeSorter::ByCanonicalExtension, false) => a
                .canonical
                .as_ref()
                .map(|s| &s.extension)
                .cmp(&b.canonical.as_ref().map(|s| &s.extension)),

            (NodeSorter::ByCanonicalExtension, true) => b
                .canonical
                .as_ref()
                .map(|s| &s.extension)
                .cmp(&a.canonical.as_ref().map(|s| &s.extension)),

            (NodeSorter::ByCanonicalIsDir, false) => a
                .canonical
                .as_ref()
                .map(|s| &s.is_dir)
                .cmp(&b.canonical.as_ref().map(|s| &s.is_dir)),

            (NodeSorter::ByCanonicalIsDir, true) => b
                .canonical
                .as_ref()
                .map(|s| &s.is_dir)
                .cmp(&a.canonical.as_ref().map(|s| &s.is_dir)),

            (NodeSorter::ByCanonicalIsFile, false) => a
                .canonical
                .as_ref()
                .map(|s| &s.is_file)
                .cmp(&b.canonical.as_ref().map(|s| &s.is_file)),

            (NodeSorter::ByCanonicalIsFile, true) => b
                .canonical
                .as_ref()
                .map(|s| &s.is_file)
                .cmp(&a.canonical.as_ref().map(|s| &s.is_file)),

            (NodeSorter::ByCanonicalIsReadonly, false) => a
                .canonical
                .as_ref()
                .map(|s| &s.is_readonly)
                .cmp(&b.canonical.as_ref().map(|s| &s.is_readonly)),

            (NodeSorter::ByCanonicalIsReadonly, true) => b
                .canonical
                .as_ref()
                .map(|s| &s.is_readonly)
                .cmp(&a.canonical.as_ref().map(|s| &s.is_readonly)),

            (NodeSorter::ByCanonicalMimeEssence, false) => a
                .canonical
                .as_ref()
                .map(|s| &s.mime_essence)
                .cmp(&b.canonical.as_ref().map(|s| &s.mime_essence)),

            (NodeSorter::ByCanonicalMimeEssence, true) => b
                .canonical
                .as_ref()
                .map(|s| &s.mime_essence)
                .cmp(&a.canonical.as_ref().map(|s| &s.mime_essence)),

            (NodeSorter::ByCanonicalSize, false) => a
                .canonical
                .as_ref()
                .map(|s| &s.size)
                .cmp(&b.canonical.as_ref().map(|s| &s.size)),

            (NodeSorter::ByCanonicalSize, true) => b
                .canonical
                .as_ref()
                .map(|s| &s.size)
                .cmp(&a.canonical.as_ref().map(|s| &s.size)),

            (NodeSorter::BySymlinkAbsolutePath, false) => natord::compare(
                &a.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &b.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            (NodeSorter::BySymlinkAbsolutePath, true) => natord::compare(
                &b.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &a.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            (NodeSorter::ByISymlinkAbsolutePath, false) => natord::compare_ignore_case(
                &a.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &b.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            (NodeSorter::ByISymlinkAbsolutePath, true) => natord::compare_ignore_case(
                &b.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
                &a.symlink
                    .as_ref()
                    .map(|s| s.absolute_path.clone())
                    .unwrap_or_default(),
            ),

            (NodeSorter::BySymlinkExtension, false) => a
                .symlink
                .as_ref()
                .map(|s| &s.extension)
                .cmp(&b.symlink.as_ref().map(|s| &s.extension)),

            (NodeSorter::BySymlinkExtension, true) => b
                .symlink
                .as_ref()
                .map(|s| &s.extension)
                .cmp(&a.symlink.as_ref().map(|s| &s.extension)),

            (NodeSorter::BySymlinkIsDir, false) => a
                .symlink
                .as_ref()
                .map(|s| &s.is_dir)
                .cmp(&b.symlink.as_ref().map(|s| &s.is_dir)),

            (NodeSorter::BySymlinkIsDir, true) => b
                .symlink
                .as_ref()
                .map(|s| &s.is_dir)
                .cmp(&a.symlink.as_ref().map(|s| &s.is_dir)),

            (NodeSorter::BySymlinkIsFile, false) => a
                .symlink
                .as_ref()
                .map(|s| &s.is_file)
                .cmp(&b.symlink.as_ref().map(|s| &s.is_file)),

            (NodeSorter::BySymlinkIsFile, true) => b
                .symlink
                .as_ref()
                .map(|s| &s.is_file)
                .cmp(&a.symlink.as_ref().map(|s| &s.is_file)),

            (NodeSorter::BySymlinkIsReadonly, false) => a
                .symlink
                .as_ref()
                .map(|s| &s.is_readonly)
                .cmp(&b.symlink.as_ref().map(|s| &s.is_readonly)),

            (NodeSorter::BySymlinkIsReadonly, true) => b
                .symlink
                .as_ref()
                .map(|s| &s.is_readonly)
                .cmp(&a.symlink.as_ref().map(|s| &s.is_readonly)),

            (NodeSorter::BySymlinkMimeEssence, false) => a
                .symlink
                .as_ref()
                .map(|s| &s.mime_essence)
                .cmp(&b.symlink.as_ref().map(|s| &s.mime_essence)),

            (NodeSorter::BySymlinkMimeEssence, true) => b
                .symlink
                .as_ref()
                .map(|s| &s.mime_essence)
                .cmp(&a.symlink.as_ref().map(|s| &s.mime_essence)),

            (NodeSorter::BySymlinkSize, false) => a
                .symlink
                .as_ref()
                .map(|s| &s.size)
                .cmp(&b.symlink.as_ref().map(|s| &s.size)),

            (NodeSorter::BySymlinkSize, true) => b
                .symlink
                .as_ref()
                .map(|s| &s.size)
                .cmp(&a.symlink.as_ref().map(|s| &s.size)),
        }
    }

    /// Get a reference to the node sorter applicable's sorter.
    pub fn sorter(&self) -> &NodeSorter {
        &self.sorter
    }

    /// Get a reference to the node sorter applicable's reverse.
    pub fn reverse(&self) -> bool {
        self.reverse
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum NodeFilter {
    RelativePathIs,
    RelativePathIsNot,

    IRelativePathIs,
    IRelativePathIsNot,

    RelativePathDoesStartWith,
    RelativePathDoesNotStartWith,

    IRelativePathDoesStartWith,
    IRelativePathDoesNotStartWith,

    RelativePathDoesContain,
    RelativePathDoesNotContain,

    IRelativePathDoesContain,
    IRelativePathDoesNotContain,

    RelativePathDoesEndWith,
    RelativePathDoesNotEndWith,

    IRelativePathDoesEndWith,
    IRelativePathDoesNotEndWith,

    AbsolutePathIs,
    AbsolutePathIsNot,

    IAbsolutePathIs,
    IAbsolutePathIsNot,

    AbsolutePathDoesStartWith,
    AbsolutePathDoesNotStartWith,

    IAbsolutePathDoesStartWith,
    IAbsolutePathDoesNotStartWith,

    AbsolutePathDoesContain,
    AbsolutePathDoesNotContain,

    IAbsolutePathDoesContain,
    IAbsolutePathDoesNotContain,

    AbsolutePathDoesEndWith,
    AbsolutePathDoesNotEndWith,

    IAbsolutePathDoesEndWith,
    IAbsolutePathDoesNotEndWith,
}

impl NodeFilter {
    fn apply(&self, node: &Node, input: &str) -> bool {
        match self {
            Self::RelativePathIs => node.relative_path.eq(input),
            Self::IRelativePathIs => node.relative_path.eq_ignore_ascii_case(input),

            Self::RelativePathIsNot => !node.relative_path.eq(input),
            Self::IRelativePathIsNot => !node.relative_path.eq_ignore_ascii_case(input),

            Self::RelativePathDoesStartWith => node.relative_path.starts_with(input),
            Self::IRelativePathDoesStartWith => node
                .relative_path
                .to_lowercase()
                .starts_with(&input.to_lowercase()),

            Self::RelativePathDoesNotStartWith => !node.relative_path.starts_with(input),

            Self::IRelativePathDoesNotStartWith => !node
                .relative_path
                .to_lowercase()
                .starts_with(&input.to_lowercase()),

            Self::RelativePathDoesContain => node.relative_path.contains(input),
            Self::IRelativePathDoesContain => node
                .relative_path
                .to_lowercase()
                .contains(&input.to_lowercase()),

            Self::RelativePathDoesNotContain => !node.relative_path.contains(input),
            Self::IRelativePathDoesNotContain => !node
                .relative_path
                .to_lowercase()
                .contains(&input.to_lowercase()),

            Self::RelativePathDoesEndWith => node.relative_path.ends_with(input),
            Self::IRelativePathDoesEndWith => node
                .relative_path
                .to_lowercase()
                .ends_with(&input.to_lowercase()),

            Self::RelativePathDoesNotEndWith => !node.relative_path.ends_with(input),
            Self::IRelativePathDoesNotEndWith => !node
                .relative_path
                .to_lowercase()
                .ends_with(&input.to_lowercase()),

            Self::AbsolutePathIs => node.absolute_path.eq(input),
            Self::IAbsolutePathIs => node.absolute_path.eq_ignore_ascii_case(input),

            Self::AbsolutePathIsNot => !node.absolute_path.eq(input),
            Self::IAbsolutePathIsNot => !node.absolute_path.eq_ignore_ascii_case(input),

            Self::AbsolutePathDoesStartWith => node.absolute_path.starts_with(input),
            Self::IAbsolutePathDoesStartWith => node
                .absolute_path
                .to_lowercase()
                .starts_with(&input.to_lowercase()),

            Self::AbsolutePathDoesNotStartWith => !node.absolute_path.starts_with(input),
            Self::IAbsolutePathDoesNotStartWith => !node
                .absolute_path
                .to_lowercase()
                .starts_with(&input.to_lowercase()),

            Self::AbsolutePathDoesContain => node.absolute_path.contains(input),
            Self::IAbsolutePathDoesContain => node
                .absolute_path
                .to_lowercase()
                .contains(&input.to_lowercase()),

            Self::AbsolutePathDoesNotContain => !node.absolute_path.contains(input),
            Self::IAbsolutePathDoesNotContain => !node
                .absolute_path
                .to_lowercase()
                .contains(&input.to_lowercase()),

            Self::AbsolutePathDoesEndWith => node.absolute_path.ends_with(input),
            Self::IAbsolutePathDoesEndWith => node
                .absolute_path
                .to_lowercase()
                .ends_with(&input.to_lowercase()),

            Self::AbsolutePathDoesNotEndWith => !node.absolute_path.ends_with(input),
            Self::IAbsolutePathDoesNotEndWith => !node
                .absolute_path
                .to_lowercase()
                .ends_with(&input.to_lowercase()),
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeFilterApplicable {
    pub filter: NodeFilter,
    pub input: String,
}

impl NodeFilterApplicable {
    pub fn new(filter: NodeFilter, input: String) -> Self {
        Self { filter, input }
    }

    fn apply(&self, node: &Node) -> bool {
        self.filter.apply(node, &self.input)
    }

    /// Get a reference to the node filter applicable's filter.
    pub fn filter(&self) -> &NodeFilter {
        &self.filter
    }

    /// Get a reference to the node filter applicable's input.
    pub fn input(&self) -> &String {
        &self.input
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExplorerConfig {
    pub filters: IndexSet<NodeFilterApplicable>,
    pub sorters: IndexSet<NodeSorterApplicable>,
}

impl ExplorerConfig {
    pub fn filter(&self, node: &Node) -> bool {
        self.filters.iter().all(|f| f.apply(node))
    }

    pub fn sort(&self, a: &Node, b: &Node) -> Ordering {
        let mut ord = Ordering::Equal;
        for s in self.sorters.iter() {
            ord = ord.then(s.apply(a, b));
        }
        ord
    }

    /// Get a reference to the explorer config's filters.
    pub fn filters(&self) -> &IndexSet<NodeFilterApplicable> {
        &self.filters
    }

    /// Get a reference to the explorer config's sorters.
    pub fn sorters(&self) -> &IndexSet<NodeSorterApplicable> {
        &self.sorters
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExternalMsg {
    /// Explore the present working directory and register the filtered nodes.
    /// This operation is expensive. So, try to avoid using it too often.
    ExplorePwd,

    /// Explore the present working directory and register the filtered nodes asynchronously.
    /// This operation happens asynchronously. That means, the xplr directory buffers won't be updated
    /// immediately. Hence, it needs to be used with care and probably with special checks in place.
    /// To explore `$PWD` synchronously, use `ExplorePwd` instead.
    ExplorePwdAsync,

    /// Explore the present working directory along with its parents and register the filtered nodes.
    /// This operation happens asynchronously. That means, the xplr directory buffers won't be updated
    /// immediately. Hence, it needs to be used with care and probably with special checks in place.
    /// To explore just the `$PWD` synchronously, use `ExplorePwd` instead.
    ExploreParentsAsync,

    /// Refresh the UI.
    /// But it will not re-explore the directory if the working directory is the same.
    /// If there is some change in the working directory and you want to re-explore it,
    /// use the `Explore` message instead.
    /// Also, it will not clear the screen. Use `ClearScreen` for that.
    Refresh,

    /// Clears the screen.
    ClearScreen,

    /// Focus next node.
    FocusNext,

    /// Focus on the `n`th node relative to the current focus where `n` is a given value.
    ///
    /// **Example:** `FocusNextByRelativeIndex: 2`
    FocusNextByRelativeIndex(usize),

    /// Focus on the `n`th node relative to the current focus where `n` is read from
    /// the input buffer.
    FocusNextByRelativeIndexFromInput,

    /// Focus on the previous item.
    FocusPrevious,

    /// Focus on the `-n`th node relative to the current focus where `n` is a given value.
    ///
    /// **Example:** `FocusPreviousByRelativeIndex: 2`
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
    /// **Example:** `FocusPath: /tmp`
    FocusPath(String),

    /// Focus on the path read from input buffer.
    FocusPathFromInput,

    /// Focus on the absolute `n`th node where `n` is a given value.
    ///
    /// **Example:** `FocusByIndex: 2`
    FocusByIndex(usize),

    /// Focus on the absolute `n`th node where `n` is read from the input buffer.
    FocusByIndexFromInput,

    /// Focus on the file by name from the present working directory.
    ///
    /// **Example:** `FocusByFileName: README.md`
    FocusByFileName(String),

    /// Change the present working directory ($PWD)
    ///
    /// **Example:** `ChangeDirectory: /tmp`
    ChangeDirectory(String),

    /// Enter into the currently focused path if it's a directory.
    Enter,

    /// Go back to the parent directory.
    Back,

    /// Go to the last path visited.
    LastVisitedPath,

    /// Go to the next path visited.
    NextVisitedPath,

    /// Follow the symlink under focus to its actual location.
    FollowSymlink,

    /// Append/buffer the given string into the input buffer.
    ///
    /// **Example:** `BufferInput: foo`
    BufferInput(String),

    /// Append/buffer the characted read from a keyboard input into the
    /// input buffer.
    BufferInputFromKey,

    /// Set/rewrite the input buffer with the given string.
    /// When the input buffer is not-null (even if empty string)
    /// it will show in the UI.
    ///
    /// **Example:** `SetInputBuffer: foo`
    SetInputBuffer(String),

    /// Remove input buffer's last character.
    RemoveInputBufferLastCharacter,

    /// Remove input buffer's last word.
    RemoveInputBufferLastWord,

    /// Reset the input buffer back to null. It will not show in the UI.
    ResetInputBuffer,

    /// Switch input mode.
    ///
    /// > **NOTE:** To be specific about which mode to switch to, use `SwitchModeBuiltin` or
    /// `SwitchModeCustom` instead.
    ///
    /// **Example:** `SwitchMode: default`
    SwitchMode(String),

    /// Switch to a builtin mode.
    ///
    /// **Example:** `SwitchModeBuiltin: default`
    SwitchModeBuiltin(String),

    /// Switch to a custom mode.
    ///
    /// **Example:** `SwitchModeCustom: my_custom_mode`
    SwitchModeCustom(String),

    /// Pop the last mode from the history and switch to it.
    PopMode,

    /// Switch layout.
    ///
    /// > **NOTE:** To be specific about which layout to switch to, use `SwitchLayoutBuiltin` or
    /// `SwitchLayoutCustom` instead.
    ///
    /// **Example:** `SwitchLayout: default`
    SwitchLayout(String),

    /// Switch to a builtin layout.
    ///
    /// **Example:** `SwitchLayoutBuiltin: default`
    SwitchLayoutBuiltin(String),

    /// Switch to a custom layout.
    ///
    /// **Example:** `SwitchLayoutCustom: my_custom_layout`
    SwitchLayoutCustom(String),

    /// Call a shell command with the given arguments.
    /// Note that the arguments will be shell-escaped.
    /// So to read the variables, the `-c` option of the shell
    /// can be used.
    /// You may need to pass `ExplorePwd` depening on the expectation.
    ///
    /// **Example:** `Call: {command: bash, args: ["-c", "read -p test"]}`
    Call(Command),

    /// Like `Call` but without the flicker. The stdin, stdout
    /// stderr will be piped to null. So it's non-interactive.
    ///
    /// **Example:** `CallSilently: {command: tput, args: ["bell"]}`
    CallSilently(Command),

    /// Call a Lua function.
    /// A `CallLuaArg` object will be passed to the function as argument.
    /// The function can optionally return a list of messages for xplr to handle
    /// after the executing the function.
    ///
    /// **Example:** `CallLua: custom.some_custom_funtion`
    CallLua(String),

    /// Like `CallLua` but without the flicker. The stdin, stdout
    /// stderr will be piped to null. So it's non-interactive.
    ///
    /// **Example:** `CallLuaSilently: custom.some_custom_function`
    CallLuaSilently(String),

    /// An alias to `Call: {command: bash, args: ["-c", "${command}"], silent: false}`
    /// where ${command} is the given value.
    ///
    /// **Example:** `BashExec: "read -p test"`
    BashExec(String),

    /// Like `BashExec` but without the flicker. The stdin, stdout
    /// stderr will be piped to null. So it's non-interactive.
    ///
    /// **Example:** `BashExecSilently: "tput bell"`
    BashExecSilently(String),

    /// Select the focused node.
    Select,

    /// Select all the visible nodes.
    SelectAll,

    /// Select the given path.
    ///
    /// **Example:** `SelectPath: "/tmp"`
    SelectPath(String),

    /// Unselect the focused node.
    UnSelect,

    /// Unselect all the visible nodes.
    UnSelectAll,

    /// UnSelect the given path.
    ///
    /// **Example:** `UnSelectPath: "/tmp"`
    UnSelectPath(String),

    /// Toggle selection on the focused node.
    ToggleSelection,

    /// Toggle between select all and unselect all.
    ToggleSelectAll,

    /// Toggle selection by file path.
    ///
    /// **Example:** `ToggleSelectionByPath: "/tmp"`
    ToggleSelectionByPath(String),

    /// Clear the selection.
    ClearSelection,

    /// Add a filter to exclude nodes while exploring directories.
    ///
    /// **Example:** `AddNodeFilter: {filter: RelativePathDoesStartWith, input: foo}`
    AddNodeFilter(NodeFilterApplicable),

    /// Remove an existing filter.
    ///
    /// **Example:** `RemoveNodeFilter: {filter: RelativePathDoesStartWith, input: foo}`
    RemoveNodeFilter(NodeFilterApplicable),

    /// Remove a filter if it exists, else, add a it.
    ///
    /// **Example:** `ToggleNodeFilter: {filter: RelativePathDoesStartWith, input: foo}`
    ToggleNodeFilter(NodeFilterApplicable),

    /// Add a node filter reading the input from the buffer.
    ///
    /// **Example:** `AddNodeFilterFromInput: RelativePathDoesStartWith`
    AddNodeFilterFromInput(NodeFilter),

    /// Remove a node filter reading the input from the buffer.
    ///
    /// **Example:** `RemoveNodeFilterFromInput: RelativePathDoesStartWith`
    RemoveNodeFilterFromInput(NodeFilter),

    /// Remove the last node filter.
    RemoveLastNodeFilter,

    /// Reset the node filters back to the default configuration.
    ResetNodeFilters,

    /// Clear all the node filters.
    ClearNodeFilters,

    /// Add a sorter to sort nodes while exploring directories.
    ///
    /// **Example:** `AddNodeSorter: {sorter: ByRelativePath, reverse: false}`
    AddNodeSorter(NodeSorterApplicable),

    /// Remove an existing sorter.
    ///
    /// **Example:** `RemoveNodeSorter: ByRelativePath`
    RemoveNodeSorter(NodeSorter),

    /// Reverse a node sorter.
    ///
    /// **Example:** `ReverseNodeSorter: ByRelativePath`
    ReverseNodeSorter(NodeSorter),

    /// Remove a sorter if it exists, else, add a it.
    ///
    /// **Example:** `ToggleSorterSorter: {sorter: ByRelativePath, reverse: false}`
    ToggleNodeSorter(NodeSorterApplicable),

    /// Reverse the node sorters.
    ReverseNodeSorters,

    /// Remove the last node sorter.
    RemoveLastNodeSorter,

    /// Reset the node sorters back to the default configuration.
    ResetNodeSorters,

    /// Clear all the node sorters.
    ClearNodeSorters,

    /// Enable mouse
    EnableMouse,

    /// Disable mouse
    DisableMouse,

    /// Toggle mouse
    ToggleMouse,

    /// Start piping the focused path to the given fifo path
    ///
    /// **Example:** `StartFifo: /tmp/xplr.fifo`
    StartFifo(String),

    /// Close the active fifo and stop piping.
    StopFifo,

    /// Toggle betwen {Start|Stop}Fifo
    ToggleFifo(String),

    /// Log information message.
    ///
    /// **Example:** `LogInfo: launching satellite`
    LogInfo(String),

    /// Log a success message.
    ///
    /// **Example:** `LogSuccess: satellite reached destination`.
    LogSuccess(String),

    /// Log an warning message.
    ///
    /// **Example:** `LogWarning: satellite is heating`
    LogWarning(String),

    /// Log an error message.
    ///
    /// **Example:** `LogError: satellite crashed`
    LogError(String),

    /// Quit with returncode zero (success).
    Quit,

    /// Print $PWD and quit.
    PrintPwdAndQuit,

    /// Print the path under focus and quit. It can be empty string if there's nothing to focus.
    PrintFocusPathAndQuit,

    /// Print the selected paths and quit. It can be empty is no path is selected.
    PrintSelectionAndQuit,

    /// Print the selected paths if it's not empty, else, print the focused node's path.
    PrintResultAndQuit,

    /// Print the state of application in YAML format. Helpful for debugging or generating
    /// the default configuration file.
    PrintAppStateAndQuit,

    /// Write the application state to a file, without quitting. Also helpful for debugging.
    Debug(String),

    /// Terminate the application with a non-zero return code.
    Terminate,
}

impl ExternalMsg {
    pub fn is_read_only(&self) -> bool {
        !matches!(
            self,
            Self::Call(_) | Self::CallSilently(_) | Self::BashExec(_) | Self::BashExecSilently(_)
        )
    }
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

impl Command {
    /// Get a reference to the command's command.
    pub fn command(&self) -> &String {
        &self.command
    }

    /// Get a reference to the command's args.
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MsgOut {
    ExplorePwdAsync,
    ExploreParentsAsync,
    Refresh,
    ClearScreen,
    Quit,
    Debug(String),
    Call(Command),
    CallSilently(Command),
    CallLua(String),
    CallLuaSilently(String),
    Enque(Task),
    EnableMouse,
    DisableMouse,
    ToggleMouse,
    StartFifo(String),
    StopFifo,
    ToggleFifo(String),
    PrintPwdAndQuit,
    PrintFocusPathAndQuit,
    PrintSelectionAndQuit,
    PrintResultAndQuit,
    PrintAppStateAndQuit,
}

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

    /// Get a reference to the log's created at.
    pub fn created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }

    /// Get a reference to the log's message.
    pub fn message(&self) -> &String {
        &self.message
    }

    /// Get a reference to the log's level.
    pub fn level(&self) -> &LogLevel {
        &self.level
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
        write!(f, "[{}] {} {}", &self.created_at, level_str, &self.message)
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
    fn push(mut self, path: String) -> Self {
        if self.peek() != Some(&path) {
            self.paths = self.paths.into_iter().take(self.loc + 1).collect();
            self.paths.push(path);
            self.loc = self.paths.len().max(1) - 1;
        }
        self
    }

    fn visit_last(mut self) -> Self {
        self.loc = self.loc.max(1) - 1;
        self
    }

    fn visit_next(mut self) -> Self {
        self.loc = (self.loc + 1).min(self.paths.len().max(1) - 1);
        self
    }

    fn peek(&self) -> Option<&String> {
        self.paths.get(self.loc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLuaArg {
    pub version: String,
    pub pwd: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub version: String,
    pub config: Config,
    pub pwd: String,
    pub directory_buffer: Option<DirectoryBuffer>,
    pub last_focus: HashMap<String, Option<String>>,
    pub selection: IndexSet<Node>,
    pub msg_out: VecDeque<MsgOut>,
    pub mode: Mode,
    pub layout: Layout,
    pub input_buffer: Option<String>,
    pub pid: u32,
    pub session_path: String,
    pub pipe: Pipe,
    pub explorer_config: ExplorerConfig,
    pub logs: Vec<Log>,
    pub logs_hidden: bool,
    pub history: History,
    pub last_modes: Vec<Mode>,
}

impl App {
    pub fn create(
        pwd: PathBuf,
        lua: &mlua::Lua,
        config_file: Option<PathBuf>,
        extra_config_files: Vec<PathBuf>,
    ) -> Result<Self> {
        let mut config = lua::init(lua)?;

        let config_file = if let Some(path) = config_file {
            Some(path)
        } else if let Some(dir) = dirs::home_dir() {
            let path = dir.join(".config").join("xplr").join("init.lua");
            if path.exists() {
                Some(path)
            } else {
                None
            }
        } else {
            let path = PathBuf::from("/").join("etc").join("xplr").join("init.lua");
            if path.exists() {
                Some(path)
            } else {
                None
            }
        };

        let config_files = config_file
            .into_iter()
            .chain(extra_config_files.into_iter());

        let mut load_errs = vec![];
        for config_file in config_files {
            match lua::extend(lua, &config_file.to_string_lossy().to_string()) {
                Ok(c) => {
                    config = c;
                }
                Err(e) => {
                    load_errs.push(e.to_string());
                }
            }
        }

        let mode = match config.modes().get(
            &config
                .general()
                .initial_mode()
                .to_owned()
                .unwrap_or_else(|| "default".into()),
        ) {
            Some(m) => m.clone().sanitized(config.general().read_only()),
            None => {
                bail!("'default' mode is missing")
            }
        };

        let layout = match config.layouts().get(
            &config
                .general()
                .initial_layout()
                .to_owned()
                .unwrap_or_else(|| "default".into()),
        ) {
            Some(l) => l.clone(),
            None => {
                bail!("'default' layout is missing")
            }
        };

        let pid = std::process::id();
        let mut session_path = dirs::runtime_dir()
            .unwrap_or_else(env::temp_dir)
            .join("xplr")
            .join("session")
            .join(&pid.to_string())
            .to_string_lossy()
            .to_string();

        if fs::create_dir_all(&session_path).is_err() {
            session_path = env::temp_dir()
                .join("xplr")
                .join("session")
                .join(&pid.to_string())
                .to_string_lossy()
                .to_string();
            fs::create_dir_all(&session_path)?;
        }

        let mut explorer_config = ExplorerConfig::default();
        if !config.general().show_hidden() {
            explorer_config.filters.replace(NodeFilterApplicable::new(
                NodeFilter::RelativePathDoesNotStartWith,
                ".".into(),
            ));
        }

        if let Some(sorters) = &config.general().initial_sorting() {
            explorer_config.sorters = sorters.clone();
        };

        env::set_current_dir(&pwd)?;
        let pwd = pwd.to_string_lossy().to_string();

        let mut app = Self {
            version: VERSION.to_string(),
            config,
            pwd,
            directory_buffer: Default::default(),
            last_focus: Default::default(),
            selection: Default::default(),
            msg_out: Default::default(),
            mode,
            layout,
            input_buffer: Default::default(),
            pid,
            session_path: session_path.clone(),
            pipe: Pipe::from_session_path(&session_path)?,
            explorer_config,
            logs: Default::default(),
            logs_hidden: Default::default(),
            history: Default::default(),
            last_modes: Default::default(),
        };

        for err in load_errs {
            app = app.log_error(err)?
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

    fn enqueue(mut self, task: Task) -> Self {
        self.msg_out.push_back(MsgOut::Enque(task));
        self
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
        }
    }

    fn handle_external(self, msg: ExternalMsg, key: Option<Key>) -> Result<Self> {
        if self.config().general().read_only() && !msg.is_read_only() {
            self.log_error("Cannot call shell command in read-only mode.".into())
        } else {
            match msg {
                ExternalMsg::ExplorePwd => self.explore_pwd(),
                ExternalMsg::ExploreParentsAsync => self.explore_parents_async(),
                ExternalMsg::ExplorePwdAsync => self.explore_pwd_async(),
                ExternalMsg::Refresh => self.refresh(),
                ExternalMsg::ClearScreen => self.clear_screen(),
                ExternalMsg::FocusFirst => self.focus_first(true),
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
                ExternalMsg::FocusPath(p) => self.focus_path(&p, true),
                ExternalMsg::FocusPathFromInput => self.focus_path_from_input(),
                ExternalMsg::FocusByIndex(i) => self.focus_by_index(i),
                ExternalMsg::FocusByIndexFromInput => self.focus_by_index_from_input(),
                ExternalMsg::FocusByFileName(n) => self.focus_by_file_name(&n, true),
                ExternalMsg::ChangeDirectory(dir) => self.change_directory(&dir, true),
                ExternalMsg::Enter => self.enter(),
                ExternalMsg::Back => self.back(),
                ExternalMsg::LastVisitedPath => self.last_visited_path(),
                ExternalMsg::NextVisitedPath => self.next_visited_path(),
                ExternalMsg::FollowSymlink => self.follow_symlink(),
                ExternalMsg::BufferInput(input) => self.buffer_input(&input),
                ExternalMsg::BufferInputFromKey => self.buffer_input_from_key(key),
                ExternalMsg::SetInputBuffer(input) => self.set_input_buffer(input),
                ExternalMsg::RemoveInputBufferLastCharacter => {
                    self.remove_input_buffer_last_character()
                }
                ExternalMsg::RemoveInputBufferLastWord => self.remove_input_buffer_last_word(),
                ExternalMsg::ResetInputBuffer => self.reset_input_buffer(),
                ExternalMsg::SwitchMode(mode) => self.switch_mode(&mode),
                ExternalMsg::SwitchModeBuiltin(mode) => self.switch_mode_builtin(&mode),
                ExternalMsg::SwitchModeCustom(mode) => self.switch_mode_custom(&mode),
                ExternalMsg::PopMode => self.pop_mode(),
                ExternalMsg::SwitchLayout(mode) => self.switch_layout(&mode),
                ExternalMsg::SwitchLayoutBuiltin(mode) => self.switch_layout_builtin(&mode),
                ExternalMsg::SwitchLayoutCustom(mode) => self.switch_layout_custom(&mode),
                ExternalMsg::Call(cmd) => self.call(cmd),
                ExternalMsg::CallSilently(cmd) => self.call_silently(cmd),
                ExternalMsg::CallLua(func) => self.call_lua(func),
                ExternalMsg::CallLuaSilently(func) => self.call_lua_silently(func),
                ExternalMsg::BashExec(cmd) => self.bash_exec(cmd),
                ExternalMsg::BashExecSilently(cmd) => self.bash_exec_silently(cmd),
                ExternalMsg::Select => self.select(),
                ExternalMsg::SelectAll => self.select_all(),
                ExternalMsg::SelectPath(p) => self.select_path(p),
                ExternalMsg::UnSelect => self.un_select(),
                ExternalMsg::UnSelectAll => self.un_select_all(),
                ExternalMsg::UnSelectPath(p) => self.un_select_path(p),
                ExternalMsg::ToggleSelection => self.toggle_selection(),
                ExternalMsg::ToggleSelectAll => self.toggle_select_all(),
                ExternalMsg::ToggleSelectionByPath(p) => self.toggle_selection_by_path(p),
                ExternalMsg::ClearSelection => self.clear_selection(),
                ExternalMsg::AddNodeFilter(f) => self.add_node_filter(f),
                ExternalMsg::AddNodeFilterFromInput(f) => self.add_node_filter_from_input(f),
                ExternalMsg::RemoveNodeFilter(f) => self.remove_node_filter(f),
                ExternalMsg::RemoveNodeFilterFromInput(f) => self.remove_node_filter_from_input(f),
                ExternalMsg::ToggleNodeFilter(f) => self.toggle_node_filter(f),
                ExternalMsg::RemoveLastNodeFilter => self.remove_last_node_filter(),
                ExternalMsg::ResetNodeFilters => self.reset_node_filters(),
                ExternalMsg::ClearNodeFilters => self.clear_node_filters(),
                ExternalMsg::AddNodeSorter(f) => self.add_node_sorter(f),
                ExternalMsg::RemoveNodeSorter(f) => self.remove_node_sorter(f),
                ExternalMsg::ReverseNodeSorter(f) => self.reverse_node_sorter(f),
                ExternalMsg::ToggleNodeSorter(f) => self.toggle_node_sorter(f),
                ExternalMsg::RemoveLastNodeSorter => self.remove_last_node_sorter(),
                ExternalMsg::ReverseNodeSorters => self.reverse_node_sorters(),
                ExternalMsg::ResetNodeSorters => self.reset_node_sorters(),
                ExternalMsg::ClearNodeSorters => self.clear_node_sorters(),
                ExternalMsg::EnableMouse => self.enable_mouse(),
                ExternalMsg::DisableMouse => self.disable_mouse(),
                ExternalMsg::ToggleMouse => self.toggle_mouse(),
                ExternalMsg::StartFifo(f) => self.start_fifo(f),
                ExternalMsg::StopFifo => self.stop_fifo(),
                ExternalMsg::ToggleFifo(f) => self.toggle_fifo(f),
                ExternalMsg::LogInfo(l) => self.log_info(l),
                ExternalMsg::LogSuccess(l) => self.log_success(l),
                ExternalMsg::LogWarning(l) => self.log_warning(l),
                ExternalMsg::LogError(l) => self.log_error(l),
                ExternalMsg::Quit => self.quit(),
                ExternalMsg::PrintPwdAndQuit => self.print_pwd_and_quit(),
                ExternalMsg::PrintFocusPathAndQuit => self.print_focus_path_and_quit(),
                ExternalMsg::PrintSelectionAndQuit => self.print_selection_and_quit(),
                ExternalMsg::PrintResultAndQuit => self.print_result_and_quit(),
                ExternalMsg::PrintAppStateAndQuit => self.print_app_state_and_quit(),
                ExternalMsg::Debug(path) => self.debug(path),
                ExternalMsg::Terminate => bail!(""),
            }
        }?
        .refresh_selection()
    }

    fn handle_key(mut self, key: Key) -> Result<Self> {
        let kb = self.mode().key_bindings().clone();
        let key_str = key.to_string();
        let default = kb.default().clone();
        let msgs = kb
            .on_key()
            .get(&key_str)
            .to_owned()
            .map(|a| Some(a.messages().clone()))
            .unwrap_or_else(|| {
                if key.is_alphabet() {
                    kb.on_alphabet().clone().map(|a| a.messages().clone())
                } else if key.is_number() {
                    kb.on_number().clone().map(|a| a.messages().clone())
                } else if key.is_special_character() {
                    kb.on_special_character()
                        .clone()
                        .map(|a| a.messages().clone())
                } else {
                    None
                }
            })
            .or_else(|| default.map(|a| a.messages().clone()))
            .unwrap_or_else(|| {
                if self.config().general().enable_recover_mode() {
                    vec![ExternalMsg::SwitchModeBuiltin("recover".into())]
                } else {
                    vec![ExternalMsg::LogWarning("Key map not found.".into())]
                }
            });

        for msg in msgs {
            self = self.enqueue(Task::new(MsgIn::External(msg), Some(key)));
        }

        Ok(self)
    }

    pub fn explore_pwd(mut self) -> Result<Self> {
        let focus = self.last_focus.get(self.pwd()).cloned().unwrap_or(None);
        let pwd = self.pwd().clone();
        self = self.add_last_focus(pwd, focus.clone())?;
        let dir = explorer::explore_sync(
            self.explorer_config().clone(),
            self.pwd().into(),
            focus.map(PathBuf::from),
            self.directory_buffer().map(|d| d.focus()).unwrap_or(0),
        )?;
        self.set_directory(dir)
    }

    fn explore_pwd_async(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ExplorePwdAsync);
        Ok(self)
    }

    fn explore_parents_async(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ExploreParentsAsync);
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

    pub fn focus_first(mut self, save_history: bool) -> Result<Self> {
        let mut history = self.history.clone();
        if let Some(dir) = self.directory_buffer_mut() {
            if save_history {
                if let Some(n) = dir.focused_node() {
                    history = history.push(n.absolute_path().clone());
                }
            }

            dir.focus = 0;

            if save_history {
                if let Some(n) = self.clone().focused_node() {
                    self.history = history.push(n.absolute_path().clone())
                }
            }
        };
        Ok(self)
    }

    fn focus_last(mut self) -> Result<Self> {
        let mut history = self.history.clone();
        if let Some(dir) = self.directory_buffer_mut() {
            if let Some(n) = dir.focused_node() {
                history = history.push(n.absolute_path().clone());
            }

            dir.focus = dir.total.max(1) - 1;

            if let Some(n) = dir.focused_node() {
                self.history = history.push(n.absolute_path().clone());
            }
        };
        Ok(self)
    }

    fn focus_previous(mut self) -> Result<Self> {
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = if dir.focus == 0 {
                dir.total.max(1) - 1
            } else {
                dir.focus.max(1) - 1
            };
        };
        Ok(self)
    }

    fn focus_previous_by_relative_index(mut self, index: usize) -> Result<Self> {
        let mut history = self.history.clone();
        if let Some(dir) = self.directory_buffer_mut() {
            if let Some(n) = dir.focused_node() {
                history = history.push(n.absolute_path().clone());
            }

            dir.focus = dir.focus.max(index) - index;
            if let Some(n) = self.focused_node() {
                self.history = history.push(n.absolute_path().clone());
            }
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
            dir.focus = if (dir.focus + 1) == dir.total {
                0
            } else {
                dir.focus + 1
            }
        };
        Ok(self)
    }

    fn focus_next_by_relative_index(mut self, index: usize) -> Result<Self> {
        let mut history = self.history.clone();
        if let Some(dir) = self.directory_buffer_mut() {
            if let Some(n) = dir.focused_node() {
                history = history.push(n.absolute_path().clone());
            }

            dir.focus = (dir.focus + index).min(dir.total.max(1) - 1);
            if let Some(n) = self.focused_node() {
                self.history = history.push(n.absolute_path().clone());
            }
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

    fn follow_symlink(self) -> Result<Self> {
        if let Some(pth) = self
            .focused_node()
            .and_then(|n| n.symlink.to_owned().map(|s| s.absolute_path))
        {
            self.focus_path(&pth, true)
        } else {
            Ok(self)
        }
    }

    fn change_directory(mut self, dir: &str, save_history: bool) -> Result<Self> {
        let mut dir = PathBuf::from(dir);
        if dir.is_relative() {
            dir = PathBuf::from(self.pwd()).join(dir);
        }

        match env::set_current_dir(&dir) {
            Ok(()) => {
                let pwd = self.pwd().clone();
                let focus = self.focused_node().map(|n| n.relative_path().clone());
                self = self.add_last_focus(pwd, focus)?;
                self.pwd = dir.to_string_lossy().to_string();
                if save_history {
                    self.history = self.history.push(format!("{}/", self.pwd));
                }
                self.explore_pwd()
            }
            Err(e) => self.log_error(e.to_string()),
        }
    }

    fn enter(self) -> Result<Self> {
        self.focused_node()
            .map(|n| n.absolute_path.clone())
            .map(|p| self.clone().change_directory(&p, true))
            .unwrap_or(Ok(self))
    }

    fn back(self) -> Result<Self> {
        PathBuf::from(self.pwd())
            .parent()
            .map(|p| {
                self.clone()
                    .change_directory(&p.to_string_lossy().to_string(), true)
            })
            .unwrap_or(Ok(self))
    }

    fn last_visited_path(mut self) -> Result<Self> {
        self.history = self.history.visit_last();
        if let Some(target) = self.history.peek() {
            if target.ends_with('/') {
                target
                    .strip_suffix('/')
                    .map(|s| self.clone().change_directory(s, false))
                    .unwrap_or(Ok(self))
            } else {
                self.clone().focus_path(target, false)
            }
        } else {
            Ok(self)
        }
    }

    fn next_visited_path(mut self) -> Result<Self> {
        self.history = self.history.visit_next();
        if let Some(target) = self.history.peek() {
            if target.ends_with('/') {
                target
                    .strip_suffix('/')
                    .map(|s| self.clone().change_directory(s, false))
                    .unwrap_or(Ok(self))
            } else {
                self.clone().focus_path(target, false)
            }
        } else {
            Ok(self)
        }
    }

    fn buffer_input(mut self, input: &str) -> Result<Self> {
        if let Some(buf) = self.input_buffer.as_mut() {
            buf.push_str(input)
        } else {
            self.input_buffer = Some(input.to_owned());
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
        self.input_buffer = Some(string);
        self.logs_hidden = true;
        Ok(self)
    }

    fn remove_input_buffer_last_character(mut self) -> Result<Self> {
        if let Some(mut buf) = self.input_buffer {
            buf.pop();
            self.input_buffer = Some(buf);
            self.logs_hidden = true;
        };
        Ok(self)
    }

    fn remove_input_buffer_last_word(mut self) -> Result<Self> {
        if let Some(buf) = self.input_buffer {
            let buf = buf
                .chars()
                .into_iter()
                .rev()
                .skip_while(|c| !c.is_ascii_alphanumeric())
                .skip_while(|c| c.is_ascii_alphanumeric())
                .collect::<Vec<char>>()
                .into_iter()
                .rev()
                .collect::<String>();

            self.input_buffer = Some(buf);
            self.logs_hidden = true;
        };
        Ok(self)
    }

    fn reset_input_buffer(mut self) -> Result<Self> {
        self.input_buffer = None;
        Ok(self)
    }

    fn focus_by_index(mut self, index: usize) -> Result<Self> {
        let history = self.history.clone();
        if let Some(dir) = self.directory_buffer_mut() {
            dir.focus = index.min(dir.total.max(1) - 1);
            if let Some(n) = self.focused_node() {
                self.history = history.push(n.absolute_path().clone());
            }
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
                        history = history.push(n.absolute_path().clone());
                    }
                }
                dir_buf.focus = focus;
                if save_history {
                    if let Some(n) = dir_buf.focused_node() {
                        self.history = history.push(n.absolute_path().clone());
                    }
                }
                Ok(self)
            } else {
                self.log_error(format!("{} not found in $PWD", name))
            }
        } else {
            Ok(self)
        }
    }

    pub fn focus_path(self, path: &str, save_history: bool) -> Result<Self> {
        let mut pathbuf = PathBuf::from(path);
        if pathbuf.is_relative() {
            pathbuf = PathBuf::from(self.pwd()).join(pathbuf);
        }
        if let Some(parent) = pathbuf.parent() {
            if let Some(filename) = pathbuf.file_name() {
                self.change_directory(&parent.to_string_lossy().to_string(), false)?
                    .focus_by_file_name(&filename.to_string_lossy().to_string(), save_history)
            } else {
                self.log_error(format!("{} not found", path))
            }
        } else {
            self.log_error(format!("Cannot focus on {}", path))
        }
    }

    fn focus_path_from_input(self) -> Result<Self> {
        if let Some(p) = self.input_buffer() {
            self.focus_path(&p, true)
        } else {
            Ok(self)
        }
    }

    fn push_mode(mut self) -> Self {
        if self.mode() != self.config().modes().builtin().recover()
            && self
                .last_modes
                .last()
                .map(|m| m != self.mode())
                .unwrap_or(true)
        {
            self.last_modes.push(self.mode.clone())
        }
        self
    }

    fn pop_mode(mut self) -> Result<Self> {
        if let Some(mode) = self.last_modes.pop() {
            self.input_buffer = None;
            self.mode = mode;
        };
        Ok(self)
    }

    fn switch_mode(mut self, mode: &str) -> Result<Self> {
        if let Some(mode) = self.config().modes().clone().get(mode) {
            self = self.push_mode();
            self.input_buffer = None;
            self.mode = mode
                .to_owned()
                .sanitized(self.config().general().read_only());
            Ok(self)
        } else {
            self.log_error(format!("Mode not found: {}", mode))
        }
    }

    fn switch_mode_builtin(mut self, mode: &str) -> Result<Self> {
        if let Some(mode) = self.config().modes().clone().get_builtin(mode) {
            self = self.push_mode();
            self.input_buffer = None;
            self.mode = mode
                .to_owned()
                .sanitized(self.config().general().read_only());
            Ok(self)
        } else {
            self.log_error(format!("Builtin mode not found: {}", mode))
        }
    }

    fn switch_mode_custom(mut self, mode: &str) -> Result<Self> {
        if let Some(mode) = self.config().modes().clone().get_custom(mode) {
            self = self.push_mode();
            self.input_buffer = None;
            self.mode = mode
                .to_owned()
                .sanitized(self.config().general().read_only());
            Ok(self)
        } else {
            self.log_error(format!("Custom mode not found: {}", mode))
        }
    }

    fn switch_layout(mut self, layout: &str) -> Result<Self> {
        if let Some(l) = self.config().layouts().get(layout) {
            self.layout = l.to_owned();
            Ok(self)
        } else {
            self.log_error(format!("Layout not found: {}", layout))
        }
    }

    fn switch_layout_builtin(mut self, layout: &str) -> Result<Self> {
        if let Some(l) = self.config().layouts().get_builtin(layout) {
            self.layout = l.to_owned();
            Ok(self)
        } else {
            self.log_error(format!("Builtin layout not found: {}", layout))
        }
    }

    fn switch_layout_custom(mut self, layout: &str) -> Result<Self> {
        if let Some(l) = self.config().layouts().get_custom(layout) {
            self.layout = l.to_owned();
            Ok(self)
        } else {
            self.log_error(format!("Custom layout not found: {}", layout))
        }
    }

    fn call(mut self, command: Command) -> Result<Self> {
        self.logs_hidden = true;
        self.msg_out.push_back(MsgOut::Call(command));
        Ok(self)
    }

    fn call_silently(mut self, command: Command) -> Result<Self> {
        self.logs_hidden = true;
        self.msg_out.push_back(MsgOut::CallSilently(command));
        Ok(self)
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

    pub fn set_directory(mut self, dir: DirectoryBuffer) -> Result<Self> {
        self = self.add_last_focus(
            dir.parent.clone(),
            dir.focused_node().map(|n| n.relative_path().clone()),
        )?;
        if dir.parent == self.pwd {
            self.directory_buffer = Some(dir);
        }
        Ok(self)
    }

    pub fn add_last_focus(mut self, parent: String, focused_path: Option<String>) -> Result<Self> {
        self.last_focus.insert(parent, focused_path);
        Ok(self)
    }

    fn select(mut self) -> Result<Self> {
        if let Some(n) = self.focused_node().map(|n| n.to_owned()) {
            self.selection.insert(n);
        }
        Ok(self)
    }

    fn select_path(mut self, path: String) -> Result<Self> {
        let mut path = PathBuf::from(path);
        if path.is_relative() {
            path = PathBuf::from(self.pwd()).join(path);
        }
        let parent = path.parent().map(|p| p.to_string_lossy().to_string());
        let filename = path.file_name().map(|p| p.to_string_lossy().to_string());
        if let (Some(p), Some(n)) = (parent, filename) {
            self.selection.insert(Node::new(p, n));
        }
        Ok(self)
    }

    fn select_all(mut self) -> Result<Self> {
        if let Some(d) = self.directory_buffer() {
            d.nodes.clone().into_iter().for_each(|n| {
                self.selection.insert(n);
            });
        };

        Ok(self)
    }

    fn un_select(mut self) -> Result<Self> {
        if let Some(n) = self.focused_node().map(|n| n.to_owned()) {
            self.selection.retain(|s| s != &n);
        }
        Ok(self)
    }

    fn un_select_path(mut self, path: String) -> Result<Self> {
        let mut pathbuf = PathBuf::from(path);
        if pathbuf.is_relative() {
            pathbuf = PathBuf::from(self.pwd()).join(pathbuf);
        }
        self.selection
            .retain(|n| PathBuf::from(&n.absolute_path) != pathbuf);
        Ok(self)
    }

    fn un_select_all(mut self) -> Result<Self> {
        if let Some(d) = self.directory_buffer() {
            d.nodes.clone().into_iter().for_each(|n| {
                self.selection.retain(|s| s != &n);
            });
        };

        Ok(self)
    }

    fn toggle_selection(self) -> Result<Self> {
        if let Some(p) = self.focused_node().map(|n| n.absolute_path().clone()) {
            self.toggle_selection_by_path(p)
        } else {
            Ok(self)
        }
    }

    fn toggle_select_all(self) -> Result<Self> {
        if let Some(d) = self.directory_buffer() {
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
        let mut pathbuf = PathBuf::from(&path);
        if pathbuf.is_relative() {
            pathbuf = PathBuf::from(self.pwd()).join(pathbuf);
        }
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
        self.selection.clear();
        Ok(self)
    }

    fn add_node_filter(mut self, filter: NodeFilterApplicable) -> Result<Self> {
        self.explorer_config.filters.replace(filter);
        Ok(self)
    }

    fn add_node_filter_from_input(mut self, filter: NodeFilter) -> Result<Self> {
        if let Some(input) = self.input_buffer() {
            self.explorer_config
                .filters
                .insert(NodeFilterApplicable::new(filter, input));
        };
        Ok(self)
    }

    fn remove_node_filter(mut self, filter: NodeFilterApplicable) -> Result<Self> {
        self.explorer_config.filters.retain(|f| f != &filter);
        Ok(self)
    }

    fn remove_node_filter_from_input(mut self, filter: NodeFilter) -> Result<Self> {
        if let Some(input) = self.input_buffer() {
            let nfa = NodeFilterApplicable::new(filter, input);
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

        if !self.config().general().show_hidden() {
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
            .config()
            .general()
            .initial_sorting()
            .to_owned()
            .unwrap_or_default();
        Ok(self)
    }

    fn clear_node_sorters(mut self) -> Result<Self> {
        self.explorer_config.sorters.clear();
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

    /// Get a reference to the app's pwd.
    pub fn pwd(&self) -> &String {
        &self.pwd
    }

    /// Get a reference to the app's current directory buffer.
    pub fn directory_buffer(&self) -> Option<&DirectoryBuffer> {
        self.directory_buffer.as_ref()
    }

    /// Get a reference to the app's config.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get a reference to the app's selection.
    pub fn selection(&self) -> &IndexSet<Node> {
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
        format!("{}\n", &self.mode.name())
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

    fn refresh_selection(mut self) -> Result<Self> {
        // Should be able to select broken symlink
        self.selection
            .retain(|n| PathBuf::from(&n.absolute_path).symlink_metadata().is_ok());
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

    pub fn logs_str(&self) -> String {
        self.logs()
            .iter()
            .map(|l| format!("{}\n", l))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn global_help_menu_str(&self) -> String {
        let builtin = self.config().modes().builtin();
        let custom = self.config().modes().custom();

        [
            builtin.default(),
            builtin.recover(),
            builtin.filter(),
            builtin.number(),
            builtin.go_to(),
            builtin.search(),
            builtin.selection_ops(),
            builtin.action(),
            builtin.create(),
            builtin.create_file(),
            builtin.create_directory(),
            builtin.rename(),
            builtin.delete(),
            builtin.sort(),
            builtin.filter(),
            builtin.relative_path_does_contain(),
            builtin.relative_path_does_not_contain(),
            builtin.switch_layout(),
        ]
        .iter().map(|m| (m.name(), m.to_owned()))
        .chain(custom.iter())
        .map(|(name, mode)| {
            let help = mode
                .help_menu()
                .iter()
                .map(|l| match l {
                    HelpMenuLine::Paragraph(p) => format!("\t{}\n", p),
                    HelpMenuLine::KeyMap(k, remaps, h) => {
                        let remaps = remaps.join(", ");
                        format!(" {:15} | {:25} | {}\n", k, remaps, h)
                    }
                })
                .collect::<Vec<String>>()
                .join("");

            format!(
                "### {}\n\n key             | remaps                    | action\n --------------- | ------------------------- | ------\n{}\n",
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

    /// Get a reference to the app's history.
    pub fn history(&self) -> &History {
        &self.history
    }

    pub fn history_str(&self) -> String {
        self.history
            .paths
            .iter()
            .map(|p| format!("{}\n", &p))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn write_pipes(&self) -> Result<()> {
        fs::create_dir_all(self.pipe().path())?;
        fs::write(&self.pipe().msg_in, "")?;

        let selection_str = self.selection_str();
        fs::write(&self.pipe().selection_out, selection_str)?;

        let history_str = self.history_str();
        fs::write(&self.pipe().history_out, history_str)?;

        let directory_nodes_str = self.directory_nodes_str();
        fs::write(&self.pipe().directory_nodes_out, directory_nodes_str)?;

        let logs_str = self.logs_str();
        fs::write(&self.pipe().logs_out, logs_str)?;

        let result_str = self.result_str();
        fs::write(&self.pipe().result_out, result_str)?;

        let global_help_menu_str = self.global_help_menu_str();
        fs::write(&self.pipe().global_help_menu_out, global_help_menu_str)?;

        Ok(())
    }

    pub fn cleanup_pipes(&self) -> Result<()> {
        while !fs::read_to_string(self.pipe().msg_in())?.is_empty() {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        fs::remove_file(self.pipe().msg_in())?;
        fs::remove_file(self.pipe().selection_out())?;
        fs::remove_file(self.pipe().result_out())?;
        fs::remove_file(self.pipe().directory_nodes_out())?;
        fs::remove_file(self.pipe().global_help_menu_out())?;
        fs::remove_file(self.pipe().logs_out())?;
        fs::remove_file(self.pipe().history_out())?;

        fs::remove_dir(self.pipe().path())?;
        Ok(())
    }

    /// Get a reference to the app's layout.
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    /// Get a reference to the app's logs hidden.
    pub fn logs_hidden(&self) -> bool {
        self.logs_hidden
    }

    pub fn to_lua_arg(&self) -> CallLuaArg {
        CallLuaArg {
            version: self.version.clone(),
            pwd: self.pwd.clone(),
            focused_node: self.focused_node().cloned(),
            directory_buffer: self.directory_buffer().cloned(),
            selection: self.selection.clone(),
            mode: self.mode.clone(),
            layout: self.layout.clone(),
            input_buffer: self.input_buffer.clone(),
            pid: self.pid,
            session_path: self.session_path.clone(),
            explorer_config: self.explorer_config.clone(),
            history: self.history.clone(),
            last_modes: self.last_modes.clone(),
        }
    }
}

/// Create a new runner object passing the default arguments
pub fn runner() -> Result<Runner> {
    Runner::new()
}

/// Create a new runner object passing the given arguments
pub fn from_cli(cli: Cli) -> Result<Runner> {
    Runner::from_cli(cli)
}
