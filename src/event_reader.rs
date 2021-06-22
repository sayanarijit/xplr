use crate::app::Task;
use crate::app::{ExternalMsg, InternalMsg, MsgIn};
use crate::input::Key;
use crossterm::event::{self, Event, MouseEventKind};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub fn keep_reading(tx_msg_in: Sender<Task>, rx_event_reader: Receiver<bool>) {
    thread::spawn(move || {
        let mut is_paused = false;
        loop {
            if let Ok(paused) = rx_event_reader.try_recv() {
                is_paused = paused;
            };

            if is_paused {
                thread::sleep(std::time::Duration::from_millis(200));
            } else if event::poll(std::time::Duration::from_millis(150)).unwrap_or_default() {
                // NOTE: The poll timeout need to stay low, else spawning sub subshell
                // and start typing immediately will cause panic.
                // To reproduce, press `:`, then press and hold `!`.
                match event::read() {
                    Ok(Event::Key(key)) => {
                        let key = Key::from_event(key);
                        let msg = MsgIn::Internal(InternalMsg::HandleKey(key));
                        tx_msg_in
                            .send(Task::new(msg, Some(key)))
                            .unwrap_or_default();
                    }

                    Ok(Event::Mouse(evt)) => match evt.kind {
                        MouseEventKind::ScrollUp => {
                            let msg = MsgIn::External(ExternalMsg::FocusPrevious);
                            tx_msg_in.send(Task::new(msg, None)).unwrap_or_default();
                        }

                        MouseEventKind::ScrollDown => {
                            let msg = MsgIn::External(ExternalMsg::FocusNext);
                            tx_msg_in.send(Task::new(msg, None)).unwrap_or_default();
                        }
                        _ => {}
                    },

                    Ok(Event::Resize(_, _)) => {
                        let msg = MsgIn::External(ExternalMsg::Refresh);
                        tx_msg_in.send(Task::new(msg, None)).unwrap_or_default();
                    }

                    Err(e) => {
                        tx_msg_in
                            .send(Task::new(
                                MsgIn::External(ExternalMsg::LogError(e.to_string())),
                                None,
                            ))
                            .unwrap_or_default();
                    }
                }
            }
        }
    });
}
