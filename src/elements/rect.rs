use crate::resources::tags::mesh::DefaultTexture;
use crate::resources::tags::mesh::NoTransform;
use crate::resources::tags::mesh::UnitSquareTopLeft;
use crate::resources::Resources;
use crate::Drawer;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::Location;
use crate::MeshDrawingInfo;
use sww::shaders::mesh::BindGroups;
use sww::shaders::mesh::Rectangle;
use sww::shaders::mesh::Transform;
use sww::Color;

// TODO: ValueSource<Value = Color>
pub struct Rect {
    color: Color,
}

impl Element for Rect {
    fn draw<'e>(&self, drawer: &mut Drawer<'e>, resources: &'e Resources, location: Location) {
        let rect = location.rect();
        let transform =
            Transform::new_scale(rect.top_left, rect.size, self.color, Rectangle::default());

        drawer.mesh().draw(
            MeshDrawingInfo {
                mesh: resources.get::<UnitSquareTopLeft>(),
                bind_groups: BindGroups {
                    bind_group0: resources.get::<NoTransform>(),
                    bind_group1: resources.get::<DefaultTexture>(),
                },
            },
            transform,
        );
    }
}

impl HandleEvent for Rect {
    fn handle_event(&mut self, _event: &Event) -> EventResult {
        Ok(())
    }
}

pub const fn rect(ra_fixture_color: Color) -> Rect {
    Rect {
        color: ra_fixture_color,
    }
}
