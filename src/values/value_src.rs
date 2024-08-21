use super::CacheRef;
use crate::shared::SharedLock;
use crate::Shared;
use std::borrow::Borrow;
use std::ops::Deref;
use std::ops::DerefMut;

pub enum SourcedValue<'s, T: ToOwned + ?Sized> {
    Ref(&'s T),
    Lock(SharedLock<'s, T>),
    Cached(CacheRef<'s, T::Owned>),
}

impl<'s, T: ToOwned + ?Sized> Deref for SourcedValue<'s, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(value) => value,
            Self::Cached(value) => (**value).borrow(),
            Self::Lock(value) => value,
        }
    }
}

pub enum SourcedValueMut<'s, T: ?Sized> {
    Ref(&'s mut T),
    Lock(SharedLock<'s, T>),
}

impl<T: ?Sized> Deref for SourcedValueMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(value) => value,
            Self::Lock(value) => value,
        }
    }
}

impl<T: ?Sized> DerefMut for SourcedValueMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Ref(value) => value,
            Self::Lock(value) => value,
        }
    }
}

pub trait ValueSource {
    type Value: ToOwned + ?Sized;

    fn value(&self) -> SourcedValue<'_, Self::Value>;
}

pub trait ValueSourceMut: ValueSource {
    fn value_mut(&mut self) -> SourcedValueMut<'_, Self::Value>;
}

pub trait AutoValueSource: ToOwned {}

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
    type Value = T;

    fn value(&self) -> SourcedValue<'_, T> {
        SourcedValue::Ref(self)
    }
}

impl<T: AutoValueSource> ValueSourceMut for T {
    fn value_mut(&mut self) -> SourcedValueMut<'_, T> {
        SourcedValueMut::Ref(self)
    }
}

impl ValueSource for &str {
    type Value = str;

    fn value(&self) -> SourcedValue<'_, Self::Value> {
        SourcedValue::Ref(self)
    }
}

impl<T: ToOwned + ?Sized> ValueSource for Shared<T> {
    type Value = T;

    fn value(&self) -> SourcedValue<'_, Self::Value> {
        SourcedValue::Lock(self.lock())
    }
}

impl<T: ?Sized> ValueSourceMut for Shared<T>
where
    Self: ValueSource<Value = T>,
{
    fn value_mut(&mut self) -> SourcedValueMut<'_, Self::Value> {
        SourcedValueMut::Lock(self.lock())
    }
}
