use crate::app::Task;
use crate::app::{ExternalMsg, InternalMsg, MsgIn};
use crate::input::Key;
use crossterm::event::EventStream;
use crossterm::event::{Event, MouseEventKind};
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub(crate) struct EventReader {
    task_sender: Sender<Task>,
    stopper: Option<(Sender<bool>, Receiver<()>)>,
}

async fn send_events(sender: Sender<Task>, stopper: Receiver<bool>, ack: Sender<()>) {
    let mut reader = EventStream::new();

    loop {
        let mut delay = Delay::new(Duration::from_millis(150)).fuse();
        let mut event = reader.next().fuse();

        select! {
            _ = delay => if stopper.try_recv().unwrap_or(false) { ack.send(()).unwrap(); break; },
            maybe_event = event => {

                match maybe_event {
                    Some(Ok(Event::Key(key))) => {
                        let key = Key::from_event(key);
                        let msg = MsgIn::Internal(InternalMsg::HandleKey(key));
                        sender.send(Task::new(msg, Some(key))).unwrap();
                    }

                    Some(Ok(Event::Mouse(evt))) => match evt.kind {
                        MouseEventKind::ScrollUp => {
                            let msg = MsgIn::External(ExternalMsg::FocusPrevious);
                            sender.send(Task::new(msg, None)).unwrap();
                        }

                        MouseEventKind::ScrollDown => {
                            let msg = MsgIn::External(ExternalMsg::FocusNext);
                            sender.send(Task::new(msg, None)).unwrap();
                        }
                        _ => {}
                    },

                    Some(Ok(Event::Resize(_, _))) => {
                        let msg = MsgIn::External(ExternalMsg::Refresh);
                        sender.send(Task::new(msg, None)).unwrap();
                    }

                    Some(Err(e)) => {
                        sender
                            .send(Task::new(
                                MsgIn::External(ExternalMsg::LogError(e.to_string())),
                                None,
                            ))
                            .unwrap();
                    }
                    None => break,
                }
            }
        };
    }
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
            async_std::task::block_on(send_events(sender, rx_stopper, tx_ack));
        });
    }

    pub(crate) fn stop(&self) {
        if let Some((stopper, ack)) = &self.stopper {
            stopper.send(true).unwrap();
            ack.recv().unwrap();
        }
    }
}
