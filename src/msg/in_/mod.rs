pub mod external;
pub mod internal;

pub use external::ExternalMsg;
pub use internal::InternalMsg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MsgIn {
    Internal(internal::InternalMsg),
    External(external::ExternalMsg),
}
