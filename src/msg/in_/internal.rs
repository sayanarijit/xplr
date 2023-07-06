use crate::app::DirectoryBuffer;
use crate::input::Key;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum InternalMsg {
    AddLastFocus(String, Option<String>),
    SetDirectory(DirectoryBuffer),
    HandleKey(Key),
    RefreshSelection,
}
