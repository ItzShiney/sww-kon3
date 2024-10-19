use crate::drawer::resources::Resources;
use crate::drawer::DrawPass;
use crate::resources::mesh::DefaultTexture;
use crate::resources::mesh::NoGlobalTransform;
use crate::resources::mesh::UnitSquareTopLeft;
use crate::values::ValueSourceBorrow;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::LocationRect;
use crate::MeshDrawingInfo;
use std::borrow::Borrow;
use sww::shaders::mesh::BindGroups;
use sww::shaders::mesh::Rectangle;
use sww::shaders::mesh::Transform;
use sww::Color;

pub struct Rect<Clr> {
    color: Clr,
}

impl<Clr: ValueSourceBorrow<Color>> Element for Rect<Clr> {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: LocationRect) {
        let rect = location.rect();
        let color = *(*self.color.value()).borrow();
        let transform = Transform::new_scale(rect.top_left, rect.size, color, Rectangle::default());

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

impl<Clr> HandleEvent for Rect<Clr> {
    fn handle_event(
        &self,
        _signal_sender: &crate::prelude::Signaler,
        _event: &Event,
    ) -> EventResult {
        Ok(())
    }
}

pub const fn rect<Clr: ValueSourceBorrow<Color>>(ra_fixture_color: Clr) -> Rect<Clr> {
    Rect {
        color: ra_fixture_color,
    }
}
