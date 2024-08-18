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
    ty: Ty,
    elements: Es,
}

impl<Ty: Build, Es: Build> Build for Split<Ty, Es> {
    type Built = Split<Ty::Built, Es::Built>;

    fn build(self) -> Self::Built {
        Split {
            ty: self.ty.build(),
            elements: self.elements.build(),
        }
    }
}

impl<Ty: ResolveAnchors, Es: ResolveAnchors> ResolveAnchors for Split<Ty, Es> {
    type AnchorsSet = (Ty::AnchorsSet, Es::AnchorsSet);

    fn get_anchor<_A: Anchor>(&self) -> Option<Shared<_A::Value>> {
        (self.ty.get_anchor::<_A>()).or_else(|| self.elements.get_anchor::<_A>())
    }

    fn resolve_anchor<_A: Anchor>(&mut self, anchor: &Shared<_A::Value>) {
        self.ty.resolve_anchor::<_A>(anchor);
        self.elements.resolve_anchor::<_A>(anchor);
    }
}

macro_rules! impl_tuple {
    ( $($T:ident)+ ) => {
        impl<Ty: ValueSource<Value = SplitType>, $($T: Element),+> Element
            for Split<Ty, ($($T),+)>
        {
            fn draw<'e>(
                &self,
                drawer: &mut Drawer<'e>,
                resources: &'e Resources,
                location: Location,
            ) {
                #[allow(non_snake_case)]
                let ($($T),+) = &self.elements;
                draw_helper(
                    *self.ty.value(),
                    [$((1, $T)),+],
                    drawer,
                    resources,
                    location,
                );
            }
        }
    };
}

impl_tuple!(A B);
impl_tuple!(A B C);
impl_tuple!(A B C D);
impl_tuple!(A B C D E);
impl_tuple!(A B C D E F);
impl_tuple!(A B C D E F G);
impl_tuple!(A B C D E F G H);
impl_tuple!(A B C D E F G H I);
impl_tuple!(A B C D E F G H I J);
impl_tuple!(A B C D E F G H I J K);
impl_tuple!(A B C D E F G H I J K L);

fn draw_helper<'e, const N: usize>(
    ty: SplitType,
    elements: [(usize, &dyn Element); N],
    drawer: &mut Drawer<'e>,
    resources: &'e Resources,
    location: Location,
) {
    let total_weight: usize = elements.iter().map(|&(weight, _)| weight).sum();
    let fraction = 1. / total_weight as f32;

    let (rect_fraction_size, rect_fraction_offset) = match ty {
        SplitType::Vertical => (vec2(1., fraction), vec2(0., fraction)),
        SplitType::Horizontal => (vec2(fraction, 1.), vec2(fraction, 0.)),
        SplitType::Adaptive => todo!(),
    };

    let mut top_left = Vec2::ZERO;
    for (weight, element) in elements {
        let weight = weight as f32;
        let size = rect_fraction_size * weight;
        let offset = rect_fraction_offset * weight;

        element.draw(
            drawer,
            resources,
            location.subrect(Rectangle::new(top_left, size)),
        );
        top_left += offset;
    }
}

impl<Ty, Es: HandleEvent> HandleEvent for Split<Ty, Es> {
    fn handle_event(&mut self, event: &Event) -> EventResult {
        self.elements.handle_event(event)
    }
}

pub const fn split<Es: Build>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
    Split {
        ty: SplitType::Adaptive,
        elements: ra_fixture_elements,
    }
}

pub const fn line<Es: Build>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
    Split {
        ty: SplitType::Horizontal,
        elements: ra_fixture_elements,
    }
}

pub const fn column<Es: Build>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
    Split {
        ty: SplitType::Vertical,
        elements: ra_fixture_elements,
    }
}
