use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

type Gc<T> = Arc<RwLock<T>>;

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

impl AnchorsTree for () {
    fn resolve_anchors(_builder: &mut impl ResolveAnchors) {}
}

impl<A: AnchorsTree, B: AnchorsTree> AnchorsTree for (A, B) {
    fn resolve_anchors(builder: &mut impl ResolveAnchors) {
        A::resolve_anchors(builder);
        B::resolve_anchors(builder);
    }
}

impl<A: AnchorsTree, B: AnchorsTree, C: AnchorsTree> AnchorsTree for (A, B, C) {
    fn resolve_anchors(builder: &mut impl ResolveAnchors) {
        A::resolve_anchors(builder);
        B::resolve_anchors(builder);
        C::resolve_anchors(builder);
    }
}

trait Build: ResolveAnchors {
    type Output;

    fn build(self) -> Self::Output;
}

trait ValueSource<V: ?Sized> {
    fn with_get(&self, f: impl FnOnce(&V));
}

trait ValueSourceMut<V: ?Sized> {
    fn with_get_mut(&mut self, f: impl FnOnce(&mut V));
}

impl<V: ?Sized> ValueSource<V> for V {
    fn with_get(&self, f: impl FnOnce(&V)) {
        f(self);
    }
}

impl<V: ?Sized> ValueSourceMut<V> for V {
    fn with_get_mut(&mut self, f: impl FnOnce(&mut V)) {
        f(self);
    }
}

impl<V: ?Sized> ValueSource<V> for Gc<V> {
    fn with_get(&self, f: impl FnOnce(&V)) {
        f(&self.read().unwrap());
    }
}

impl<V: ?Sized> ValueSourceMut<V> for Gc<V> {
    fn with_get_mut(&mut self, f: impl FnOnce(&mut V)) {
        f(&mut self.write().unwrap());
    }
}

struct SetAnchor<A: Anchor>(Gc<A::Value>);

impl<A: Anchor> Debug for SetAnchor<A>
where
    A::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0.read().unwrap() as &A::Value, f)
    }
}

impl<A: Anchor> SetAnchor<A> {
    fn new(value: A::Value) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }
}

impl<A: Anchor> Build for SetAnchor<A> {
    type Output = Gc<A::Value>;

    fn build(self) -> Self::Output {
        self.0
    }
}

impl<A: Anchor> ResolveAnchors for SetAnchor<A> {
    type AnchorsSet = A;

    fn get_anchor<B: Anchor>(&self) -> Option<Gc<B::Value>> {
        (&self.0 as &dyn Any).downcast_ref().map(Gc::clone)
    }

    fn resolve_anchor<B: Anchor>(&mut self, _anchor: &Gc<B::Value>) {}
}

struct GetAnchor<A: Anchor>(Option<Gc<A::Value>>);

impl<A: Anchor> Debug for GetAnchor<A>
where
    A::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => Debug::fmt(&value.read().unwrap() as &A::Value, f),
            None => Debug::fmt("None", f),
        }
    }
}

impl<A: Anchor> GetAnchor<A> {
    fn new() -> Self {
        Self(None)
    }
}

impl<A: Anchor> Build for GetAnchor<A> {
    type Output = Gc<A::Value>;

    fn build(self) -> Self::Output {
        self.0.expect("anchor was not set")
    }
}

impl<A: Anchor> ResolveAnchors for GetAnchor<A> {
    type AnchorsSet = ();

    fn get_anchor<B: Anchor>(&self) -> Option<Gc<B::Value>> {
        None
    }

    fn resolve_anchor<B: Anchor>(&mut self, anchor: &Gc<B::Value>) {
        if let Some(anchor) = (anchor as &dyn Any).downcast_ref().map(Gc::clone) {
            self.0 = Some(anchor);
        }
    }
}

trait ResolveAnchors {
    type AnchorsSet: AnchorsTree;

    fn get_anchor<A: Anchor>(&self) -> Option<Gc<A::Value>>;
    fn resolve_anchor<A: Anchor>(&mut self, anchor: &Gc<A::Value>);
}

#[derive(Debug)]
struct LeafBuilder;

impl Build for LeafBuilder {
    type Output = LeafBuilder;

    fn build(self) -> Self::Output {
        LeafBuilder
    }
}

impl ResolveAnchors for LeafBuilder {
    type AnchorsSet = ();

    fn get_anchor<A: Anchor>(&self) -> Option<Gc<A::Value>> {
        None
    }

    fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Gc<A::Value>) {}
}

