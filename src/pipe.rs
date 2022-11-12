use crate::app::ExternalMsg;
use crate::yaml;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

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
}

pub fn read_all(pipe: &str, delimiter: char) -> Result<Vec<ExternalMsg>> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(pipe)?;

    let mut in_str = String::new();
    file.read_to_string(&mut in_str)?;
    file.set_len(0)?;

    if !in_str.is_empty() {
        let mut msgs = vec![];
        for msg in in_str.trim_matches(delimiter).split(delimiter) {
            if !msg.is_empty() {
                msgs.push(yaml::from_str(msg)?);
            }
        }
        Ok(msgs)
    } else {
        Ok(vec![])
    }
}
