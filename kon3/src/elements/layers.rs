use crate::app::SignalSender;
use crate::drawer::resources::Resources;
use crate::drawer::DrawPass;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::LocationRect;
use crate::ReversedTuple;

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
    fn handle_event(&self, signal_sender: &SignalSender, event: &Event) -> EventResult {
        ReversedTuple(&self.elements).handle_event(signal_sender, event)
    }
}

pub const fn layers<Es>(ra_fixture_elements: Es) -> Layers<Es> {
    Layers {
        elements: ra_fixture_elements,
    }
}