#[derive(Debug)]
struct SomeBuilderWithAnchor<T, A>(T, A);

impl<T: Build, A: Build> Build for SomeBuilderWithAnchor<T, A>
where
    A::Output: ValueSource<usize>,
{
    type Output = SomeBuilderWithAnchor<T::Output, A::Output>;

    fn build(self) -> Self::Output {
        SomeBuilderWithAnchor(self.0.build(), self.1.build())
    }
}

impl<T: ResolveAnchors, A: Build> ResolveAnchors for SomeBuilderWithAnchor<T, A>
where
    A::Output: ValueSource<usize>,
{
    type AnchorsSet = (T::AnchorsSet, A::AnchorsSet);

    fn get_anchor<B: Anchor>(&self) -> Option<Gc<B::Value>> {
        (self.1.get_anchor::<B>()).or_else(|| self.0.get_anchor::<B>())
    }

    fn resolve_anchor<B: Anchor>(&mut self, anchor: &Gc<B::Value>) {
        self.0.resolve_anchor::<B>(anchor);
        self.1.resolve_anchor::<B>(anchor);
    }
}

#[derive(Debug)]
struct SomeBuilder<T>(T);

impl<T: Build> Build for SomeBuilder<T> {
    type Output = SomeBuilder<T::Output>;

    fn build(self) -> Self::Output {
        SomeBuilder(self.0.build())
    }
}

impl<T: ResolveAnchors> ResolveAnchors for SomeBuilder<T> {
    type AnchorsSet = T::AnchorsSet;

    fn get_anchor<B: Anchor>(&self) -> Option<Gc<B::Value>> {
        self.0.get_anchor::<B>()
    }

    fn resolve_anchor<B: Anchor>(&mut self, anchor: &Gc<B::Value>) {
        self.0.resolve_anchor::<B>(anchor)
    }
}

#[derive(Debug)]
struct SomeBuilder2<T, U>(T, U);

impl<T: Build, U: Build> Build for SomeBuilder2<T, U> {
    type Output = SomeBuilder2<T::Output, U::Output>;

    fn build(self) -> Self::Output {
        SomeBuilder2(self.0.build(), self.1.build())
    }
}

impl<T: ResolveAnchors, U: ResolveAnchors> ResolveAnchors for SomeBuilder2<T, U> {
    type AnchorsSet = (T::AnchorsSet, U::AnchorsSet);

    fn get_anchor<B: Anchor>(&self) -> Option<Gc<B::Value>> {
        (self.0.get_anchor::<B>()).or_else(|| self.1.get_anchor::<B>())
    }

    fn resolve_anchor<B: Anchor>(&mut self, anchor: &Gc<B::Value>) {
        self.0.resolve_anchor::<B>(anchor);
        self.1.resolve_anchor::<B>(anchor);
    }
}

#[derive(Debug)]
struct Sum<A, B>(A, B);

impl<A: Build, B: Build> Build for Sum<A, B>
where
    A::Output: ValueSource<usize>,
    B::Output: ValueSource<usize>,
{
    type Output = Sum<A::Output, B::Output>;

    fn build(self) -> Self::Output {
        Sum(self.0.build(), self.1.build())
    }
}

impl<A: Build, B: Build> ResolveAnchors for Sum<A, B>
where
    A::Output: ValueSource<usize>,
    B::Output: ValueSource<usize>,
{
    type AnchorsSet = (A::AnchorsSet, B::AnchorsSet);

    fn get_anchor<C: Anchor>(&self) -> Option<Gc<C::Value>> {
        (self.0.get_anchor::<C>()).or_else(|| self.1.get_anchor::<C>())
    }

    fn resolve_anchor<C: Anchor>(&mut self, anchor: &Gc<C::Value>) {
        self.0.resolve_anchor::<C>(anchor);
        self.1.resolve_anchor::<C>(anchor);
    }
}

impl<A: ValueSource<usize>, B: ValueSource<usize>> ValueSource<usize> for Sum<A, B> {
    fn with_get(&self, f: impl FnOnce(&usize)) {
        (self.0).with_get(move |&a| (self.1).with_get(move |&b| f(&(a + b))))
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

    let builder = build(SomeBuilder2(
        SomeBuilder(SomeBuilderWithAnchor(
            LeafBuilder,
            SetAnchor::<MyAnchor>::new(1),
        )),
        SomeBuilderWithAnchor(
            LeafBuilder,
            Sum(GetAnchor::<MyAnchor>::new(), GetAnchor::<MyAnchor>::new()),
        ),
    ));

    println!("{:#?}", builder);
}
