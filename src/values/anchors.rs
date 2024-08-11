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
