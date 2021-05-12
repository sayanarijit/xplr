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

            if !is_paused && event::poll(std::time::Duration::from_millis(200)).unwrap_or_default()
            {
                match event::read() {
                    Ok(Event::Key(key)) => {
                        let key = Key::from_event(key);
                        let msg = MsgIn::Internal(InternalMsg::HandleKey(key));
                        tx_msg_in
                            .send(Task::new(msg, Some(key)))
                            .unwrap_or_default();
                    }

                    Ok(Event::Resize(_, _)) => {
                        let msg = MsgIn::External(ExternalMsg::Refresh);
                        tx_msg_in.send(Task::new(msg, None)).unwrap_or_default();
                    }
                    Ok(_) => {}
                    Err(e) => {
                        tx_msg_in
                            .send(Task::new(
                                MsgIn::External(ExternalMsg::LogError(e.to_string())),
                                None,
                            ))
                            .unwrap_or_default();
                    }
                }
            } else {
                thread::sleep(Duration::from_millis(50));
            }
        }
    });
}
