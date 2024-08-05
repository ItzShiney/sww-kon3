use kon_macros::Build;
use shared::Shared;
use std::any::Any;
use std::fmt::Debug;
use values::ValueSource;
use values::ValueSourceMut;

pub mod shared {
    use std::borrow::Borrow;
    use std::borrow::BorrowMut;
    use std::fmt::Debug;
    use std::ops::Deref;
    use std::ops::DerefMut;
    use std::sync::Arc;
    use std::sync::RwLock;
    use std::sync::RwLockReadGuard;
    use std::sync::RwLockWriteGuard;

    pub struct Shared<T: ?Sized>(Arc<RwLock<T>>);

    impl<T: Debug> Debug for Shared<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("Shared").field(&*self.read()).finish()
        }
    }

    impl<T: ?Sized> Clone for Shared<T> {
        fn clone(&self) -> Self {
            Shared(self.0.clone())
        }
    }

    impl<T> Shared<T> {
        pub fn new(value: T) -> Self {
            Self(Arc::new(RwLock::new(value)))
        }
    }

    impl<T: ?Sized> Shared<T> {
        pub fn read(&self) -> SharedReadGuard<'_, T> {
            SharedReadGuard(self.0.read().unwrap())
        }

        pub fn write(&self) -> SharedWriteGuard<'_, T> {
            SharedWriteGuard(self.0.write().unwrap())
        }
    }

    pub struct SharedReadGuard<'s, T: ?Sized>(RwLockReadGuard<'s, T>);

    impl<T: ?Sized> Deref for SharedReadGuard<'_, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: ?Sized> Borrow<T> for SharedReadGuard<'_, T> {
        fn borrow(&self) -> &T {
            &self.0
        }
    }

    pub struct SharedWriteGuard<'s, T: ?Sized>(RwLockWriteGuard<'s, T>);

    impl<T: ?Sized> Deref for SharedWriteGuard<'_, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: ?Sized> DerefMut for SharedWriteGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T: ?Sized> Borrow<T> for SharedWriteGuard<'_, T> {
        fn borrow(&self) -> &T {
            &self.0
        }
    }

    impl<T: ?Sized> BorrowMut<T> for SharedWriteGuard<'_, T> {
        fn borrow_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }
}

pub enum Event {
    Click,
    Hover,
}

pub struct EventConsumed;

pub const fn consume() -> Result<(), EventConsumed> {
    Err(EventConsumed)
}

pub trait HandleEvent {
    fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed>;
}

