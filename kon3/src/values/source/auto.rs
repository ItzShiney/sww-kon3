use super::ValueSource;
use super::ValueSourceMut;
use crate::app::SignalSender;
use crate::shared::SharedAddr;
use crate::ContainsShared;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use sww::Color;
use sww::Mat2;
use sww::Vec2;
use sww::Vec4;

pub trait AutoValueSource {}

impl<T: AutoValueSource + ?Sized> ValueSource for T {
    type Value<'s>
        = &'s T
    where
        Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self
    }
}

impl<T: AutoValueSource + ?Sized> ValueSourceMut for T {
    type ValueMut<'s>
        = &'s mut T
    where
        Self: 's;

    fn value_mut<'s>(
        &'s mut self,
        _signal_sender: &'s crate::prelude::SignalSender,
    ) -> Self::ValueMut<'s> {
        self
    }
}

impl<T: AutoValueSource + ?Sized> ContainsShared for T {
    fn contains_shared(&self, _addr: SharedAddr) -> bool {
        false
    }
}

impl<T: AutoValueSource + ?Sized> HandleEvent for T {
    fn handle_event(&self, _signal_sender: &SignalSender, _event: &Event) -> EventResult {
        Ok(())
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
