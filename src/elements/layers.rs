use crate::drawer::DrawPass;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::Location;

pub struct Layers<Es>(Es);

impl<R, A: Element<R>, B: Element<R>> Element<R> for Layers<(A, B)> {
    fn draw(&self, pass: &mut DrawPass, resources: &R, location: Location) {
        let (a, b) = &self.0;
        a.draw(pass, resources, location);
        b.draw(pass, resources, location);
    }
}

impl<R, A: Element<R>, B: Element<R>, C: Element<R>> Element<R> for Layers<(A, B, C)> {
    fn draw(&self, pass: &mut DrawPass, resources: &R, location: Location) {
        let (a, b, c) = &self.0;
        a.draw(pass, resources, location);
        b.draw(pass, resources, location);
        c.draw(pass, resources, location);
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
