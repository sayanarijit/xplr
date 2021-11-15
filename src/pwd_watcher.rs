use crate::app::Task;
use crate::app::{ExternalMsg, MsgIn};
use anyhow::{Error, Result};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn keep_watching(
    pwd: &str,
    tx_msg_in: Sender<Task>,
    rx_pwd_watcher: Receiver<String>,
) -> Result<()> {
    let mut pwd = PathBuf::from(pwd);
    let mut last_modified = pwd.metadata().and_then(|m| m.modified())?;

    thread::spawn(move || loop {
        if let Ok(new_pwd) = rx_pwd_watcher.try_recv() {
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
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_pwd_watcher() {
        let (tx_msg_in, rx_msg_in) = mpsc::channel();
        let (_, rx_pwd_watcher) = mpsc::channel();

        let result = keep_watching("/tmp", tx_msg_in, rx_pwd_watcher);

        assert!(result.is_ok());

        let file = "/tmp/__xplr_pwd_watcher_test__";
        std::fs::write(file, "test").unwrap();
        std::fs::remove_file(file).unwrap();

        let task = rx_msg_in.recv().unwrap();
        let msg = MsgIn::External(ExternalMsg::ExplorePwdAsync);
        assert_eq!(task, Task::new(msg, None));
    }
}
