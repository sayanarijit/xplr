use crate::app::Task;
use crate::app::{ExternalMsg, InternalMsg, MsgIn};
use crate::input::Key;
use crossterm::event::{self, Event};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn keep_reading(tx_msg_in: Sender<Task>, rx_event_reader: Receiver<bool>) {
    thread::spawn(move || {
        let mut is_paused = false;
        loop {
            if let Ok(paused) = rx_event_reader.try_recv() {
                is_paused = paused;
            };

            if !is_paused && event::poll(std::time::Duration::from_millis(1)).unwrap() {
                match event::read() {
                    Ok(Event::Key(key)) => {
                        let key = Key::from_event(key);
                        let msg = MsgIn::Internal(InternalMsg::HandleKey(key));
                        tx_msg_in.send(Task::new(0, msg, Some(key))).unwrap();
                    }

                    Ok(Event::Resize(_, _)) => {
                        let msg = MsgIn::External(ExternalMsg::Refresh);
                        tx_msg_in.send(Task::new(0, msg, None)).unwrap();
                    }
                    Ok(_) => {}
                    Err(e) => {
                        tx_msg_in
                            .send(Task::new(
                                0,
                                MsgIn::External(ExternalMsg::LogError(e.to_string())),
                                None,
                            ))
                            .unwrap();
                    }
                }
            } else {
                thread::sleep(Duration::from_millis(1));
            }
        }
    });
}
