use crate::app::Signal;
use crate::app::SignalSender;
use parking_lot::Mutex;
use parking_lot::MutexGuard;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SharedAddr(usize);

// TODO this shouldn't be the only shared type, what about `RwLock` or atomics?
pub struct Shared<T: ?Sized + 'static>(Arc<Mutex<T>>);

impl<T: ?Sized> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Shared<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(Mutex::new(value)))
    }
}

impl<T: ?Sized> Shared<T> {
    pub fn read(&self) -> SharedReadGuard<'_, T> {
        SharedReadGuard(self.0.try_lock().expect("shared value was already locked"))
    }

    pub fn write<'s>(&'s self, signal_sender: &'s SignalSender) -> SharedWriteGuard<'s, T> {
        SharedWriteGuard {
            guard: self.0.try_lock().expect("shared value was already locked"),
            addr: self.addr(),
            signal_sender,
        }
    }

    pub fn addr(&self) -> SharedAddr {
        SharedAddr(Arc::as_ptr(&self.0).cast::<()>() as usize)
    }
}

pub struct SharedReadGuard<'s, T: ?Sized + 'static>(MutexGuard<'s, T>);

impl<T: ?Sized> Deref for SharedReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SharedWriteGuard<'s, T: ?Sized + 'static> {
    guard: MutexGuard<'s, T>,
    signal_sender: &'s SignalSender,
    addr: SharedAddr,
}

impl<T: ?Sized> Deref for SharedWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T: ?Sized> DerefMut for SharedWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.signal_sender.send(Signal::SharedUpdated(self.addr));
        &mut self.guard
    }
}
