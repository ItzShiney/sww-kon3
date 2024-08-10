use crate::shared::Shared;
use crate::Anchor;
use crate::Build;
use crate::Element;
use crate::Event;
use crate::EventConsumed;
use crate::HandleEvent;
use crate::ResolveAnchors;
use sww::Color;

#[derive(Debug)]
pub struct Rect(pub Color);

impl Build for Rect {
    type Built = Self;

    fn build(self) -> Self::Built {
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
