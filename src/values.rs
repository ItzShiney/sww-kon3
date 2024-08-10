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
        type Built = Self;

        fn build(self) -> Self::Built {
            self
        }
    }

    impl<T: AutoValueSource> ResolveAnchors for T {
        type AnchorsSet = ();

        fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
            None
        }

        fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Shared<A::Value>) {}
    }

    impl ValueSource for &str {
        type Value = str;

        fn value(&self) -> SourcedValue<'_, Self::Value> {
            SourcedValue::Ref(self)
        }
    }

    impl Build for &str {
        type Built = Self;

        fn build(self) -> Self::Built {
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
        type Built = Shared<A::Value>;

        fn build(self) -> Self::Built {
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
        type Built = Shared<A::Value>;

        fn build(self) -> Self::Built {
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
    type Built = Strfy<Src::Built, Cch::Built>;

    fn build(self) -> Self::Built {
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

pub fn strfy<V: ToString, Src: Build<Built: ValueSource<Value = V>>>(
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
    type Built = Concat<Src::Built, Cch::Built>;

    fn build(self) -> Self::Built {
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
