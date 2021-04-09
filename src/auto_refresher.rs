use crate::app::{ExternalMsg, MsgIn, Task};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub fn start_auto_refreshing(tx: Sender<Task>) {
    thread::spawn(move || loop {
        tx.send(Task::new(MsgIn::External(ExternalMsg::Refresh), None))
            .unwrap();
        thread::sleep(Duration::from_secs(1));
    });
}
