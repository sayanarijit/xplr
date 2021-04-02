use crate::app::{ExternalMsg, MsgIn, Task};
use serde_yaml;
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
                .filter_map(|s| serde_yaml::from_str::<ExternalMsg>(s.trim()).ok());

            msgs.for_each(|msg| {
                tx.send(Task::new(2, MsgIn::External(msg), None)).unwrap();
            });
            fs::write(&pipe, "").unwrap();
        };
        thread::sleep(Duration::from_millis(10));
    });
}
