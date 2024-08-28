use crate::drawer::DrawPass;
use crate::prelude::Resources;
use crate::resources::mesh::DefaultTexture;
use crate::resources::mesh::NoGlobalTransform;
use crate::resources::mesh::UnitSquareTopLeft;
use crate::shared;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::InvalidateCache;
use crate::Location;
use crate::MeshDrawingInfo;
use sww::shaders::mesh::BindGroups;
use sww::shaders::mesh::Rectangle;
use sww::shaders::mesh::Transform;
use sww::Color;

// TODO ValueSource<Value = Color>
pub struct Rect {
    color: Color,
}

impl Element for Rect {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: Location) {
        let rect = location.rect();
        let transform =
            Transform::new_scale(rect.top_left, rect.size, self.color, Rectangle::default());

        pass.mesh().draw(
            &MeshDrawingInfo {
                mesh: resources.get::<UnitSquareTopLeft>(),
                bind_groups: BindGroups {
                    bind_group0: resources.get::<NoGlobalTransform>(),
                    bind_group1: resources.get::<DefaultTexture>(),
                },
            },
            transform,
        );
    }
}

impl HandleEvent for Rect {
    fn handle_event(&self, _event: &Event) -> EventResult {
        Ok(())
    }
}

impl InvalidateCache for Rect {
    fn invalidate_cache(&self, _addr: shared::Addr) -> bool {
        false
    }
}

pub const fn rect(ra_fixture_color: Color) -> Rect {
    Rect {
        color: ra_fixture_color,
    }
}
