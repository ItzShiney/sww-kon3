use super::CacheRef;
use crate::shared::SharedReadGuard;
use crate::shared::SharedWriteGuard;
use crate::Anchor;
use crate::Build;
use crate::ResolveAnchors;
use crate::Shared;
use std::borrow::Borrow;
use std::ops::Deref;
use std::ops::DerefMut;

pub enum SourcedValue<'s, T: ToOwned + ?Sized> {
    Ref(&'s T),
    Guard(SharedReadGuard<'s, T>),
    Cached(CacheRef<'s, T::Owned>),
}

impl<'s, T: ToOwned + ?Sized> Deref for SourcedValue<'s, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(value) => value,
            Self::Cached(value) => (**value).borrow(),
            Self::Guard(value) => value,
        }
    }
}

pub enum SourcedValueMut<'s, T: ?Sized> {
    Ref(&'s mut T),
    Guard(SharedWriteGuard<'s, T>),
}

impl<T: ?Sized> Deref for SourcedValueMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(value) => value,
            Self::Guard(value) => value,
        }
    }
}

impl<T: ?Sized> DerefMut for SourcedValueMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Ref(value) => value,
            Self::Guard(value) => value,
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

impl<T: AutoValueSource> Build for T {
    type Built = Self;

    fn build(self) -> Self::Built {
        self
    }
}

impl<T: AutoValueSource> ResolveAnchors for T {
    type AnchorsSet = ();

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        None
    }

    fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Shared<A::Value>) {}
}

impl ValueSource for &str {
    type Value = str;

    fn value(&self) -> SourcedValue<'_, Self::Value> {
        SourcedValue::Ref(self)
    }
}

impl Build for &str {
    type Built = Self;

    fn build(self) -> Self::Built {
        self
    }
}

impl ResolveAnchors for &str {
    type AnchorsSet = ();

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        None
    }

    fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Shared<A::Value>) {}
}

impl<T: ToOwned + ?Sized> ValueSource for Shared<T> {
    type Value = T;

    fn value(&self) -> SourcedValue<'_, Self::Value> {
        SourcedValue::Guard(self.read())
    }
}

impl<T: ?Sized> ValueSourceMut for Shared<T>
where
    Self: ValueSource<Value = T>,
{
    fn value_mut(&mut self) -> SourcedValueMut<'_, Self::Value> {
        SourcedValueMut::Guard(self.write())
    }
}
