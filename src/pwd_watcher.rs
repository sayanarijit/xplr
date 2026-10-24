use crate::app::Task;
use crate::app::{ExternalMsg, MsgIn};
use anyhow::{Error, Result};
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub(crate) struct PwdWatcher {
    task_sender: Sender<Task>,
    pwd_updater: Option<Sender<String>>,
    stopper: Option<Sender<bool>>,
}

impl PwdWatcher {
    pub(crate) fn new(task_sender: Sender<Task>) -> Self {
        Self {
            task_sender,
            pwd_updater: None,
            stopper: None,
        }
    }

    pub(crate) fn start(&mut self, pwd: &str) {
        let sender = self.task_sender.clone();
        let (tx_stopper, rx_stopper) = mpsc::channel();
        let (tx_pwd_updater, rx_pwd_updater) = mpsc::channel();
        self.pwd_updater = Some(tx_pwd_updater);
        self.stopper = Some(tx_stopper);
        let pwd = pwd.to_string();

        thread::spawn(move || keep_watching(pwd, sender, rx_pwd_updater, rx_stopper));
    }

    pub(crate) fn update_pwd(&self, pwd: &str) {
        if let Some(tx_pwd_updater) = &self.pwd_updater {
            tx_pwd_updater.send(pwd.into()).unwrap_or_default();
        }
    }

    pub(crate) fn stop_async(&self) {
        if let Some(stopper) = &self.stopper {
            stopper.send(true).unwrap_or_default(); // Let's not panic when xplr stops.
            // ack.recv().unwrap_or_default();  // Don't block, let the thread exit on its own.
        }
    }
}

fn keep_watching(
    pwd: String,
    tx_msg_in: Sender<Task>,
    rx_pwd_updater: Receiver<String>,
    rx_stopper: Receiver<bool>,
) {
    let mut pwd = PathBuf::from(pwd);
    let mut last_modified = pwd.metadata().and_then(|m| m.modified()).unwrap();

    loop {
        if rx_stopper.try_recv().unwrap_or(false) {
            break;
        } else if let Ok(new_pwd) = rx_pwd_updater.try_recv() {
            pwd = PathBuf::from(new_pwd);
            last_modified = pwd
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or(last_modified)
        } else if let Err(e) = pwd
            .metadata()
            .map_err(Error::new)
            .and_then(|m| m.modified().map_err(Error::new))
            .and_then(|modified| {
                if modified != last_modified {
                    last_modified = modified;
                    let msg = MsgIn::External(ExternalMsg::ExplorePwdAsync);
                    tx_msg_in.send(Task::new(msg, None)).map_err(Error::new)
                } else {
                    thread::sleep(Duration::from_secs(1));
                    Result::Ok(())
                }
            })
        {
            let msg = MsgIn::External(ExternalMsg::LogError(e.to_string()));
            tx_msg_in.send(Task::new(msg, None)).unwrap_or_default();
            thread::sleep(Duration::from_secs(1));
        }
    }
}
