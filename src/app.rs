use crate::config::Config;
use crate::config::Mode;
use crate::input::Key;
use crate::ui::Layout;
use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use indexmap::set::IndexSet;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub const TEMPLATE_TABLE_ROW: &str = "TEMPLATE_TABLE_ROW";
pub const UNSUPPORTED_STR: &str = "???";
pub const UPGRADE_GUIDE_LINK: &str = "https://github.com/sayanarijit/xplr/wiki/Upgrade-Guide";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipe {
    msg_in: String,
    focus_out: String,
    selection_out: String,
    result_out: String,
    directory_nodes_out: String,
    global_help_menu_out: String,
    logs_out: String,
    history_out: String,
}

impl Pipe {
    fn from_session_path(path: &str) -> Result<Self> {
        let pipesdir = PathBuf::from(path).join("pipe");

        fs::create_dir_all(&pipesdir).unwrap();

        let msg_in = pipesdir.join("msg_in").to_string_lossy().to_string();

        let focus_out = pipesdir.join("focus_out").to_string_lossy().to_string();

        let selection_out = pipesdir.join("selection_out").to_string_lossy().to_string();

        let result_out = pipesdir.join("result_out").to_string_lossy().to_string();

        let directory_nodes_out = pipesdir
            .join("directory_nodes_out")
            .to_string_lossy()
            .to_string();

        let global_help_menu_out = pipesdir
            .join("global_help_menu_out")
            .to_string_lossy()
            .to_string();

        let logs_out = pipesdir.join("logs_out").to_string_lossy().to_string();

        let history_out = pipesdir.join("history_out").to_string_lossy().to_string();

        fs::write(&msg_in, "")?;
        fs::write(&focus_out, "")?;
        fs::write(&selection_out, "")?;
        fs::write(&directory_nodes_out, "")?;
        fs::write(&global_help_menu_out, "")?;
        fs::write(&result_out, "")?;
        fs::write(&logs_out, "")?;
        fs::write(&history_out, "")?;

        Ok(Self {
            msg_in,
            focus_out,
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
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ResolvedNode {
    absolute_path: String,
    extension: String,
    is_dir: bool,
    is_file: bool,
    is_readonly: bool,
    mime_essence: String,
    size: u64,
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

        Self {
            absolute_path: path.to_string_lossy().to_string(),
            extension,
            is_dir,
            is_file,
            is_readonly,
            mime_essence,
            size,
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
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Node {
    parent: String,
    relative_path: String,
    absolute_path: String,
    extension: String,
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
    is_broken: bool,
    is_readonly: bool,
    mime_essence: String,
    size: u64,
    canonical: Option<ResolvedNode>,
    symlink: Option<ResolvedNode>,
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

        let (is_symlink, is_dir, is_file, is_readonly, size) = path
            .symlink_metadata()
            .map(|m| {
                (
                    m.file_type().is_symlink(),
                    m.is_dir(),
                    m.is_file(),
                    m.permissions().readonly(),
                    m.len(),
                )
            })
            .unwrap_or((false, false, false, false, 0));

        let mime_essence = mime_guess::from_path(&path)
            .first()
            .map(|m| m.essence_str().to_string())
            .unwrap_or_default();

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
    parent: String,
    nodes: Vec<Node>,
    total: usize,
    focus: usize,
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
    AddDirectory(String, DirectoryBuffer),
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
    sorter: NodeSorter,
    #[serde(default)]
    reverse: bool,
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
    filter: NodeFilter,
    input: String,
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
    filters: IndexSet<NodeFilterApplicable>,
    sorters: IndexSet<NodeSorterApplicable>,
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
    /// Once exploration is done, it will auto `Refresh` the state.
    ExplorePwd,

    /// Explore the present working directory and register the filtered nodes asynchronously.
    /// This operation happens asynchronously. That means, the xplr directory buffers won't be updated
    /// immediately. Hence, it needs to be used with care and probably with special checks in place.
    /// To explore `$PWD` synchronously, use `ExplorePwd` instead.
    /// Once exploration is done, it will auto `Refresh` the state.
    ExplorePwdAsync,

    /// Explore the present working directory along with its parents and register the filtered nodes.
    /// This operation happens asynchronously. That means, the xplr directory buffers won't be updated
    /// immediately. Hence, it needs to be used with care and probably with special checks in place.
    /// To explore just the `$PWD` synchronously, use `ExplorePwd` instead.
    /// Once exploration is done, it will auto `Refresh` the state.
    ExploreParentsAsync,

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
    /// This will reset the input buffer and call `Refresh` automatically.
    ///
    /// > **NOTE:** To be specific about which mode to switch to, use `SwitchModeBuiltin` or
    /// `SwitchModeCustom` instead.
    ///
    /// **Example:** `SwitchMode: default`
    SwitchMode(String),

    /// Switch to a builtin mode.
    /// This will reset the input buffer and call `Refresh` automatically.
    ///
    /// **Example:** `SwitchModeBuiltin: default`
    SwitchModeBuiltin(String),

    /// Switch to a custom mode.
    /// This will reset the input buffer and call `Refresh` automatically.
    ///
    /// **Example:** `SwitchModeCustom: my_custom_mode`
    SwitchModeCustom(String),

    /// Switch layout.
    /// This will call `Refresh` automatically.
    ///
    /// > **NOTE:** To be specific about which layout to switch to, use `SwitchLayoutBuiltin` or
    /// `SwitchLayoutCustom` instead.
    ///
    /// **Example:** `SwitchLayout: default`
    SwitchLayout(String),

    /// Switch to a builtin layout.
    /// This will call `Refresh` automatically.
    ///
    /// **Example:** `SwitchLayoutBuiltin: default`
    SwitchLayoutBuiltin(String),

    /// Switch to a custom layout.
    /// This will call `Refresh` automatically.
    ///
    /// **Example:** `SwitchLayoutCustom: my_custom_layout`
    SwitchLayoutCustom(String),

    /// Call a shell command with the given arguments.
    /// Note that the arguments will be shell-escaped.
    /// So to read the variables, the `-c` option of the shell
    /// can be used.
    /// You may need to pass `Refresh` or `Explore` depening on the expectation.
    ///
    /// **Example:** `Call: {command: bash, args: ["-c", "read -p test"]}`
    Call(Command),

    /// Like `Call` but without the flicker. The stdin, stdout
    /// stderr will be piped to null. So it's non-interactive.
    ///
    /// **Example:** `CallSilently: {command: tput, args: ["bell"]}`
    CallSilently(Command),

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

    /// Log information message.
    ///
    /// **Example:** `LogInfo: launching satellite`
    LogInfo(String),

    /// Log a success message.
    ///
    /// **Example:** `LogSuccess: satellite reached destination`.
    LogSuccess(String),

    /// Log an error message.
    ///
    /// **Example:** `LogError: satellite crashed`
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
    command: String,

    #[serde(default)]
    args: Vec<String>,
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
    ExplorePwd,
    ExplorePwdAsync,
    ExploreParentsAsync,
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
    level: LogLevel,
    message: String,
    created_at: DateTime<Local>,
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
    fn push(mut self, path: String) -> Self {
        self.paths = self.paths.into_iter().take(self.loc + 1).collect();
        self.paths.push(path);
        self.loc = self.paths.len().max(1) - 1;
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
pub struct App {
    version: String,
    config: Config,
    pwd: String,
    directory_buffers: HashMap<String, DirectoryBuffer>,
    selection: IndexSet<Node>,
    msg_out: VecDeque<MsgOut>,
    mode: Mode,
    layout: Layout,
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
        let default_config_version = default_config.version().clone();

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
                config.version(),
                default_config_version,
                UPGRADE_GUIDE_LINK,
            )
        };

        let mode = match config.modes().get(
            &config
                .general()
                .initial_mode()
                .to_owned()
                .unwrap_or_else(|| "default".into()),
        ) {
            Some(m) => m
                .clone()
                .sanitized(config.general().read_only().unwrap_or_default()),
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
        let session_path = dirs::runtime_dir()
            .unwrap_or_else(|| "/tmp".into())
            .join("xplr")
            .join("session")
            .join(&pid.to_string())
            .to_string_lossy()
            .to_string();

        let mut explorer_config = ExplorerConfig::default();
        if !config.general().show_hidden().unwrap_or_default() {
            explorer_config.filters.replace(NodeFilterApplicable::new(
                NodeFilter::RelativePathDoesNotStartWith,
                ".".into(),
            ));
        }

        if let Some(sorters) = &config.general().initial_sorting() {
            explorer_config.sorters = sorters.clone();
        };

        let pwd = pwd.to_string_lossy().to_string();

        let mut app = Self {
            version: Config::default().version().clone(),
            config: config.clone(),
            pwd: pwd.clone(),
            directory_buffers: Default::default(),
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
            history: Default::default(),
        }
        .change_directory(&pwd)?;

        if let Some(notif) = config.upgrade_notification()? {
            let notif = format!(
                "{}. To stop seeing this log, update your config version from {} to {}.",
                &notif,
                config.version(),
                app.version()
            );
            app = app.enqueue(Task::new(
                MsgIn::External(ExternalMsg::LogInfo(notif)),
                None,
            ));
        }

        app.write_pipes()?;
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
        if self.config().general().read_only().unwrap_or_default() && !msg.is_read_only() {
            self.log_error("Cannot call shell command in read-only mode.".into())
        } else {
            match msg {
                ExternalMsg::ExplorePwd => self.explore_pwd(),
                ExternalMsg::ExploreParentsAsync => self.explore_parents_async(),
                ExternalMsg::ExplorePwdAsync => self.explore_pwd_async(),
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
                ExternalMsg::SwitchLayout(mode) => self.switch_layout(&mode),
                ExternalMsg::SwitchLayoutBuiltin(mode) => self.switch_layout_builtin(&mode),
                ExternalMsg::SwitchLayoutCustom(mode) => self.switch_layout_custom(&mode),
                ExternalMsg::Call(cmd) => self.call(cmd),
                ExternalMsg::CallSilently(cmd) => self.call_silently(cmd),
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
                ExternalMsg::LogInfo(l) => self.log_info(l),
                ExternalMsg::LogSuccess(l) => self.log_success(l),
                ExternalMsg::LogError(l) => self.log_error(l),
                ExternalMsg::Quit => self.quit(),
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
            .remaps()
            .get(&key_str)
            .and_then(|k| kb.on_key().get(k))
            .or_else(|| kb.on_key().get(&key_str))
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
            .unwrap_or_else(|| default.map(|a| a.messages().clone()).unwrap_or_default());

        for msg in msgs {
            self = self.enqueue(Task::new(MsgIn::External(msg), Some(key)));
        }

        Ok(self)
    }

    fn explore_pwd(mut self) -> Result<Self> {
        self.msg_out.push_back(MsgOut::ExplorePwd);
        Ok(self)
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

    fn follow_symlink(self) -> Result<Self> {
        if let Some(pth) = self
            .focused_node()
            .and_then(|n| n.symlink.to_owned().map(|s| s.absolute_path))
        {
            self.focus_path(&pth)
        } else {
            Ok(self)
        }
    }

    fn change_directory(mut self, dir: &str) -> Result<Self> {
        match env::set_current_dir(dir) {
            Ok(()) => {
                self.pwd = dir.to_owned();
                self.history = self.history.push(self.pwd.clone());
                self.explore_pwd_async()
            }
            Err(e) => self.log_error(e.to_string()),
        }
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
        env::set_current_dir(&self.pwd)?;
        self.explore_pwd_async()
    }

    fn next_visited_path(mut self) -> Result<Self> {
        self.history = self.history.visit_next();
        self.pwd = self
            .history
            .peek()
            .map(|p| p.to_owned())
            .unwrap_or(self.pwd);
        env::set_current_dir(&self.pwd)?;
        self.explore_pwd_async()
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

    fn remove_input_buffer_last_character(mut self) -> Result<Self> {
        if let Some(mut buf) = self.input_buffer {
            buf.pop();
            self.input_buffer = Some(buf);
        };
        self.msg_out.push_back(MsgOut::Refresh);
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
        };
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
            } else {
                self = self.log_error(format!("{} not found in $PWD", name))?;
            }
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
                self.log_error(format!("{} not found", path))
            }
        } else {
            self.log_error(format!("Cannot focus on {}", path))
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
        if let Some(mode) = self.config().modes().clone().get(mode) {
            self.input_buffer = None;
            self.mode = mode
                .to_owned()
                .sanitized(self.config().general().read_only().unwrap_or_default());
            self.msg_out.push_back(MsgOut::Refresh);
            Ok(self)
        } else {
            self.log_error(format!("Mode not found: {}", mode))
        }
    }

    fn switch_mode_builtin(mut self, mode: &str) -> Result<Self> {
        if let Some(mode) = self.config().modes().clone().get_builtin(mode) {
            self.input_buffer = None;
            self.mode = mode
                .to_owned()
                .sanitized(self.config().general().read_only().unwrap_or_default());
            self.msg_out.push_back(MsgOut::Refresh);
            Ok(self)
        } else {
            self.log_error(format!("Builtin mode not found: {}", mode))
        }
    }

    fn switch_mode_custom(mut self, mode: &str) -> Result<Self> {
        if let Some(mode) = self.config().modes().clone().get_custom(mode) {
            self.input_buffer = None;
            self.mode = mode
                .to_owned()
                .sanitized(self.config().general().read_only().unwrap_or_default());
            self.msg_out.push_back(MsgOut::Refresh);
            Ok(self)
        } else {
            self.log_error(format!("Custom mode not found: {}", mode))
        }
    }

    fn switch_layout(mut self, layout: &str) -> Result<Self> {
        if let Some(l) = self.config().layouts().get(layout) {
            self.layout = l.to_owned();
            self.msg_out.push_back(MsgOut::Refresh);
            Ok(self)
        } else {
            self.log_error(format!("Layout not found: {}", layout))
        }
    }

    fn switch_layout_builtin(mut self, layout: &str) -> Result<Self> {
        if let Some(l) = self.config().layouts().get_builtin(layout) {
            self.layout = l.to_owned();
            self.msg_out.push_back(MsgOut::Refresh);
            Ok(self)
        } else {
            self.log_error(format!("Builtin layout not found: {}", layout))
        }
    }

    fn switch_layout_custom(mut self, layout: &str) -> Result<Self> {
        if let Some(l) = self.config().layouts().get_custom(layout) {
            self.layout = l.to_owned();
            self.msg_out.push_back(MsgOut::Refresh);
            Ok(self)
        } else {
            self.log_error(format!("Custom layout not found: {}", layout))
        }
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

    pub fn add_directory(mut self, parent: String, dir: DirectoryBuffer) -> Result<Self> {
        self.directory_buffers.insert(parent, dir);
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn select(mut self) -> Result<Self> {
        if let Some(n) = self.focused_node().map(|n| n.to_owned()) {
            self.selection.insert(n);
            self.msg_out.push_back(MsgOut::Refresh);
        }
        Ok(self)
    }

    fn select_path(mut self, path: String) -> Result<Self> {
        let path = PathBuf::from(path);
        let parent = path.parent().map(|p| p.to_string_lossy().to_string());
        let filename = path.file_name().map(|p| p.to_string_lossy().to_string());
        if let (Some(p), Some(n)) = (parent, filename) {
            self.selection.insert(Node::new(p, n));
            self.msg_out.push_back(MsgOut::Refresh);
        };
        Ok(self)
    }

    fn select_all(mut self) -> Result<Self> {
        if let Some(d) = self.directory_buffer() {
            d.nodes.clone().into_iter().for_each(|n| {
                self.selection.insert(n);
            });
            self.msg_out.push_back(MsgOut::Refresh);
        };

        Ok(self)
    }

    fn un_select(mut self) -> Result<Self> {
        if let Some(n) = self.focused_node().map(|n| n.to_owned()) {
            self.selection.retain(|s| s != &n);
            self.msg_out.push_back(MsgOut::Refresh);
        }
        Ok(self)
    }

    fn un_select_path(mut self, path: String) -> Result<Self> {
        self.selection.retain(|n| n.absolute_path != path);
        self.msg_out.push_back(MsgOut::Refresh);
        Ok(self)
    }

    fn un_select_all(mut self) -> Result<Self> {
        if let Some(d) = self.directory_buffer() {
            d.nodes.clone().into_iter().for_each(|n| {
                self.selection.retain(|s| s != &n);
            });
            self.msg_out.push_back(MsgOut::Refresh);
        };

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
        if self.selection.iter().any(|n| n.absolute_path == path) {
            self.select_path(path)
        } else {
            self.un_select_path(path)
        }
    }

    fn clear_selection(mut self) -> Result<Self> {
        self.selection.clear();
        self.msg_out.push_back(MsgOut::Refresh);
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

        if !self.config().general().show_hidden().unwrap_or_default() {
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

    pub fn log_info(mut self, message: String) -> Result<Self> {
        self.logs.push(Log::new(LogLevel::Info, message));
        Ok(self)
    }

    pub fn log_success(mut self, message: String) -> Result<Self> {
        self.logs.push(Log::new(LogLevel::Success, message));
        Ok(self)
    }

    pub fn log_error(mut self, message: String) -> Result<Self> {
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

    fn refresh_selection(mut self) -> Result<Self> {
        self.selection
            .retain(|n| PathBuf::from(&n.absolute_path).exists());
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
                    HelpMenuLine::KeyMap(k, h) => {
                        let remaps = self
                            .mode()
                            .key_bindings()
                            .remaps()
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
        // TODO optimize and test

        let focused_node_str = self.focused_node_str();
        // if last_app
        //     .map(|a| a.focused_node_str() != focused_node_str)
        //     .unwrap_or(true)
        // {
        fs::write(&self.pipe().focus_out, focused_node_str)?;
        // };

        let selection_str = self.selection_str();
        // if last_app
        //     .map(|a| a.selection_str() != selection_str)
        //     .unwrap_or(true)
        // {
        fs::write(&self.pipe().selection_out, selection_str)?;
        // };

        let history_str = self.history_str();
        // if last_app
        //     .map(|a| a.history_str() != history_str)
        //     .unwrap_or(true)
        // {
        fs::write(&self.pipe().history_out, history_str)?;
        // };

        let directory_nodes_str = self.directory_nodes_str();
        // if last_app
        //     .map(|a| a.directory_nodes_str() != directory_nodes_str)
        //     .unwrap_or(true)
        // {
        fs::write(&self.pipe().directory_nodes_out, directory_nodes_str)?;
        // };

        // if last_app
        //     .map(|a| a.logs().len() != self.logs().len())
        //     .unwrap_or(true)
        // {
        // let new_logs = self
        //     .logs()
        //     .iter()
        //     .skip(last_app.map(|a| a.logs().len()).unwrap_or(0))
        //     .map(|l| format!("{}\n", l))
        //     .collect::<Vec<String>>()
        //     .join("");

        // let mut file = fs::OpenOptions::new()
        //     .append(true)
        //     .open(&self.pipe().logs_out)?;
        // file.write_all(new_logs.as_bytes())?;
        // };
        let logs_str = self.logs_str();
        fs::write(&self.pipe().logs_out, logs_str)?;

        let result_str = self.result_str();
        // if last_app
        //     .map(|a| a.result_str() != result_str)
        //     .unwrap_or(true)
        // {
        fs::write(&self.pipe().result_out, result_str)?;
        // };
        let global_help_menu_str = self.global_help_menu_str();
        // if last_app
        //     .map(|a| a.global_help_menu_str() != global_help_menu_str)
        //     .unwrap_or(true)
        // {
        fs::write(&self.pipe().global_help_menu_out, global_help_menu_str)?;
        // };
        Ok(())
    }

    /// Get a reference to the app's layout.
    pub fn layout(&self) -> &Layout {
        &self.layout
    }
}
