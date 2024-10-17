use crate::shared;
use crate::shared::SharedGuard;
use crate::InvalidateCaches;
use crate::Shared;
use std::borrow::Borrow;
use std::collections::BTreeSet;
use std::ops::Deref;
use std::ops::DerefMut;

mod auto;

pub use auto::*;

pub trait ValueSource: InvalidateCaches {
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

impl<T: ?Sized> InvalidateCaches for Shared<T> {
    fn invalidate_caches(&self, addrs: &BTreeSet<shared::Addr>) -> bool {
        addrs.contains(&self.addr())
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
