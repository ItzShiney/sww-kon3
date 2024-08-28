use super::ValueSource;
use super::ValueSourceMut;
use crate::shared;
use crate::InvalidateCache;
use sww::Color;
use sww::Mat2;
use sww::Vec2;
use sww::Vec4;

pub trait AutoValueSource {}

impl<T: AutoValueSource + ?Sized> ValueSource for T {
    type Value<'s> = &'s T where Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self
    }
}

impl<T: AutoValueSource + ?Sized> ValueSourceMut for T {
    type ValueMut<'s> = &'s mut T where Self: 's;

    fn value_mut(&mut self) -> Self::ValueMut<'_> {
        self
    }
}

impl<T: AutoValueSource + ?Sized> InvalidateCache for T {
    fn invalidate_cache(&self, _addr: shared::Addr) -> bool {
        false
    }
}

impl<T: ?Sized> AutoValueSource for &T {}

impl AutoValueSource for bool {}
impl AutoValueSource for f32 {}
impl AutoValueSource for f64 {}
impl AutoValueSource for i8 {}
impl AutoValueSource for i16 {}
impl AutoValueSource for i32 {}
impl AutoValueSource for i64 {}
impl AutoValueSource for i128 {}
impl AutoValueSource for isize {}
impl AutoValueSource for u8 {}
impl AutoValueSource for u16 {}
impl AutoValueSource for u32 {}
impl AutoValueSource for u64 {}
impl AutoValueSource for u128 {}
impl AutoValueSource for usize {}
impl AutoValueSource for str {}

impl AutoValueSource for Color {}
impl AutoValueSource for Mat2 {}
impl AutoValueSource for Vec2 {}
impl AutoValueSource for Vec4 {}
