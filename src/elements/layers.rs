use crate as kon3;
use crate::Build;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;

#[derive(Build)]
pub struct Layers<Es>(Es);

impl<A: Element, B: Element> Element for Layers<(A, B)> {
    fn draw<'e>(&'e self, drawer: &mut crate::Drawer<'e>, location: crate::Location) {
        let (a, b) = &self.0;
        b.draw(drawer, location);
        a.draw(drawer, location);
    }
}

impl<A: Element, B: Element, C: Element> Element for Layers<(A, B, C)> {
    fn draw<'e>(&'e self, drawer: &mut crate::Drawer<'e>, location: crate::Location) {
        let (a, b, c) = &self.0;
        c.draw(drawer, location);
        b.draw(drawer, location);
        a.draw(drawer, location);
    }
}

impl<Es: HandleEvent> HandleEvent for Layers<Es> {
    fn handle_event(&mut self, event: &Event) -> EventResult {
        self.0.handle_event(event)
    }
}

pub const fn layers<Es>(ra_fixture_elements: Es) -> Layers<Es> {
    Layers(ra_fixture_elements)
}
