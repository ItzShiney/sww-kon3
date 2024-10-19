use crate::shared::SharedAddr;
use std::sync::mpsc::Sender;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Signal {
    Redraw,
    SharedUpdated(SharedAddr),
}

pub struct SignalSender(pub(super) Sender<Signal>);

impl SignalSender {
    pub fn send(&self, signal: Signal) {
        self.0.send(signal).unwrap();
    }
}
