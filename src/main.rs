use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

trait Anchor: 'static {
    type Value;
}

type Gc<T> = Arc<RwLock<T>>;

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

trait UseAnchor: ResolveAnchors {
    type Anchor: Anchor;

    fn unwrap(self) -> Gc<<Self::Anchor as Anchor>::Value>;
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

impl<A: Anchor> UseAnchor for SetAnchor<A> {
    type Anchor = A;

    fn unwrap(self) -> Gc<A::Value> {
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

impl<A: Anchor> UseAnchor for GetAnchor<A> {
    type Anchor = A;

    fn unwrap(self) -> Gc<A::Value> {
        self.0.unwrap()
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
impl ResolveAnchors for LeafBuilder {
    type AnchorsSet = ();

    fn get_anchor<A: Anchor>(&self) -> Option<Gc<A::Value>> {
        None
    }

    fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Gc<A::Value>) {}
}

#[derive(Debug)]
struct SomeBuilderWithAnchor<T: ResolveAnchors, A: UseAnchor>(T, A);
impl<T: ResolveAnchors, A: UseAnchor> ResolveAnchors for SomeBuilderWithAnchor<T, A> {
    type AnchorsSet = (T::AnchorsSet, A::Anchor);

    fn get_anchor<B: Anchor>(&self) -> Option<Gc<B::Value>> {
        self.1
            .get_anchor::<B>()
            .or_else(|| self.0.get_anchor::<B>())
    }

    fn resolve_anchor<B: Anchor>(&mut self, anchor: &Gc<B::Value>) {
        self.0.resolve_anchor::<B>(anchor);
        self.1.resolve_anchor::<B>(anchor);
    }
}

#[derive(Debug)]
struct SomeBuilder<T: ResolveAnchors>(T);
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
struct SomeBuilder2<T: ResolveAnchors, U: ResolveAnchors>(T, U);
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

fn resolve_anchors<T: ResolveAnchors>(builder: &mut T) {
    T::AnchorsSet::resolve_anchors(builder);
}

fn main() {
    struct MyAnchor;
    impl Anchor for MyAnchor {
        type Value = usize;
    }

    let mut builder = SomeBuilder2(
        SomeBuilder(SomeBuilderWithAnchor(
            LeafBuilder,
            SetAnchor::<MyAnchor>::new(1),
        )),
        SomeBuilderWithAnchor(LeafBuilder, GetAnchor::<MyAnchor>::new()),
    );
    resolve_anchors(&mut builder);

    println!("{:#?}", builder);
}
