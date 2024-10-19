use crate::app::SignalSender;
use crate::shared::SharedAddr;
use crate::shared::SharedReadGuard;
use crate::shared::SharedWriteGuard;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::Shared;
use std::borrow::Borrow;
use std::ops::Deref;
use std::ops::DerefMut;

mod auto;

pub use auto::*;

pub trait ContainsShared {
    fn contains_shared(&self, addr: SharedAddr) -> bool;
}

pub trait ValueSource: ContainsShared + HandleEvent {
    type Value<'s>: Deref + 's
    where
        Self: 's;

    fn value(&self) -> Self::Value<'_>;
}

pub trait ValueSourceMut: ValueSource {
    type ValueMut<'s>: DerefMut + 's
    where
        Self: 's;

    fn value_mut<'s>(&'s mut self, signal_sender: &'s SignalSender) -> Self::ValueMut<'s>;
}

impl<T: ?Sized> ValueSource for Shared<T> {
    type Value<'s>
        = SharedReadGuard<'s, T>
    where
        Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self.read()
    }
}

impl<T: ?Sized> ContainsShared for Shared<T> {
    fn contains_shared(&self, addr: SharedAddr) -> bool {
        self.addr() == addr
    }
}

impl<T: ?Sized> HandleEvent for Shared<T> {
    fn handle_event(&self, _signal_sender: &SignalSender, _event: &Event) -> EventResult {
        Ok(())
    }
}

impl<T: ?Sized> ValueSourceMut for Shared<T> {
    type ValueMut<'s>
        = SharedWriteGuard<'s, T>
    where
        Self: 's;

    fn value_mut<'s>(&'s mut self, signal_sender: &'s SignalSender) -> Self::ValueMut<'s> {
        self.write(signal_sender)
    }
}

pub trait ValueSourceBorrow<V: ?Sized>:
    for<'s> ValueSource<Value<'s>: Deref<Target: Borrow<V>>>
{
}
impl<V: ?Sized, T: for<'s> ValueSource<Value<'s>: Deref<Target: Borrow<V>>> + ?Sized>
    ValueSourceBorrow<V> for T
{
}
