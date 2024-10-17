use crate::drawer::DrawPass;
use crate::prelude::Resources;
use crate::shared;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::InvalidateCaches;
use crate::LocationRect;
use crate::ReversedTuple;
use std::collections::BTreeSet;

pub struct Layers<Es> {
    elements: Es,
}

impl<A: Element, B: Element> Element for Layers<(A, B)> {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: LocationRect) {
        let (a, b) = &self.elements;
        a.draw(pass, resources, location);
        b.draw(pass, resources, location);
    }
}

impl<A: Element, B: Element, C: Element> Element for Layers<(A, B, C)> {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: LocationRect) {
        let (a, b, c) = &self.elements;
        a.draw(pass, resources, location);
        b.draw(pass, resources, location);
        c.draw(pass, resources, location);
    }
}

impl<Es: HandleEvent> HandleEvent for Layers<Es>
where
    for<'s> ReversedTuple<&'s Es>: HandleEvent,
{
    fn handle_event(&self, event: &Event) -> EventResult {
        ReversedTuple(&self.elements).handle_event(event)
    }
}

impl<Es: InvalidateCaches> InvalidateCaches for Layers<Es> {
    fn invalidate_caches(&self, addrs: &BTreeSet<shared::Addr>) -> bool {
        self.elements.invalidate_caches(addrs)
    }
}

pub const fn layers<Es>(ra_fixture_elements: Es) -> Layers<Es> {
    Layers {
        elements: ra_fixture_elements,
    }
}
