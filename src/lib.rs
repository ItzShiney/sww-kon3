pub use kon3_macros::*;
use shared::Shared;
use std::cell::RefCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use sww::wgpu;

pub mod shared {
    use std::borrow::Borrow;
    use std::borrow::BorrowMut;
    use std::fmt;
    use std::fmt::Debug;
    use std::ops::Deref;
    use std::ops::DerefMut;
    use std::sync::Arc;
    use std::sync::RwLock;
    use std::sync::RwLockReadGuard;
    use std::sync::RwLockWriteGuard;

    pub struct Shared<T: ?Sized>(Arc<RwLock<T>>);

    impl<T: Debug> Debug for Shared<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("Shared").field(&*self.read()).finish()
        }
    }

    impl<T: ?Sized> Clone for Shared<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }

    impl<T> Shared<T> {
        pub fn new(value: T) -> Self {
            Self(Arc::new(RwLock::new(value)))
        }
    }

    impl<T: ?Sized> Shared<T> {
        pub fn read(&self) -> SharedReadGuard<'_, T> {
            SharedReadGuard(self.0.read().expect("shared value was already locked"))
        }

        pub fn write(&self) -> SharedWriteGuard<'_, T> {
            SharedWriteGuard(self.0.write().expect("shared value was already locked"))
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

mod location {
    use sww::shaders::mesh::Rectangle;
    use sww::vec2;
    use sww::window::event::PhysicalSize;

    pub struct Location {
        rect: Rectangle,
        window_size: PhysicalSize,
    }

    impl Location {
        pub const fn new(window_size: PhysicalSize) -> Self {
            Self {
                rect: Rectangle {
                    top_left: vec2(-1., -1.),
                    size: vec2(2., 2.),
                },
                window_size,
            }
        }

        pub const fn rect(&self) -> Rectangle {
            self.rect
        }

        pub const fn window_size(&self) -> PhysicalSize {
            self.window_size
        }

        pub fn window_rect_size(&self) -> PhysicalSize {
            let rect_size = self.rect.size * 0.5;
            PhysicalSize::new(
                (rect_size.x * self.window_size.width as f32).round() as _,
                (rect_size.y * self.window_size.height as f32).round() as _,
            )
        }

        pub fn subrect(self, rect: Rectangle) -> Self {
            Self {
                rect: self.rect.subrect(rect),
                window_size: self.window_size,
            }
        }
    }
}
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

pub mod elements {
    use crate as kon3;
    use crate::consume;
    use crate::id;
    use crate::shared::Shared;
    use crate::values::ArgsSource;
    use crate::values::ValueSource;
    use crate::Anchor;
    use crate::Build;
    use crate::BuildElement;
    use crate::Element;
    use crate::Event;
    use crate::EventConsumed;
    use crate::HandleEvent;
    use crate::Ident;
    use crate::ResolveAnchors;
    use std::fmt;
    use std::fmt::Debug;
    use sww::Color;

    #[derive(Debug)]
    pub struct Rect(pub Color);

    impl Build for Rect {
        type Output = Self;

        fn build(self) -> Self::Output {
            self
        }
    }

    impl ResolveAnchors for Rect {
        type AnchorsSet = ();

        fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
            None
        }

        fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Shared<A::Value>) {}
    }

    impl Element for Rect {}

    impl HandleEvent for Rect {
        fn handle_event(&mut self, _event: &Event) -> Result<(), EventConsumed> {
            Ok(())
        }
    }

    pub const fn rect(ra_fixture_color: Color) -> Rect {
        Rect(ra_fixture_color)
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

    pub const fn split<Es: Build>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
        Split {
            type_: SplitType::Adaptive,
            elements: ra_fixture_elements,
        }
    }

    pub const fn line<Es: Build>(ra_fixture_elements: Es) -> Split<Ident<SplitType>, Es> {
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

    pub const fn layers<Es>(ra_fixture_elements: Es) -> Layers<Es> {
        Layers(ra_fixture_elements)
    }

    #[derive(Debug, Build)]
    pub struct Label<Src>(Src);

    impl<Src> HandleEvent for Label<Src> {
        fn handle_event(&mut self, _event: &Event) -> Result<(), EventConsumed> {
            Ok(())
        }
    }

    impl<Src: ValueSource<Value = str>> Element for Label<Src> {}

    pub const fn label<Src: Build<Output: ValueSource<Value = str>>>(
        ra_fixture_source: Src,
    ) -> Label<Src> {
        Label(ra_fixture_source)
    }

    pub struct OnClickConsume<E, Src, F> {
        element: E,
        source: Src,
        f: F,
    }

    impl<E: Debug, Src: Debug, F> Debug for OnClickConsume<E, Src, F> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("OnClickConsume")
                .field("element", &self.element)
                .field("source", &self.source)
                .finish_non_exhaustive()
        }
    }

    impl<E: Build, Src: Build, F> Build for OnClickConsume<E, Src, F> {
        type Output = OnClickConsume<E::Output, Src::Output, F>;

        fn build(self) -> Self::Output {
            OnClickConsume {
                element: self.element.build(),
                source: self.source.build(),
                f: self.f,
            }
        }
    }

    impl<E: ResolveAnchors, Src: ResolveAnchors, F> ResolveAnchors for OnClickConsume<E, Src, F> {
        type AnchorsSet = (E::AnchorsSet, Src::AnchorsSet);

        fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
            (self.element.get_anchor::<A>()).or_else(|| self.source.get_anchor::<A>())
        }

        fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
            self.element.resolve_anchor::<A>(anchor);
            self.source.resolve_anchor::<A>(anchor);
        }
    }

    // impl<E: Element, Src, F> Element for OnClickConsume<E, Src, F> where Self: HandleEvent {}
    impl<E: Element, Src: ArgsSource> Element for OnClickConsume<E, Src, Src::Fn> {}

    impl<E: HandleEvent, Src: ArgsSource> HandleEvent for OnClickConsume<E, Src, Src::Fn> {
        fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
            match event {
                Event::Click => {
                    self.source.apply_to(&self.f);
                    consume()
                }

                _ => self.element.handle_event(event),
            }
        }
    }

    pub const fn on_click_consume<
        E: BuildElement,
        Src: Build<Output = ArgSrc>,
        ArgSrc: ArgsSource,
    >(
        ra_fixture_element: E,
        ra_fixture_source: Src,
        ra_fixture_f: ArgSrc::Fn,
    ) -> OnClickConsume<E, Src, ArgSrc::Fn> {
        OnClickConsume {
            element: ra_fixture_element,
            source: ra_fixture_source,
            f: ra_fixture_f,
        }
    }
}

mod values {
    use crate as kon3;
    use crate::cache;
    use crate::shared::Shared;
    use crate::Anchor;
    use crate::Build;
    use crate::Cache;
    use crate::Cached;
    use crate::ResolveAnchors;
    use std::ops;

    mod traits {
        use crate::shared::SharedReadGuard;
        use crate::shared::SharedWriteGuard;
        use crate::Anchor;
        use crate::Build;
        use crate::ResolveAnchors;
        use crate::Shared;
        use std::borrow::Borrow;
        use std::cell;
        use std::ops::Deref;
        use std::ops::DerefMut;

        pub enum SourcedValue<'s, T: ToOwned + ?Sized> {
            Ref(&'s T),
            Guard(SharedReadGuard<'s, T>),
            Cached(cell::Ref<'s, Option<T::Owned>>),
        }

        impl<'s, T: ToOwned + ?Sized> Deref for SourcedValue<'s, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                match self {
                    Self::Ref(value) => value,
                    Self::Cached(value) => value.as_ref().expect("value was not cached").borrow(),
                    Self::Guard(value) => value,
                }
            }
        }

