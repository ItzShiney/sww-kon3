use crate::app::Signal;
use crate::app::SignalSender;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Addr(usize);

pub struct Shared<T: ?Sized + 'static> {
    value: Arc<Mutex<T>>,
    signal_sender: SignalSender,
}

impl<T: ?Sized> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            signal_sender: SignalSender::clone(&self.signal_sender),
        }
    }
}

impl<T> Shared<T> {
    pub fn new(value: T, signal_sender: SignalSender) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
            signal_sender,
        }
    }
}

impl<T: ?Sized> Shared<T> {
    pub fn lock(&self) -> SharedGuard<T> {
        SharedGuard {
            addr: self.addr(),
            guard: self.value.lock().expect("shared value was already locked"),
            signal_sender: SignalSender::clone(&self.signal_sender),
        }
    }

    pub fn addr(&self) -> Addr {
        Addr(Arc::as_ptr(&self.value).cast::<()>() as usize)
    }
}

pub struct SharedGuard<'s, T: ?Sized + 'static> {
    guard: MutexGuard<'s, T>,
    signal_sender: SignalSender,
    addr: Addr,
}

impl<T: ?Sized> Deref for SharedGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T: ?Sized> DerefMut for SharedGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.signal_sender.send(Signal::InvalidateCache(self.addr));
        &mut self.guard
    }
}
