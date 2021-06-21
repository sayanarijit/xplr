use crate::app::ExternalMsg;
use anyhow::Result;
use std::fs;
use std::io::prelude::*;

pub fn read_all(pipe: &str) -> Result<Vec<ExternalMsg>> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(&pipe)?;

    let mut in_str = String::new();
    file.read_to_string(&mut in_str)?;
    file.set_len(0)?;

    if !in_str.is_empty() {
        let mut msgs = vec![];
        for msg in in_str.lines().map(|s| serde_yaml::from_str(s.trim())) {
            msgs.push(msg?);
        }
        Ok(msgs)
    } else {
        Ok(vec![])
    }
}
