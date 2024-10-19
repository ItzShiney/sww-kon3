use std::ops::Deref;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

// TODO reuse the allocation for Vec, Box, etc.
pub struct Cache<T>(RwLock<Option<T>>);

impl<T> Cache<T> {
    #[expect(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self(RwLock::new(None))
    }

    pub fn reset(&self) {
        *self.0.write().unwrap() = None;
    }

    pub fn get_or_insert_with(&self, f: impl FnOnce() -> T) -> CacheGuard<T> {
        let value = self.0.read().unwrap();
        if value.is_some() {
            CacheGuard(value)
        } else {
            drop(value);
            self.0.write().unwrap().get_or_insert_with(f);
            CacheGuard(self.0.read().unwrap())
        }
    }
}

pub struct CacheGuard<'s, T>(RwLockReadGuard<'s, Option<T>>);

impl<T> Deref for CacheGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}
