use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::RwLockReadGuard;

pub struct SharedReadGuard<'s, T: ?Sized>(pub(super) RwLockReadGuard<'s, T>);

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
