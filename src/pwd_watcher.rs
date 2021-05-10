use crate::app::Task;
use crate::app::{ExternalMsg, MsgIn};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

pub fn keep_watching(
    pwd: &str,
    tx_msg_in: Sender<Task>,
    rx_pwd_watcher: Receiver<String>,
) -> Result<()> {
    let mut pwd = PathBuf::from(pwd);
    let mut last_modified = pwd
        .metadata()
        .and_then(|m| m.modified())
        .unwrap_or_else(|_| SystemTime::now());

    thread::spawn(move || loop {
        if let Ok(new_pwd) = rx_pwd_watcher.try_recv() {
            pwd = PathBuf::from(new_pwd);
        } else {
            let modified = pwd
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or_else(|_| SystemTime::now());

            if modified != last_modified {
                let msg = MsgIn::External(ExternalMsg::ExplorePwdAsync);
                tx_msg_in.send(Task::new(msg, None)).unwrap_or_default();
                last_modified = modified;
            } else {
                thread::sleep(Duration::from_millis(1000));
            };
        }
    });
    Ok(())
}
