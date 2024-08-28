use crate::drawer::DrawPass;
use crate::prelude::Resources;
use crate::shared;
use crate::values::ValueSourceBorrow;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::InvalidateCache;
use crate::LocationRect;
use std::borrow::Borrow;
use sww::shaders::mesh::Rectangle;
use sww::vec2;

pub struct Label<Src> {
    source: Src,
}

impl<Src> HandleEvent for Label<Src> {
    fn handle_event(&self, _event: &Event) -> EventResult {
        Ok(())
    }
}

// FIXME
impl<Src: ValueSourceBorrow<str>> Element for Label<Src> {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: LocationRect) {
        use super::rect;
        use sww::Color;

        let hash = {
            use std::hash::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let mut hasher = DefaultHasher::default();
            (*self.source.value()).borrow().hash(&mut hasher);
            (hasher.finish() % 16) as usize
        };

        let padding = 1. / hash as f32;
        let location = location.subrect(Rectangle {
            top_left: vec2(padding, 0.),
            size: vec2(padding.mul_add(-2., 1.), 1.),
        });
        rect(Color::new_rgba(1., 1., 1., 0.5)).draw(pass, resources, location);
    }
}

impl<Src: InvalidateCache> InvalidateCache for Label<Src> {
    fn invalidate_cache(&self, addr: shared::Addr) -> bool {
        self.source.invalidate_cache(addr)
    }
}

pub const fn label<Src: ValueSourceBorrow<str>>(ra_fixture_source: Src) -> Label<Src> {
    Label {
        source: ra_fixture_source,
    }
}
