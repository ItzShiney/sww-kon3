use crate::shared::SharedAddr;
use std::sync::mpsc::Sender;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Signal {
    Redraw,
    SharedUpdated(SharedAddr),
}

pub struct Signaler(pub(super) Sender<Signal>);

impl Signaler {
    pub fn send(&self, signal: Signal) {
        self.0.send(signal).unwrap();
    }
}
