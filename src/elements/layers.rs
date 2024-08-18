use crate as kon3;
use crate::resources::Resources;
use crate::Build;
use crate::Drawer;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::Location;

#[derive(Build)]
pub struct Layers<Es>(Es);

impl<A: Element, B: Element> Element for Layers<(A, B)> {
    fn draw<'e>(&self, drawer: &mut Drawer<'e>, resources: &'e Resources, location: Location) {
        let (a, b) = &self.0;
        b.draw(drawer, resources, location);
        a.draw(drawer, resources, location);
    }
}

impl<A: Element, B: Element, C: Element> Element for Layers<(A, B, C)> {
    fn draw<'e>(&self, drawer: &mut Drawer<'e>, resources: &'e Resources, location: Location) {
        let (a, b, c) = &self.0;
        c.draw(drawer, resources, location);
        b.draw(drawer, resources, location);
        a.draw(drawer, resources, location);
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
