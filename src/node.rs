use crate::permissions::Permissions;
use humansize::{format_size, DECIMAL};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::os::unix::prelude::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

fn to_human_size(size: u64) -> String {
    format_size(size, DECIMAL)
}

fn mime_essence(
    path: &Path,
    is_dir: bool,
    extension: &str,
    is_executable: bool,
) -> String {
    if is_dir {
        String::from("inode/directory")
    } else if extension.is_empty() && is_executable {
        String::from("application/x-executable")
    } else {
        mime_guess::from_path(path)
            .first()
            .map(|m| m.essence_str().to_string())
            .unwrap_or_default()
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
    pub created: Option<u128>,
    pub last_modified: Option<u128>,
    pub uid: u32,
    pub gid: u32,
}

impl ResolvedNode {
    pub fn from(path: PathBuf) -> Self {
        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();

        let (
            is_dir,
            is_file,
            is_readonly,
            size,
            permissions,
            created,
            last_modified,
            uid,
            gid,
        ) = path
            .metadata()
            .map(|m| {
                (
                    m.is_dir(),
                    m.is_file(),
                    m.permissions().readonly(),
                    m.len(),
                    Permissions::from(&m),
                    m.created()
                        .ok()
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_nanos()),
                    m.modified()
                        .ok()
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_nanos()),
                    m.uid(),
                    m.gid(),
                )
            })
            .unwrap_or((false, false, false, 0, Default::default(), None, None, 0, 0));

        let is_executable = permissions.user_execute
            || permissions.group_execute
            || permissions.other_execute;
        let mime_essence = mime_essence(&path, is_dir, &extension, is_executable);
        let human_size = to_human_size(size);

        Self {
            absolute_path: path.to_string_lossy().to_string(),
            extension,
            is_dir,
            is_file,
            is_readonly,
            mime_essence,
            size,
            human_size,
            created,
            last_modified,
            uid,
            gid,
        }
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
    pub created: Option<u128>,
    pub last_modified: Option<u128>,
    pub uid: u32,
    pub gid: u32,

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

        let (
            is_symlink,
            is_dir,
            is_file,
            is_readonly,
            size,
            permissions,
            created,
            last_modified,
            uid,
            gid,
        ) = path
            .symlink_metadata()
            .map(|m| {
                (
                    m.file_type().is_symlink(),
                    m.is_dir(),
                    m.is_file(),
                    m.permissions().readonly(),
                    m.len(),
                    Permissions::from(&m),
                    m.created()
                        .ok()
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_nanos()),
                    m.modified()
                        .ok()
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_nanos()),
                    m.uid(),
                    m.gid(),
                )
            })
            .unwrap_or_else(|_| {
                (
                    false,
                    false,
                    false,
                    false,
                    0,
                    Permissions::default(),
                    None,
                    None,
                    0,
                    0,
                )
            });

        let is_executable = permissions.user_execute
            || permissions.group_execute
            || permissions.other_execute;

        let mime_essence = mime_essence(&path, is_dir, &extension, is_executable);
        let human_size = to_human_size(size);

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
            created,
            last_modified,
            uid,
            gid,
            canonical: maybe_canonical_meta.clone(),
            symlink: if is_symlink {
                maybe_canonical_meta
            } else {
                None
            },
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
