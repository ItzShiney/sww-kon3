use crate::shared::Shared;
use crate::values::AutoValueSource;
use crate::Anchor;
use crate::Build;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::ResolveAnchors;

#[derive(Debug, Clone, Copy)]
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

impl<Ty, A: Element, B: Element> Element for Split<Ty, (A, B)> {}

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
