use crate::app::Task;
use crate::app::{ExternalMsg, InternalMsg, MsgIn};
use crate::input::Key;
use anyhow::Result;
use crossterm::event::{self, Event, MouseEventKind};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub fn pause_reading(tx_event_reader: &Sender<bool>, rx_loop_waiter: &Receiver<()>) -> Result<()> {
    tx_event_reader.send(true)?;
    rx_loop_waiter.recv()?;
    Ok(())
}

pub fn resume_reading(tx_event_reader: &Sender<bool>, rx_loop_waiter: &Receiver<()>) -> Result<()> {
    tx_event_reader.send(false)?;
    rx_loop_waiter.recv()?;
    Ok(())
}

pub fn keep_reading(
    tx_msg_in: Sender<Task>,
    rx_event_reader: Receiver<bool>,
    tx_loop_waiter: Sender<()>,
) {
    thread::spawn(move || {
        let mut is_paused = false;
        loop {
            if let Ok(paused) = rx_event_reader.try_recv() {
                is_paused = paused;
                tx_loop_waiter.send(()).unwrap();
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
                        tx_msg_in.send(Task::new(msg, Some(key))).unwrap();
                    }

                    Ok(Event::Mouse(evt)) => match evt.kind {
                        MouseEventKind::ScrollUp => {
                            let msg = MsgIn::External(ExternalMsg::FocusPrevious);
                            tx_msg_in.send(Task::new(msg, None)).unwrap();
                        }

                        MouseEventKind::ScrollDown => {
                            let msg = MsgIn::External(ExternalMsg::FocusNext);
                            tx_msg_in.send(Task::new(msg, None)).unwrap();
                        }
                        _ => {}
                    },

                    Ok(Event::Resize(_, _)) => {
                        let msg = MsgIn::External(ExternalMsg::Refresh);
                        tx_msg_in.send(Task::new(msg, None)).unwrap();
                    }

                    Err(e) => {
                        tx_msg_in
                            .send(Task::new(
                                MsgIn::External(ExternalMsg::LogError(e.to_string())),
                                None,
                            ))
                            .unwrap();
                    }
                }
            }
        }
    });
}
