pub mod app;
pub mod elements;
pub mod prelude;
pub mod shared;
pub mod values;

use app::Signaler;
use resources::Resources;
use shared::Shared;
use shared::SharedAddr;
use std::sync::Arc;
use sww::window::event::MouseButton;
use values::ContainsShared;

mod drawer;
mod location;

pub use drawer::*;
pub use location::*;

#[derive(Clone, Copy)]
pub enum Event {
    Click {
        point: LocationPoint,
        button: MouseButton,
    },
    SharedUpdated(SharedAddr),
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

// TODO add `location: LocationRect`?
pub trait HandleEvent {
    fn handle_event(&self, signaler: &Signaler, event: &Event) -> EventResult;
}

pub trait Element: HandleEvent {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: LocationRect);
}

impl<T: Element + ?Sized> Element for Arc<T> {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: LocationRect) {
        self.as_ref().draw(pass, resources, location);
    }
}

impl<T: HandleEvent + ?Sized> HandleEvent for Arc<T> {
    fn handle_event(&self, signaler: &Signaler, event: &Event) -> EventResult {
        self.as_ref().handle_event(signaler, event)
    }
}

pub struct ReversedTuple<T>(pub T);

macro_rules! impl_tuple {
    ( $($T:ident)+ | $($Reversed:tt)+ ) => {
        impl<$($T: HandleEvent),+> HandleEvent for ($($T),+) {
            fn handle_event(&self, signaler: &Signaler, event: &Event) -> EventResult {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                $( $T.handle_event(signaler, event)?; )+
                Ok(())
            }
        }

        impl<$($T: HandleEvent),+> HandleEvent for ReversedTuple<&($($T),+)> {
            fn handle_event(&self, signaler: &Signaler, event: &Event) -> EventResult {
                $( self.0 .$Reversed.handle_event(signaler, event)?; )+
                Ok(())
            }
        }

        impl<$($T: ContainsShared),+> ContainsShared for ($($T),+) {
            fn contains_shared(&self, addr: SharedAddr) -> bool {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                $( $T.contains_shared(addr) )||+
            }
        }
    };
}

impl_tuple!(A B                     |                       1 0);
impl_tuple!(A B C                   |                     2 1 0);
impl_tuple!(A B C D                 |                   3 2 1 0);
impl_tuple!(A B C D E               |                 4 3 2 1 0);
impl_tuple!(A B C D E F             |               5 4 3 2 1 0);
impl_tuple!(A B C D E F G           |             6 5 4 3 2 1 0);
impl_tuple!(A B C D E F G H         |           7 6 5 4 3 2 1 0);
impl_tuple!(A B C D E F G H I       |         8 7 6 5 4 3 2 1 0);
impl_tuple!(A B C D E F G H I J     |       9 8 7 6 5 4 3 2 1 0);
impl_tuple!(A B C D E F G H I J K   |    10 9 8 7 6 5 4 3 2 1 0);
impl_tuple!(A B C D E F G H I J K L | 11 10 9 8 7 6 5 4 3 2 1 0);