        pub enum SourcedValueMut<'s, T: ?Sized> {
            Ref(&'s mut T),
            Guard(SharedWriteGuard<'s, T>),
        }

        impl<T: ?Sized> Deref for SourcedValueMut<'_, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                match self {
                    Self::Ref(value) => value,
                    Self::Guard(value) => value,
                }
            }
        }

        impl<T: ?Sized> DerefMut for SourcedValueMut<'_, T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                match self {
                    Self::Ref(value) => value,
                    Self::Guard(value) => value,
                }
            }
        }

        impl<'s, T: ToOwned + ?Sized> From<cell::Ref<'s, Option<T::Owned>>> for SourcedValue<'s, T> {
            fn from(value: cell::Ref<'s, Option<T::Owned>>) -> Self {
                Self::Cached(value)
            }
        }

        impl<'s, T: ToOwned + ?Sized> From<SharedReadGuard<'s, T>> for SourcedValue<'s, T> {
            fn from(value: SharedReadGuard<'s, T>) -> Self {
                Self::Guard(value)
            }
        }

        impl<'s, T: ?Sized> From<SharedWriteGuard<'s, T>> for SourcedValueMut<'s, T> {
            fn from(value: SharedWriteGuard<'s, T>) -> Self {
                Self::Guard(value)
            }
        }

        pub trait ValueSource {
            type Value: ToOwned + ?Sized;

            fn value(&self) -> SourcedValue<'_, Self::Value>;
        }

        pub trait ValueSourceMut: ValueSource {
            fn value_mut(&mut self) -> SourcedValueMut<'_, Self::Value>;
        }

        pub trait AutoValueSource: ToOwned {}

        impl AutoValueSource for u8 {}
        impl AutoValueSource for u16 {}
        impl AutoValueSource for u32 {}
        impl AutoValueSource for u64 {}
        impl AutoValueSource for u128 {}
        impl AutoValueSource for usize {}
        impl AutoValueSource for i8 {}
        impl AutoValueSource for i16 {}
        impl AutoValueSource for i32 {}
        impl AutoValueSource for i64 {}
        impl AutoValueSource for i128 {}
        impl AutoValueSource for isize {}

        impl<T: AutoValueSource> ValueSource for T {
            type Value = T;

            fn value(&self) -> SourcedValue<'_, T> {
                SourcedValue::Ref(self)
            }
        }

        impl<T: AutoValueSource> ValueSourceMut for T {
            fn value_mut(&mut self) -> SourcedValueMut<'_, T> {
                SourcedValueMut::Ref(self)
            }
        }

        impl<T: AutoValueSource> Build for T {
            type Output = Self;

            fn build(self) -> Self::Output {
                self
            }
        }

        impl ValueSource for &str {
            type Value = str;

            fn value(&self) -> SourcedValue<'_, Self::Value> {
                SourcedValue::Ref(self)
            }
        }

        impl Build for &str {
            type Output = Self;

            fn build(self) -> Self::Output {
                self
            }
        }

        impl ResolveAnchors for &str {
            type AnchorsSet = ();

            fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
                None
            }

            fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Shared<A::Value>) {}
        }

        impl<T: ToOwned + ?Sized> ValueSource for Shared<T> {
            type Value = T;

            fn value(&self) -> SourcedValue<'_, Self::Value> {
                SourcedValue::Guard(self.read())
            }
        }

        impl<T: ?Sized> ValueSourceMut for Shared<T>
        where
            Self: ValueSource<Value = T>,
        {
            fn value_mut(&mut self) -> SourcedValueMut<'_, Self::Value> {
                SourcedValueMut::Guard(self.write())
            }
        }
    }
    pub use traits::*;

    mod anchors {
        use crate::shared::Shared;
        use crate::Anchor;
        use crate::Build;
        use crate::ResolveAnchors;
        use std::any::Any;

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
                <dyn Any>::downcast_ref(&self.0).cloned()
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
                if let Some(anchor) = <dyn Any>::downcast_ref(anchor).cloned() {
                    self.0 = Some(anchor);
                }
            }
        }

        pub const fn get<A: Anchor>() -> GetAnchor<A> {
            GetAnchor(None)
        }
    }
    pub use anchors::*;

    #[derive(Debug, Build)]
    pub struct Write<T>(T);

    pub const fn write<T>(ra_fixture_source: T) -> Write<T> {
        Write(ra_fixture_source)
    }

    pub trait ArgsSource {
        type Fn;

        fn apply_to(&self, f: &Self::Fn);
    }

    impl<T: ?Sized> ArgsSource for Shared<T> {
        type Fn = fn(&T);

        fn apply_to(&self, f: &Self::Fn) {
            f(&self.read());
        }
    }

    impl<T: ?Sized> ArgsSource for Write<Shared<T>> {
        type Fn = fn(&mut T);

        fn apply_to(&self, f: &Self::Fn) {
            f(&mut self.0.write());
        }
    }

    #[derive(Debug)]
    pub struct Strfy<Src, Cch> {
        source: Src,
        cache: Cch,
    }

    impl<Src: Build, Cch: Build> Build for Strfy<Src, Cch> {
        type Output = Strfy<Src::Output, Cch::Output>;

        fn build(self) -> Self::Output {
            Strfy {
                source: self.source.build(),
                cache: self.cache.build(),
            }
        }
    }

    impl<Src: ResolveAnchors, Cch> ResolveAnchors for Strfy<Src, Cch> {
        type AnchorsSet = Src::AnchorsSet;

        fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
            self.source.get_anchor::<A>()
        }

        fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
            self.source.resolve_anchor::<A>(anchor);
        }
    }

    impl<Src: ValueSource<Value: ToString>> ValueSource for Strfy<Src, Cached<String>> {
        type Value = str;

        fn value(&self) -> SourcedValue<'_, Self::Value> {
            {
                let mut value = self.cache.borrow_mut();
                if value.is_none() {
                    *value = Some(self.source.value().to_string());
                }
            }
            self.cache.borrow().into()
        }
    }

    pub fn strfy<V: ToString, Src: Build<Output: ValueSource<Value = V>>>(
        ra_fixture_source: Src,
    ) -> Strfy<Src, Cache<String>> {
        Strfy {
            source: ra_fixture_source,
            cache: cache(),
        }
    }

    #[derive(Debug, Build)]
    pub struct Sum<A, B>(A, B);

    impl<A: ValueSource, B: ValueSource, R: ToOwned> ValueSource for Sum<A, B>
    where
        for<'s> &'s A::Value: ops::Add<&'s B::Value, Output = R>,
        for<'s> SourcedValue<'s, R>: From<R>,
    {
        type Value = R;

        fn value(&self) -> SourcedValue<'_, Self::Value> {
            (&*self.0.value() + &*self.1.value()).into()
        }
    }

    #[derive(Debug)]
    pub struct Concat<Src, Cch> {
        sources: Src,
        cache: Cch,
    }

    impl<Src: Build, Cch: Build> Build for Concat<Src, Cch> {
        type Output = Concat<Src::Output, Cch::Output>;

        fn build(self) -> Self::Output {
            Concat {
                sources: self.sources.build(),
                cache: self.cache.build(),
            }
        }
    }

    impl<Src: ResolveAnchors, Cch> ResolveAnchors for Concat<Src, Cch> {
        type AnchorsSet = Src::AnchorsSet;

        fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
            self.sources.get_anchor::<A>()
        }

        fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
            self.sources.resolve_anchor::<A>(anchor);
        }
    }

    impl<A: ValueSource<Value = str>, B: ValueSource<Value = str>, C: ValueSource<Value = str>>
        ValueSource for Concat<(A, B, C), Cached<String>>
    {
        type Value = str;

        fn value(&self) -> SourcedValue<'_, Self::Value> {
            todo!()
        }
    }

    pub const fn concat<Srcs>(ra_fixture_sources: Srcs) -> Concat<Srcs, Cache<String>> {
        Concat {
            sources: ra_fixture_sources,
            cache: cache(),
        }
    }
}

