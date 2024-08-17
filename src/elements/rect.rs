use crate::shared::Shared;
use crate::Anchor;
use crate::Build;
use crate::Drawer;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::Location;
use crate::ResolveAnchors;
use sww::shaders::mesh::Transform;
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

impl Element for Rect {
    fn draw<'e>(&'e self, drawer: &mut Drawer<'e>, location: Location) {
        let rect = location.rect();
        let mesh_drawing_info = todo!();
        let transform =
            Transform::new_diagonal(rect.size, rect.top_left, self.0, Default::default());

        drawer.mesh().draw(mesh_drawing_info, transform);
    }
}

impl HandleEvent for Rect {
    fn handle_event(&mut self, _event: &Event) -> EventResult {
        Ok(())
    }
}

pub const fn rect(ra_fixture_color: Color) -> Rect {
    Rect(ra_fixture_color)
}
