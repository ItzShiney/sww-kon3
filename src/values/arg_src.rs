use crate as kon3;
use crate::shared::Shared;
use crate::Build;
use crate::EventResult;
use crate::IntoEventResult;

pub trait ArgSource {
    type Arg<'s>: 's;

    fn apply_to<R: IntoEventResult>(&self, f: &mut impl FnMut(Self::Arg<'_>) -> R) -> EventResult;
}

impl<T: ?Sized> ArgSource for Shared<T> {
    type Arg<'s> = &'s T;

    fn apply_to<R: IntoEventResult>(&self, f: &mut impl FnMut(Self::Arg<'_>) -> R) -> EventResult {
        f(&self.read()).into_event_result()
    }
}

#[derive(Debug, Build)]
pub struct Write<T>(T);

impl<T: ?Sized> ArgSource for Write<Shared<T>> {
    type Arg<'s> = &'s mut T;

    fn apply_to<R: IntoEventResult>(&self, f: &mut impl FnMut(Self::Arg<'_>) -> R) -> EventResult {
        f(&mut self.0.write()).into_event_result()
    }
}

pub const fn write<T>(ra_fixture_source: T) -> Write<T> {
    Write(ra_fixture_source)
}
