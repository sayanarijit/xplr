// Stolen from https://github.com/Peltoche/lsd/blob/master/src/meta/owner.rs

use serde::{Deserialize, Serialize};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash, Default)]
pub struct Owner {
    user: String,
    group: String,
}

#[cfg(unix)]
impl<'a> From<&'a Metadata> for Owner {
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        use users::{get_group_by_gid, get_user_by_uid};

        let user = match get_user_by_uid(meta.uid()) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => meta.uid().to_string(),
        };

        let group = match get_group_by_gid(meta.gid()) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => meta.gid().to_string(),
        };

        Self { user, group }
    }
}
