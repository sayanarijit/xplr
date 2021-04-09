use crate::app::{ExternalMsg, MsgIn, Task};
use std::fs;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub fn keep_reading(pipe: String, tx: Sender<Task>) {
    thread::spawn(move || loop {
        let in_str = fs::read_to_string(&pipe).unwrap_or_default();

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
            fs::write(&pipe, "").unwrap();
        } else {
            thread::sleep(Duration::from_millis(50));
        }
    });
}