pub trait Element: HandleEvent {
    fn draw(&self) {
        todo!();
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

pub fn id<T>(ra_fixture_value: T) -> Ident<T> {
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

// TODO: remove `+ Debug`
pub trait BuildElement: Build<Output: Element + Debug> + ResolveAnchors {}
impl<T: Build<Output: Element + Debug> + ResolveAnchors> BuildElement for T {}

pub trait BuildValue<V: ?Sized>: Build<Output: ValueSource<V>> {}
impl<V: ?Sized, T: Build<Output: ValueSource<V>>> BuildValue<V> for T {}

pub trait BuildValueMut<V: ?Sized>: Build<Output: ValueSourceMut<V>> {}
impl<V: ?Sized, T: Build<Output: ValueSourceMut<V>>> BuildValueMut<V> for T {}

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

pub mod elements {
    use super::*;
    use shared::Shared;
    use std::borrow::BorrowMut;
    use std::marker::PhantomData;
    use sww::Color;

    #[derive(Debug)]
    pub struct Fill(pub Color);

    impl Build for Fill {
        type Output = Self;

        fn build(self) -> Self::Output {
            self
        }
    }

    impl ResolveAnchors for Fill {
        type AnchorsSet = ();

        fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
            None
        }

        fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Shared<A::Value>) {}
    }

    impl Element for Fill {}

    impl HandleEvent for Fill {
        fn handle_event(&mut self, _event: &Event) -> Result<(), EventConsumed> {
            Ok(())
        }
    }

    pub fn fill(ra_fixture_color: Color) -> Fill {
        Fill(ra_fixture_color)
    }

    #[derive(Debug)]
    pub enum SplitType {
        Vertical,
        Horizontal,
        Adaptive,
    }

    #[derive(Debug)]
    pub struct Split<Ty, Es> {
        type_: Ty,
        elements: Es,
    }

    impl<Ty: Build, Es: Build> Build for Split<Ty, Es> {
        type Output = Split<Ty::Output, Es::Output>;

        fn build(self) -> Self::Output {
            Split {
                type_: self.type_.build(),
                elements: self.elements.build(),
            }
        }
    }

    impl<Ty: ResolveAnchors, Es: ResolveAnchors> ResolveAnchors for Split<Ty, Es> {
        type AnchorsSet = (Ty::AnchorsSet, Es::AnchorsSet);

        fn get_anchor<_A: Anchor>(&self) -> Option<Shared<_A::Value>> {
            (self.type_.get_anchor::<_A>()).or_else(|| self.elements.get_anchor::<_A>())
        }

        fn resolve_anchor<_A: Anchor>(&mut self, anchor: &Shared<_A::Value>) {
            self.type_.resolve_anchor::<_A>(anchor);
            self.elements.resolve_anchor::<_A>(anchor);
        }
    }

    impl<Ty, A: Element, B: Element> Element for Split<Ty, (A, B)> {}

    impl<Ty, Es: HandleEvent> HandleEvent for Split<Ty, Es> {
        fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
            self.elements.handle_event(event)
        }
    }

    pub fn split<Es: Build>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
        Split {
            type_: SplitType::Adaptive,
            elements: ra_fixture_elements,
        }
    }

    pub fn line<Es: Build>(ra_fixture_elements: Es) -> Split<Ident<SplitType>, Es> {
        Split {
            type_: id(SplitType::Horizontal),
            elements: ra_fixture_elements,
        }
    }

    pub fn column<Es: Build>(ra_fixture_elements: Es) -> Split<Ident<SplitType>, Es> {
        Split {
            type_: id(SplitType::Vertical),
            elements: ra_fixture_elements,
        }
    }

    #[derive(Debug, Build)]
    pub struct Layers<Es>(Es);

    impl<A: Element, B: Element> Element for Layers<(A, B)> {}
    impl<A: Element, B: Element, C: Element> Element for Layers<(A, B, C)> {}

    impl<Es: HandleEvent> HandleEvent for Layers<Es> {
        fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
            self.0.handle_event(event)
        }
    }

    pub fn layers<Es>(ra_fixture_elements: Es) -> Layers<Es> {
        Layers(ra_fixture_elements)
    }

    #[derive(Debug, Build)]
    pub struct Label<Src>(Src);

    impl<Src> HandleEvent for Label<Src> {
        fn handle_event(&mut self, _event: &Event) -> Result<(), EventConsumed> {
            Ok(())
        }
    }

    impl<Src: ValueSource<str>> Element for Label<Src> {}

    pub fn label<Src: BuildValue<str>>(ra_fixture_source: Src) -> Label<Src> {
        Label(ra_fixture_source)
    }

    pub struct OnClickConsume<E, Src, F, V: ?Sized> {
        element: E,
        source: Src,
        f: F,
        type_: PhantomData<V>,
    }

    impl<E: Debug, Src: Debug, F, V> Debug for OnClickConsume<E, Src, F, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("OnClickConsume")
                .field("element", &self.element)
                .field("source", &self.source)
                .finish_non_exhaustive()
        }
    }

    impl<E: Build, Src: BuildValueMut<V>, F, V: ?Sized> Build for OnClickConsume<E, Src, F, V> {
        type Output = OnClickConsume<E::Output, Src::Output, F, V>;

        fn build(self) -> Self::Output {
            OnClickConsume {
                element: self.element.build(),
                source: self.source.build(),
                f: self.f,
                type_: PhantomData,
            }
        }
    }

    impl<E: ResolveAnchors, Src: ResolveAnchors, F, V: ?Sized> ResolveAnchors
        for OnClickConsume<E, Src, F, V>
    {
        type AnchorsSet = (E::AnchorsSet, Src::AnchorsSet);

        fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
            (self.element.get_anchor::<A>()).or_else(|| self.source.get_anchor::<A>())
        }

        fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
            self.element.resolve_anchor::<A>(anchor);
            self.source.resolve_anchor::<A>(anchor);
        }
    }

    impl<E: Element, Src: ValueSourceMut<V>, F: FnMut(&mut V), V: ?Sized> Element
        for OnClickConsume<E, Src, F, V>
    {
    }

    impl<E: HandleEvent, Src: ValueSourceMut<V>, F: FnMut(&mut V), V: ?Sized> HandleEvent
        for OnClickConsume<E, Src, F, V>
    {
        fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
            match event {
                Event::Click => {
                    (self.f)(self.source.value_mut().borrow_mut());
                    consume()
                }

                _ => self.element.handle_event(event),
            }
        }
    }

    pub fn on_click_consume<
        E: BuildElement,
        Src: BuildValueMut<V>,
        F: FnOnce(&mut V),
        V: ?Sized,
    >(
        ra_fixture_element: E,
        ra_fixture_source: Src,
        ra_fixture_f: F,
    ) -> OnClickConsume<E, Src, F, V> {
        OnClickConsume {
            element: ra_fixture_element,
            source: ra_fixture_source,
            f: ra_fixture_f,
            type_: PhantomData,
        }
    }
}

