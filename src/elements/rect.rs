use crate::drawer::DrawPass;
use crate::prelude::Shared;
use crate::resources::mesh::DefaultTexture;
use crate::resources::mesh::NoGlobalTransform;
use crate::resources::mesh::UnitSquareTopLeft;
use crate::resources::ResourceFrom;
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

impl<R> Element<R> for Rect
where
    UnitSquareTopLeft: ResourceFrom<R>,
    NoGlobalTransform: ResourceFrom<R>,
    DefaultTexture: ResourceFrom<R>,
{
    fn draw(&self, pass: &mut DrawPass, resources: &R, location: Location) {
        let rect = location.rect();
        let transform =
            Transform::new_scale(rect.top_left, rect.size, self.color, Rectangle::default());

        pass.mesh().draw(
            &MeshDrawingInfo {
                mesh: UnitSquareTopLeft::resource_from(resources),
                bind_groups: BindGroups {
                    bind_group0: NoGlobalTransform::resource_from(resources),
                    bind_group1: DefaultTexture::resource_from(resources),
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

impl<T: ?Sized> InvalidateCache<T> for Rect {
    fn invalidate_cache(&self, _shared: &Shared<T>) -> bool {
        false
    }
}

pub const fn rect(ra_fixture_color: Color) -> Rect {
    Rect {
        color: ra_fixture_color,
    }
}
