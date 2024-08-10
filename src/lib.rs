pub use kon3_macros::*;

pub mod app;
pub mod elements;
pub mod prelude;
pub mod shared;
pub mod values;

use shared::Shared;
use std::cell::RefCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use sww::wgpu;

mod location;

pub use location::*;

pub enum Event {
    Click,
    _1,
    _2,
}

pub struct EventConsumed;

pub const fn consume() -> Result<(), EventConsumed> {
    Err(EventConsumed)
}

pub trait HandleEvent {
    fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed>;
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
    type Output;

    fn build(self) -> Self::Output;
}

#[derive(Debug)]
pub struct Ident<T>(T);

pub const fn id<T>(ra_fixture_value: T) -> Ident<T> {
    Ident(ra_fixture_value)
}

impl<T> Build for Ident<T> {
    type Output = T;

    fn build(self) -> Self::Output {
        self.0
    }
}

impl<T> ResolveAnchors for Ident<T> {
    type AnchorsSet = ();

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        None
    }

    fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Shared<A::Value>) {}
}

pub struct Cache<T>(PhantomData<T>);

pub type Cached<T> = RefCell<Option<T>>;

impl<T> Build for Cache<T> {
    type Output = Cached<T>;

    fn build(self) -> Self::Output {
        RefCell::new(None)
    }
}

pub const fn cache<T>() -> Cache<T> {
    Cache(PhantomData)
}

// TODO: remove `+ Debug`
pub trait BuildElement: Build<Output: Element + Debug> + ResolveAnchors {}
impl<T: Build<Output: Element + Debug> + ResolveAnchors> BuildElement for T {}

macro_rules! tuple_impls {
    ( $($T:ident)+ ) => {
        impl<$($T: Build),+> Build for ($($T),+) {
            type Output = ($($T::Output),+);

            fn build(self) -> Self::Output {
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
            fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
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
