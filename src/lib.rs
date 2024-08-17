#![forbid(unsafe_code)]
pub use kon3_macros::*;

pub mod app;
pub mod elements;
pub mod prelude;
pub mod shared;
pub mod values;

use shared::Shared;
use std::cell::RefCell;
use std::marker::PhantomData;
use sww::wgpu;

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
    fn handle_event(&mut self, event: &Event) -> EventResult;
}

pub trait Element: HandleEvent {
    fn draw<'c>(&'c self, render_pass: &mut wgpu::RenderPass<'c>, location: Location) {
        _ = render_pass;
        _ = location;
        todo!()
    }
}

pub trait Anchor: 'static {
    type Value;
}

pub trait AnchorsTree {
    fn resolve_anchors(builder: &mut impl ResolveAnchors);
}

impl<A: Anchor> AnchorsTree for A {
    fn resolve_anchors(builder: &mut impl ResolveAnchors) {
        let anchor = builder.get_anchor::<A>().expect("anchor wasn't set");
        builder.resolve_anchor::<A>(&anchor);
    }
}

macro_rules! impl_anchors_tree {
    ($($T:ident)*) => {
        impl<$($T: AnchorsTree),*> AnchorsTree for ($($T),*) {
            fn resolve_anchors(_builder: &mut impl ResolveAnchors) {
                $( $T::resolve_anchors(_builder); )*
            }
        }
    };
}

impl_anchors_tree!();
impl_anchors_tree!(A B);
impl_anchors_tree!(A B C);
impl_anchors_tree!(A B C D);
impl_anchors_tree!(A B C D E);

pub trait Build {
    type Built;

    fn build(self) -> Self::Built;
}

pub struct Cache<T>(PhantomData<T>);

pub type Cached<T> = RefCell<Option<T>>;

impl<T> Build for Cache<T> {
    type Built = Cached<T>;

    fn build(self) -> Self::Built {
        RefCell::new(None)
    }
}

pub const fn cache<T>() -> Cache<T> {
    Cache(PhantomData)
}

pub trait BuildElement: Build<Built: Element> + ResolveAnchors {}
impl<T: Build<Built: Element> + ResolveAnchors> BuildElement for T {}

macro_rules! tuple_impls {
    ( $($T:ident)+ ) => {
        impl<$($T: Build),+> Build for ($($T),+) {
            type Built = ($($T::Built),+);

            fn build(self) -> Self::Built {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                ($($T.build()),+)
            }
        }

        impl<$($T: ResolveAnchors),+> ResolveAnchors for ($($T),+) {
            type AnchorsSet = ($($T::AnchorsSet),+);

            fn get_anchor<_A: Anchor>(&self) -> Option<$crate::shared::Shared<_A::Value>> {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                None $( .or_else(|| $T.get_anchor::<_A>()) )+
            }

            fn resolve_anchor<_A: Anchor>(&mut self, anchor: &$crate::shared::Shared<_A::Value>) {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                $( $T.resolve_anchor::<_A>(anchor); )+
            }
        }

        impl<$($T: HandleEvent),+> HandleEvent for ($($T),+) {
            fn handle_event(&mut self, event: &Event) -> EventResult {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                $( $T.handle_event(event)?; )+
                Ok(())
            }
        }
    };
}

tuple_impls!(A B);
tuple_impls!(A B C);
tuple_impls!(A B C D);
tuple_impls!(A B C D E);

pub trait ResolveAnchors {
    type AnchorsSet: AnchorsTree;

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>>;
    fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>);
}
