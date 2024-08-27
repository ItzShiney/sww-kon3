use crate::app::SharedBuilder;
use crate::InvalidateCache;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Addr(usize);

pub struct Shared<T: ?Sized + 'static> {
    value: Arc<Mutex<T>>,
    app: SharedBuilder,
}

impl<T: ?Sized> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            app: SharedBuilder::clone(&self.app),
        }
    }
}

impl<T> Shared<T> {
    pub fn new(value: T, app: SharedBuilder) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
            app,
        }
    }
}

impl<T: ?Sized> Shared<T> {
    pub fn lock(&self) -> SharedGuard<T> {
        SharedGuard {
            addr: self.addr(),
            guard: self.value.lock().expect("shared value was already locked"),
            app: SharedBuilder::clone(&self.app),
        }
    }

    pub fn addr(&self) -> Addr {
        Addr(Arc::as_ptr(&self.value).cast::<()>() as usize)
    }
}

pub struct SharedGuard<'s, T: ?Sized + 'static> {
    guard: MutexGuard<'s, T>,
    app: SharedBuilder,
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
        self.app.invalidate_cache(self.addr);
        &mut self.guard
    }
}
