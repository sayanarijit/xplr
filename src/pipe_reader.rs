use crate::app::{ExternalMsg, MsgIn, Task};
use std::fs;
use std::io::prelude::*;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub fn keep_reading(pipe: String, tx: Sender<Task>) {
    thread::spawn(move || loop {
        if let Ok(mut file) = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&pipe)
        {
            let mut in_str = String::new();
            file.read_to_string(&mut in_str).unwrap_or_default();
            file.set_len(0).unwrap_or_default();

            if !in_str.is_empty() {
                let msgs = in_str
                    .lines()
                    .map(|s| serde_yaml::from_str::<ExternalMsg>(s.trim()));

                msgs.for_each(|msg| match msg {
                    Ok(m) => {
                        tx.send(Task::new(MsgIn::External(m), None)).unwrap();
                    }
                    Err(e) => {
                        tx.send(Task::new(
                            MsgIn::External(ExternalMsg::LogError(e.to_string())),
                            None,
                        ))
                        .unwrap();
                    }
                });
            } else {
                thread::sleep(Duration::from_millis(50));
            }
        } else {
            tx.send(Task::new(
                MsgIn::External(ExternalMsg::LogError(format!(
                    "Failed to open input pipe: {}",
                    &pipe
                ))),
                None,
            ))
            .unwrap();
            thread::sleep(Duration::from_secs(3));
        }
    });
}
