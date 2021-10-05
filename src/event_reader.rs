use crate::app::Task;
use crate::app::{ExternalMsg, InternalMsg, MsgIn};
use crate::input::Key;
use crossterm::event::{self, Event, MouseEventKind};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub(crate) struct EventReader {
    task_sender: Sender<Task>,
    stopper: Option<(Sender<bool>, Receiver<()>)>,
}

impl EventReader {
    pub(crate) fn new(task_sender: Sender<Task>) -> Self {
        Self {
            task_sender,
            stopper: None,
        }
    }

    pub(crate) fn start(&mut self) {
        let sender = self.task_sender.clone();
        let (tx_stopper, rx_stopper) = mpsc::channel();
        let (tx_ack, rx_ack) = mpsc::channel();
        self.stopper = Some((tx_stopper, rx_ack));

        thread::spawn(move || {
            keep_reading(sender, rx_stopper, tx_ack);
        });
    }

    pub(crate) fn stop(&self) {
        if let Some((stopper, ack)) = &self.stopper {
            stopper.send(true).unwrap();
            ack.recv().unwrap();
        }
    }
}

fn keep_reading(tx_msg_in: Sender<Task>, rx_stopper: Receiver<bool>, tx_ack: Sender<()>) {
    loop {
        if rx_stopper.try_recv().unwrap_or(false) {
            tx_ack.send(()).unwrap();
            break;
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
}
