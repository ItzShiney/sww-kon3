use crate::shared::SharedLock;
use crate::Shared;
use std::ops::Deref;
use std::ops::DerefMut;

pub trait ValueSource {
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

pub trait AutoValueSource {}

impl AutoValueSource for u8 {}
impl AutoValueSource for u16 {}
impl AutoValueSource for u32 {}
impl AutoValueSource for u64 {}
impl AutoValueSource for u128 {}
impl AutoValueSource for usize {}
impl AutoValueSource for i8 {}
impl AutoValueSource for i16 {}
impl AutoValueSource for i32 {}
impl AutoValueSource for i64 {}
impl AutoValueSource for i128 {}
impl AutoValueSource for isize {}

impl<T: AutoValueSource> ValueSource for T {
    type Value<'s> = &'s T where Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self
    }
}

impl<T: AutoValueSource> ValueSourceMut for T {
    type ValueMut<'s> = &'s mut T where Self: 's;

    fn value_mut(&mut self) -> Self::ValueMut<'_> {
        self
    }
}

impl ValueSource for &str {
    type Value<'s> = &'s str where Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self
    }
}

impl<T: ?Sized> ValueSource for Shared<T> {
    type Value<'s> = SharedLock<'s, T>
    where
        Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self.lock()
    }
}

impl<T: ?Sized> ValueSourceMut for Shared<T> {
    type ValueMut<'s> = SharedLock<'s, T>
    where
        Self: 's;

    fn value_mut(&mut self) -> Self::ValueMut<'_> {
        self.lock()
    }
}
