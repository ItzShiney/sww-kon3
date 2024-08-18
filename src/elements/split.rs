use crate::resources::Resources;
use crate::shared::Shared;
use crate::values::AutoValueSource;
use crate::values::ValueSource;
use crate::Anchor;
use crate::Build;
use crate::Drawer;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::Location;
use crate::ResolveAnchors;
use sww::shaders::mesh::Rectangle;
use sww::vec2;
use sww::Vec2;

#[derive(Clone, Copy)]
pub enum SplitType {
    Vertical,
    Horizontal,
    Adaptive,
}

impl AutoValueSource for SplitType {}

#[derive(Debug)]
pub struct Split<Ty, Es> {
    type_: Ty,
    elements: Es,
}

impl<Ty: Build, Es: Build> Build for Split<Ty, Es> {
    type Built = Split<Ty::Built, Es::Built>;

    fn build(self) -> Self::Built {
        Split {
            type_: self.type_.build(),
            elements: self.elements.build(),
        }
    }
}

impl<Ty: ResolveAnchors, Es: ResolveAnchors> ResolveAnchors for Split<Ty, Es> {
    type AnchorsSet = (Ty::AnchorsSet, Es::AnchorsSet);

    fn get_anchor<_A: Anchor>(&self) -> Option<Shared<_A::Value>> {
        (self.type_.get_anchor::<_A>()).or_else(|| self.elements.get_anchor::<_A>())
    }

    fn resolve_anchor<_A: Anchor>(&mut self, anchor: &Shared<_A::Value>) {
        self.type_.resolve_anchor::<_A>(anchor);
        self.elements.resolve_anchor::<_A>(anchor);
    }
}

impl<Ty: ValueSource<Value = SplitType>, A: Element, B: Element> Element for Split<Ty, (A, B)> {
    fn draw<'e>(&self, drawer: &mut Drawer<'e>, resources: &'e Resources, location: Location) {
        const COUNT: usize = 2;
        const FRACTION: f32 = 1. / COUNT as f32;

        let (subrect_size, subrect_offset) = {
            let type_ = *self.type_.value();
            match type_ {
                SplitType::Vertical => (vec2(1., FRACTION), vec2(0., FRACTION)),
                SplitType::Horizontal => (vec2(FRACTION, 1.), vec2(FRACTION, 0.)),
                SplitType::Adaptive => todo!(),
            }
        };

        let mut top_left = Vec2::default();
        {
            let element = &self.elements.0;
            let weight = 1_usize;
            let size = subrect_size * weight as f32;

            element.draw(
                drawer,
                resources,
                location.subrect(Rectangle { top_left, size }),
            );
            top_left += subrect_offset;
        }

        {
            let element = &self.elements.1;
            let weight = 1_usize;
            let size = subrect_size * weight as f32;

            element.draw(
                drawer,
                resources,
                location.subrect(Rectangle { top_left, size }),
            );
            top_left += subrect_offset;
        }
    }
}

impl<Ty, Es: HandleEvent> HandleEvent for Split<Ty, Es> {
    fn handle_event(&mut self, event: &Event) -> EventResult {
        self.elements.handle_event(event)
    }
}

pub const fn split<Es: Build>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
    Split {
        type_: SplitType::Adaptive,
        elements: ra_fixture_elements,
    }
}

pub const fn line<Es: Build>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
    Split {
        type_: SplitType::Horizontal,
        elements: ra_fixture_elements,
    }
}

pub const fn column<Es: Build>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
    Split {
        type_: SplitType::Vertical,
        elements: ra_fixture_elements,
    }
}
