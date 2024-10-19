use crate::app::Signal;
use crate::app::Signaler;
use crate::drawer::resources::Resources;
use crate::drawer::DrawPass;
use crate::values::ValueSourceBorrow;
use crate::ContainsShared;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::LocationRect;
use std::borrow::Borrow;
use sww::shaders::mesh::Rectangle;
use sww::vec2;

pub struct Label<Src> {
    source: Src,
}

impl<Src: ValueSourceBorrow<str>> Element for Label<Src> {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: LocationRect) {
        // FIXME
        use super::rect;
        use sww::Color;

        let hash = {
            use std::hash::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let mut hasher = DefaultHasher::default();
            (*self.source.value()).borrow().hash(&mut hasher);
            (hasher.finish() % 64) as usize
        };
        let hash = hash as f32 / 64.;

        let padding = 1. - hash;
        let location = location.subrect(Rectangle {
            top_left: vec2(padding, 0.),
            size: vec2(padding.mul_add(-2., 1.), 1.),
        });
        rect(Color::new_rgba(1., 1., 1., 0.5)).draw(pass, resources, location);
    }
}

impl<Src: ContainsShared + HandleEvent> HandleEvent for Label<Src> {
    fn handle_event(&self, signaler: &Signaler, event: &Event) -> EventResult {
        match *event {
            Event::SharedUpdated(addr) if self.source.contains_shared(addr) => {
                signaler.send(Signal::Redraw);
            }

            _ => {}
        }

        self.source.handle_event(signaler, event)
    }
}

pub const fn label<Src: ValueSourceBorrow<str>>(ra_fixture_source: Src) -> Label<Src> {
    Label {
        source: ra_fixture_source,
    }
}
