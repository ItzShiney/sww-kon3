use crate::shared;
use crate::shared::SharedGuard;
use crate::InvalidateCache;
use crate::Shared;
use std::borrow::Borrow;
use std::ops::Deref;
use std::ops::DerefMut;

mod auto;

pub use auto::*;

pub trait ValueSource: InvalidateCache {
    type Value<'s>: Deref + 's
    where
        Self: 's;

    fn value(&self) -> Self::Value<'_>;
}

pub trait ValueSourceMut: ValueSource {
    type ValueMut<'s>: DerefMut + 's
    where
        Self: 's;

    fn value_mut(&mut self) -> Self::ValueMut<'_>;
}

impl<T: ?Sized> ValueSource for Shared<T> {
    type Value<'s> = SharedGuard<'s, T>
    where
        Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self.lock()
    }
}

impl<T: ?Sized> ValueSourceMut for Shared<T> {
    type ValueMut<'s> = SharedGuard<'s, T>
    where
        Self: 's;

    fn value_mut(&mut self) -> Self::ValueMut<'_> {
        self.lock()
    }
}

impl<T: ?Sized> InvalidateCache for Shared<T> {
    fn invalidate_cache(&self, addr: shared::Addr) -> bool {
        self.addr() == addr
    }
}

impl ValueSource for &str {
    type Value<'s> = &'s str where Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self
    }
}

impl InvalidateCache for &str {
    fn invalidate_cache(&self, _addr: shared::Addr) -> bool {
        false
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
