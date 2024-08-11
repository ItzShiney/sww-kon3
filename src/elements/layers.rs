use crate as kon3;
use crate::Build;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use std::fmt::Debug;

#[derive(Debug, Build)]
pub struct Layers<Es>(Es);

impl<A: Element, B: Element> Element for Layers<(A, B)> {}
impl<A: Element, B: Element, C: Element> Element for Layers<(A, B, C)> {}

impl<Es: HandleEvent> HandleEvent for Layers<Es> {
    fn handle_event(&mut self, event: &Event) -> EventResult {
        self.0.handle_event(event)
    }
}

pub const fn layers<Es>(ra_fixture_elements: Es) -> Layers<Es> {
    Layers(ra_fixture_elements)
}
