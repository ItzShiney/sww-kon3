use kon_macros::Build;
use std::any::Any;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::fmt::Debug;

mod shared {
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

use shared::*;

enum Event {
    Update,
}

struct EventConsumed;

trait HandleEvent {
    fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed>;
}

trait Element: HandleEvent {
    fn draw(&self)
    where
        Self: Debug,
    {
        println!("{:#?}", self);
    }
}

trait Anchor: 'static {
    type Value;
}

trait AnchorsTree {
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

trait Build: ResolveAnchors {
    type Output;

    fn build(self) -> Self::Output;
}

macro_rules! tuple_impls {
    ( $($T:ident)+ ) => {
        impl<$($T: Build),+> Build for ($($T),+)
        where
            Self: ResolveAnchors,
        {
            type Output = ($($T::Output),+);

            fn build(self) -> Self::Output {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                ($($T.build()),+)
            }
        }

        impl<$($T: Build),+> ResolveAnchors for ($($T),+) {
            type AnchorsSet = ($($T::AnchorsSet),+);

            fn get_anchor<_A: Anchor>(&self) -> Option<Shared<_A::Value>> {
                #[allow(non_snake_case)]
                let ($($T),+) = self;

                None $( .or_else(|| $T.get_anchor::<_A>()) )+
            }

            fn resolve_anchor<_A: Anchor>(&mut self, anchor: &Shared<_A::Value>) {
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

trait GetValue<V: ?Sized> {
    type Output<'s>: Borrow<V> + 's
    where
        Self: 's;

    fn value(&self) -> Self::Output<'_>;
}

trait GetValueMut<V: ?Sized> {
    type Output<'s>: BorrowMut<V> + 's
    where
        Self: 's;

    fn value_mut(&mut self) -> Self::Output<'_>;
}

impl<V: ?Sized> GetValue<V> for V {
    type Output<'s> = &'s V where V: 's;

    fn value(&self) -> Self::Output<'_> {
        self
    }
}

impl<V: ?Sized> GetValueMut<V> for V {
    type Output<'s> = &'s mut V where V: 's;

    fn value_mut(&mut self) -> Self::Output<'_> {
        self
    }
}

impl<V: ?Sized> GetValue<V> for Shared<V> {
    type Output<'s> = SharedReadGuard<'s, V> where V: 's;

    fn value(&self) -> Self::Output<'_> {
        self.read()
    }
}

impl<V: ?Sized> GetValueMut<V> for Shared<V> {
    type Output<'s> = SharedWriteGuard<'s, V> where V: 's;

    fn value_mut(&mut self) -> Self::Output<'_> {
        self.write()
    }
}

struct SetAnchor<A: Anchor>(Shared<A::Value>);

impl<A: Anchor> SetAnchor<A> {
    fn new(value: A::Value) -> Self {
        Self(Shared::new(value))
    }
}

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

struct GetAnchor<A: Anchor>(Option<Shared<A::Value>>);

impl<A: Anchor> GetAnchor<A> {
    fn new() -> Self {
        Self(None)
    }
}

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

trait ResolveAnchors {
    type AnchorsSet: AnchorsTree;

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>>;
    fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>);
}

#[derive(Debug)]
struct LeafElement;

impl Build for LeafElement {
    type Output = LeafElement;

    fn build(self) -> Self::Output {
        LeafElement
    }
}

impl ResolveAnchors for LeafElement {
    type AnchorsSet = ();

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        None
    }

    fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Shared<A::Value>) {}
}

impl Element for LeafElement {}

impl HandleEvent for LeafElement {
    fn handle_event(&mut self, _event: &Event) -> Result<(), EventConsumed> {
        Ok(())
    }
}

#[derive(Debug, Build)]
struct SomeElementWithAnchor<T, A>(T, A);

impl<T: Element, A: GetValue<usize>> Element for SomeElementWithAnchor<T, A> {}

impl<T: Element, A: GetValue<usize>> HandleEvent for SomeElementWithAnchor<T, A> {
    fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
        self.0.handle_event(event)
    }
}

#[derive(Debug, Build)]
struct SomeElement<T>(T);

impl<T: Element> Element for SomeElement<T> {}

impl<T: Element> HandleEvent for SomeElement<T> {
    fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
        self.0.handle_event(event)
    }
}

#[derive(Debug, Build)]
struct SomeElement2<T, U>(T, U);

impl<T: Element, U: Element> Element for SomeElement2<T, U> {}

impl<T: Element, U: Element> HandleEvent for SomeElement2<T, U> {
    fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
        self.0.handle_event(event)?;
        self.1.handle_event(event)
    }
}

#[derive(Debug, Build)]
struct Sum<A, B>(A, B);

impl<A: GetValue<usize>, B: GetValue<usize>> GetValue<usize> for Sum<A, B> {
    type Output<'s> = usize where A: 's, B: 's;

    fn value(&self) -> Self::Output<'_> {
        self.0.value().borrow() + self.1.value().borrow()
    }
}

fn build<T: Build>(mut builder: T) -> T::Output {
    T::AnchorsSet::resolve_anchors(&mut builder);
    builder.build()
}

fn main() {
    struct MyAnchor;
    impl Anchor for MyAnchor {
        type Value = usize;
    }

    let mut ui = build(SomeElement2(
        SomeElement(SomeElementWithAnchor(
            LeafElement,
            SetAnchor::<MyAnchor>::new(1),
        )),
        SomeElementWithAnchor(
            LeafElement,
            Sum(GetAnchor::<MyAnchor>::new(), GetAnchor::<MyAnchor>::new()),
        ),
    ));

    ui.draw();
    _ = ui.handle_event(&Event::Update);
    ui.draw();
}