mod values {
    use super::*;
    use std::borrow::Borrow;
    use std::marker::PhantomData;

    mod traits {
        use crate::shared::Shared;
        use crate::shared::SharedReadGuard;
        use crate::shared::SharedWriteGuard;
        use std::borrow::Borrow;
        use std::borrow::BorrowMut;

        pub trait ValueSource<V: ?Sized> {
            type Output<'s>: Borrow<V>
            where
                Self: 's;

            fn value(&self) -> Self::Output<'_>;
        }

        pub trait ValueSourceMut<V: ?Sized> {
            type Output<'s>: BorrowMut<V>
            where
                Self: 's;

            fn value_mut(&mut self) -> Self::Output<'_>;
        }

        impl<V: ?Sized> ValueSource<V> for V {
            type Output<'s> = &'s V where Self: 's;

            fn value(&self) -> Self::Output<'_> {
                self
            }
        }

        impl<V: ?Sized> ValueSource<V> for &V {
            type Output<'s> = &'s V where Self: 's;

            fn value(&self) -> Self::Output<'_> {
                self
            }
        }

        impl<V: ?Sized> ValueSourceMut<V> for V {
            type Output<'s> = &'s mut V where Self: 's;

            fn value_mut(&mut self) -> Self::Output<'_> {
                self
            }
        }

        impl<V: ?Sized> ValueSourceMut<V> for &mut V {
            type Output<'s> = &'s mut V where Self: 's;

            fn value_mut(&mut self) -> Self::Output<'_> {
                self
            }
        }

        impl<V: ?Sized> ValueSource<V> for Shared<V> {
            type Output<'s> = SharedReadGuard<'s, V> where Self: 's;

            fn value(&self) -> Self::Output<'_> {
                self.read()
            }
        }

        impl<V: ?Sized> ValueSourceMut<V> for Shared<V> {
            type Output<'s> = SharedWriteGuard<'s, V> where Self: 's;

            fn value_mut(&mut self) -> Self::Output<'_> {
                self.write()
            }
        }
    }
    pub use traits::*;

    mod anchors {
        use super::*;

        pub struct SetAnchor<A: Anchor>(Shared<A::Value>);

        impl<A: Anchor> Build for SetAnchor<A> {
            type Output = Shared<A::Value>;

            fn build(self) -> Self::Output {
                self.0
            }
        }

        impl<A: Anchor> ResolveAnchors for SetAnchor<A> {
            type AnchorsSet = A;

            fn get_anchor<B: Anchor>(&self) -> Option<Shared<B::Value>> {
                (&self.0 as &dyn Any).downcast_ref().map(Shared::clone)
            }

            fn resolve_anchor<B: Anchor>(&mut self, _anchor: &Shared<B::Value>) {}
        }

        pub fn set<A: Anchor>(value: A::Value) -> SetAnchor<A> {
            SetAnchor(Shared::new(value))
        }

        pub struct GetAnchor<A: Anchor>(Option<Shared<A::Value>>);

        impl<A: Anchor> Build for GetAnchor<A> {
            type Output = Shared<A::Value>;

            fn build(self) -> Self::Output {
                self.0.expect("anchor was not set")
            }
        }

        impl<A: Anchor> ResolveAnchors for GetAnchor<A> {
            type AnchorsSet = ();

            fn get_anchor<B: Anchor>(&self) -> Option<Shared<B::Value>> {
                None
            }

            fn resolve_anchor<B: Anchor>(&mut self, anchor: &Shared<B::Value>) {
                if let Some(anchor) = (anchor as &dyn Any).downcast_ref().map(Shared::clone) {
                    self.0 = Some(anchor);
                }
            }
        }

        pub fn get<A: Anchor>() -> GetAnchor<A> {
            GetAnchor(None)
        }
    }
    pub use anchors::*;

    #[derive(Debug)]
    pub struct Strfy<V, Src> {
        type_: PhantomData<V>,
        source: Src,
    }

    impl<V: ToString, Src: BuildValue<V>> Build for Strfy<V, Src> {
        type Output = Strfy<V, Src::Output>;

        fn build(self) -> Self::Output {
            Strfy {
                type_: PhantomData,
                source: self.source.build(),
            }
        }
    }

    impl<V: ToString, Src: ResolveAnchors> ResolveAnchors for Strfy<V, Src> {
        type AnchorsSet = Src::AnchorsSet;

        fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
            self.source.get_anchor::<A>()
        }

        fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
            self.source.resolve_anchor::<A>(anchor)
        }
    }

    impl<V: ToString, Src: ValueSource<V>> ValueSource<str> for Strfy<V, Src> {
        type Output<'s> = String where Self: 's;

        fn value(&self) -> Self::Output<'_> {
            self.source.value().borrow().to_string()
        }
    }

    pub fn strfy<V: ToString, Src: BuildValue<V>>(ra_fixture_source: Src) -> Strfy<V, Src> {
        Strfy {
            type_: PhantomData,
            source: ra_fixture_source,
        }
    }

    #[derive(Debug, Build)]
    pub struct Sum<A, B>(A, B);

    impl<A: ValueSource<usize>, B: ValueSource<usize>> ValueSource<usize> for Sum<A, B> {
        type Output<'s> = usize where Self: 's;

        fn value(&self) -> Self::Output<'_> {
            self.0.value().borrow() + self.1.value().borrow()
        }
    }

    #[derive(Debug, Build)]
    pub struct Concat<Srcs>(Srcs);

    impl<A: ValueSource<str>, B: ValueSource<str>, C: ValueSource<str>> ValueSource<str>
        for Concat<(A, B, C)>
    {
        type Output<'s> = String where Self: 's;

        fn value(&self) -> Self::Output<'_> {
            let (a, b, c) = &self.0;
            format!(
                "{}{}{}",
                a.value().borrow(),
                b.value().borrow(),
                c.value().borrow(),
            )
        }
    }

    pub fn concat<Srcs>(ra_fixture_sources: Srcs) -> Concat<Srcs> {
        Concat(ra_fixture_sources)
    }
}

use crate as kon;

pub mod prelude {
    pub use crate::elements::*;
    pub use crate::shared::*;
    pub use crate::values::*;
    pub use crate::*;
    pub use sww::Color;
}

pub fn build<T: BuildElement>(mut builder: T) -> T::Output {
    T::AnchorsSet::resolve_anchors(&mut builder);
    builder.build()
}