pub mod prelude {
    pub use crate::elements::*;
    pub use crate::shared::*;
    pub use crate::values::*;
    pub use crate::*;
    pub use sww::window::DefaultRenderWindowSettings;
    pub use sww::window::RenderWindowSettings;
    pub use sww::Color;
}

pub mod app {
    use crate::AnchorsTree;
    use crate::BuildElement;
    use crate::Element;
    use crate::Location;
    use sww::app::App as AppRaw;
    use sww::app::AppPack;
    use sww::app::EventInfo;
    use sww::app::HandleEvent;
    use sww::wgpu;
    use sww::window::event::ActiveEventLoop;
    use sww::window::event::EventLoopError;
    use sww::window::event_loop;
    use sww::window::rw_builder;
    use sww::window::window_attributes;
    use sww::window::DefaultRenderWindowSettings;
    use sww::window::RenderWindow;
    use sww::window::RenderWindowSettings;

    pub struct App<F: FnOnce(&ActiveEventLoop) -> AppPack>(AppRaw<F>);

    pub fn build_settings<B: BuildElement<Output: 'static>>(
        mut ui_builder: B,
        settings: &impl RenderWindowSettings,
    ) -> App<impl FnOnce(&ActiveEventLoop) -> AppPack + '_> {
        B::AnchorsSet::resolve_anchors(&mut ui_builder);
        let ui = ui_builder.build();

        App(AppRaw::new(move |event_loop| {
            let window = event_loop
                .create_window(window_attributes("che6", 400, 200))
                .expect("failed to create window");

            AppPack::new(window, rw_builder(settings), move |rw| {
                Box::new(EventHandler { rw, ui })
            })
        }))
    }

    pub fn build<B: BuildElement<Output: 'static>>(
        ui_builder: B,
    ) -> App<impl FnOnce(&ActiveEventLoop) -> AppPack> {
        build_settings(ui_builder, &DefaultRenderWindowSettings)
    }

    impl<F: FnOnce(&ActiveEventLoop) -> AppPack> App<F> {
        pub fn run(&mut self) -> Result<(), EventLoopError> {
            event_loop().run_app(&mut self.0)
        }
    }

    struct EventHandler<'w, E: Element> {
        rw: &'w RenderWindow<'w>,
        ui: E,
    }

    impl<E: Element> HandleEvent for EventHandler<'_, E> {
        fn on_redraw_requested(&mut self, info: EventInfo) {
            let mut frame = self.rw.start_drawing();
            let mut render_pass =
                frame
                    .commands
                    .encoder()
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: frame.surface.view(),
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        ..Default::default()
                    });

            let window_size = info.window.inner_size();
            let location = Location::new(window_size);
            self.ui.draw(&mut render_pass, location);
        }
    }
}
