use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

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

pub struct SharedReadGuard<'s, T: ?Sized>(RwLockReadGuard<'s, T>);

impl<T: ?Sized> Deref for SharedReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> Borrow<T> for SharedReadGuard<'_, T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

pub struct SharedWriteGuard<'s, T: ?Sized>(RwLockWriteGuard<'s, T>);

impl<T: ?Sized> Deref for SharedWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for SharedWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ?Sized> Borrow<T> for SharedWriteGuard<'_, T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T: ?Sized> BorrowMut<T> for SharedWriteGuard<'_, T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
