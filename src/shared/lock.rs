use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::MutexGuard;

pub struct SharedLock<'s, T: ?Sized>(pub(super) MutexGuard<'s, T>);

impl<T: ?Sized> Deref for SharedLock<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for SharedLock<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ?Sized> Borrow<T> for SharedLock<'_, T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T: ?Sized> BorrowMut<T> for SharedLock<'_, T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
