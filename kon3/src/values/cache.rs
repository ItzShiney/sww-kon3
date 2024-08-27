use std::cell;
use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;

// FIXME use `Mutex`
pub struct Cache<T>(RefCell<Option<T>>);

impl<T> Cache<T> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self(RefCell::new(None))
    }

    pub fn reset(&self) {
        *self.0.borrow_mut() = None;
    }

    pub fn get_or_insert_with(&self, f: impl FnOnce() -> T) -> CacheRef<T> {
        let mut value = self.0.borrow_mut();
        value.get_or_insert_with(f);
        CacheRef(value)
    }
}

pub struct CacheRef<'s, T>(cell::RefMut<'s, Option<T>>);

impl<T> Deref for CacheRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl<T> DerefMut for CacheRef<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}
