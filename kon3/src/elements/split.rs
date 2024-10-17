use crate::drawer::DrawPass;
use crate::resources::Resources;
use crate::shared;
use crate::values::AutoValueSource;
use crate::values::ValueSourceBorrow;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::InvalidateCaches;
use crate::LocationRect;
use std::borrow::Borrow;
use std::collections::BTreeSet;
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

pub struct Split<Ty, Es> {
    ty: Ty,
    elements: Es,
}

macro_rules! impl_tuple {
    ( $($T:ident)+ ) => {
        impl<Ty: ValueSourceBorrow<SplitType>, $($T: Element),+> Element
            for Split<Ty, ($($T),+)>
        {
            fn draw(
                &self,
                pass: &mut DrawPass,
                resources: &Resources,
                location: LocationRect,
            ) {
                #[allow(non_snake_case)]
                let ($($T),+) = &self.elements;
                draw_helper(
                    *(*self.ty.value()).borrow(),
                    &[$((1, $T)),+],
                    pass,
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

impl<Ty: InvalidateCaches, Es: InvalidateCaches> InvalidateCaches for Split<Ty, Es> {
    fn invalidate_caches(&self, addrs: &BTreeSet<shared::Addr>) -> bool {
        self.ty.invalidate_caches(addrs) || self.elements.invalidate_caches(addrs)
    }
}

fn draw_helper(
    ty: SplitType,
    elements: &[(usize, &dyn Element)],
    pass: &mut DrawPass,
    resources: &Resources,
    location: LocationRect,
) {
    let total_weight: usize = elements.iter().map(|&(weight, _)| weight).sum();
    let fraction = 1. / total_weight as f32;

    let (rect_fraction_size, rect_fraction_offset) = match ty {
        SplitType::Vertical => (vec2(1., fraction), vec2(0., fraction)),
        SplitType::Horizontal => (vec2(fraction, 1.), vec2(fraction, 0.)),
        SplitType::Adaptive => todo!(),
    };

    let mut top_left = Vec2::ZERO;
    for (weight, element) in elements.iter().copied() {
        let weight = weight as f32;
        let size = rect_fraction_size * weight;
        let offset = rect_fraction_offset * weight;

        element.draw(
            pass,
            resources,
            location.subrect(Rectangle::new(top_left, size)),
        );
        top_left += offset;
    }
}

impl<Ty, Es: HandleEvent> HandleEvent for Split<Ty, Es> {
    fn handle_event(&self, event: &Event) -> EventResult {
        self.elements.handle_event(event)
    }
}

pub const fn split<Es>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
    Split {
        ty: SplitType::Adaptive,
        elements: ra_fixture_elements,
    }
}

pub const fn line<Es>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
    Split {
        ty: SplitType::Horizontal,
        elements: ra_fixture_elements,
    }
}

pub const fn column<Es>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
    Split {
        ty: SplitType::Vertical,
        elements: ra_fixture_elements,
    }
}
