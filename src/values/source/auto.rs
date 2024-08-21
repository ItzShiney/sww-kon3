use super::ValueSource;
use super::ValueSourceMut;

pub trait AutoValueSource {}

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
