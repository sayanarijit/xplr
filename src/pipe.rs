use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

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
    pub fn from_session_path(path: &str) -> Result<Self> {
        let path = PathBuf::from(path).join("pipe");

        let msg_in = path.join("msg_in").to_string_lossy().to_string();

        let selection_out =
            path.join("selection_out").to_string_lossy().to_string();

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

        let history_out =
            path.join("history_out").to_string_lossy().to_string();

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
}
