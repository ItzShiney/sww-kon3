use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

mod read;
mod write;

pub use read::*;
pub use write::*;

pub struct Shared<T: ?Sized>(Arc<RwLock<T>>);

impl<T: Debug> Debug for Shared<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Shared").field(&*self.read()).finish()
    }
}

impl<T: ?Sized> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Shared<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }
}

impl<T: ?Sized> Shared<T> {
    pub fn read(&self) -> SharedReadGuard<'_, T> {
        SharedReadGuard(self.0.read().expect("shared value was already locked"))
    }

    pub fn write(&self) -> SharedWriteGuard<'_, T> {
        SharedWriteGuard(self.0.write().expect("shared value was already locked"))
    }
}
