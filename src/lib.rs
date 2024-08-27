pub mod app;
pub mod elements;
pub mod prelude;
pub mod shared;
pub mod values;

use shared::Shared;
use std::sync::Arc;

mod drawer;
mod location;

pub use drawer::*;
pub use location::*;

pub enum Event {
    Click,
    _1,
    _2,
}

pub struct Consume;

pub type EventResult = Result<(), Consume>;

pub trait IntoEventResult {
    fn into_event_result(self) -> EventResult;
}

impl IntoEventResult for EventResult {
    fn into_event_result(self) -> EventResult {
        self
    }
}

impl IntoEventResult for () {
    fn into_event_result(self) -> EventResult {
        Ok(())
    }
}

impl IntoEventResult for Consume {
    fn into_event_result(self) -> EventResult {
        Err(Self)
    }
}

pub trait HandleEvent {
    fn handle_event(&self, event: &Event) -> EventResult;
}

pub trait Element<R>: HandleEvent + InvalidateCache {
    fn draw(&self, pass: &mut DrawPass, resources: &R, location: Location);
}

impl<R, T: Element<R> + ?Sized> Element<R> for Arc<T> {
    fn draw(&self, pass: &mut DrawPass, resources: &R, location: Location) {
        self.as_ref().draw(pass, resources, location);
    }
}

impl<T: HandleEvent + ?Sized> HandleEvent for Arc<T> {
    fn handle_event(&self, event: &Event) -> EventResult {
        self.as_ref().handle_event(event)
    }
}

impl<T: InvalidateCache + ?Sized> InvalidateCache for Arc<T> {
    fn invalidate_cache(&self, addr: shared::Addr) -> bool {
        self.as_ref().invalidate_cache(addr)
    }
}

pub trait InvalidateCache {
    fn invalidate_cache(&self, addr: shared::Addr) -> bool;
}

macro_rules! impl_tuple {
    ( $($T:ident)+ ) => {
        impl<$($T: HandleEvent),+> HandleEvent for ($($T),+) {
            fn handle_event(&self, event: &Event) -> EventResult {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                $( $T.handle_event(event)?; )+
                Ok(())
            }
        }

        impl<$($T: InvalidateCache),+> InvalidateCache for ($($T),+) {
            fn invalidate_cache(&self, addr: shared::Addr) -> bool {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                $( $T.invalidate_cache(addr) )||+
            }
        }
    };
}

impl_tuple!(A B);
impl_tuple!(A B C);
impl_tuple!(A B C D);
impl_tuple!(A B C D E);
impl_tuple!(A B C D E F);
impl_tuple!(A B C D E F G);
impl_tuple!(A B C D E F G H);
impl_tuple!(A B C D E F G H I);
impl_tuple!(A B C D E F G H I J);
impl_tuple!(A B C D E F G H I J K);
impl_tuple!(A B C D E F G H I J K L);
