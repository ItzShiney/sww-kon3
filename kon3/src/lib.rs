pub mod app;
pub mod elements;
pub mod prelude;
pub mod shared;
pub mod values;

use resources::Resources;
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

pub fn consume(ra_fixture_f: impl Fn()) -> impl Fn() -> Consume {
    move || {
        ra_fixture_f();
        Consume
    }
}

pub trait HandleEvent {
    fn handle_event(&self, event: &Event) -> EventResult;
}

pub trait Element: HandleEvent + InvalidateCache {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: Location);
}

impl<T: Element + ?Sized> Element for Arc<T> {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: Location) {
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

pub struct ReversedTuple<T>(pub T);

macro_rules! impl_tuple {
    ( $($T:ident)+ ; $($Reversed:tt)+ ) => {
        impl<$($T: HandleEvent),+> HandleEvent for ($($T),+) {
            fn handle_event(&self, event: &Event) -> EventResult {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                $( $T.handle_event(event)?; )+
                Ok(())
            }
        }

        impl<$($T: HandleEvent),+> HandleEvent for ReversedTuple<&($($T),+)> {
            fn handle_event(&self, event: &Event) -> EventResult {
                $( self.0 .$Reversed.handle_event(event)?; )+
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

impl_tuple!(A B; 1 0);
impl_tuple!(A B C; 2 1 0);
impl_tuple!(A B C D; 3 2 1 0);
impl_tuple!(A B C D E; 4 3 2 1 0);
impl_tuple!(A B C D E F; 5 4 3 2 1 0);
impl_tuple!(A B C D E F G; 6 5 4 3 2 1 0);
impl_tuple!(A B C D E F G H; 7 6 5 4 3 2 1 0);
impl_tuple!(A B C D E F G H I; 8 7 6 5 4 3 2 1 0);
impl_tuple!(A B C D E F G H I J; 9 8 7 6 5 4 3 2 1 0);
impl_tuple!(A B C D E F G H I J K; 10 9 8 7 6 5 4 3 2 1 0);
impl_tuple!(A B C D E F G H I J K L; 11 10 9 8 7 6 5 4 3 2 1 0);
