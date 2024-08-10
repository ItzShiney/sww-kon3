use crate::consume;
use crate::shared::Shared;
use crate::values::ArgsSource;
use crate::Anchor;
use crate::Build;
use crate::BuildElement;
use crate::Element;
use crate::Event;
use crate::EventConsumed;
use crate::HandleEvent;
use crate::ResolveAnchors;
use std::fmt;
use std::fmt::Debug;

pub struct OnClickConsume<E, Src, F> {
    element: E,
    source: Src,
    f: F,
}

impl<E: Debug, Src: Debug, F> Debug for OnClickConsume<E, Src, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OnClickConsume")
            .field("element", &self.element)
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}

impl<E: Build, Src: Build, F> Build for OnClickConsume<E, Src, F> {
    type Output = OnClickConsume<E::Output, Src::Output, F>;

    fn build(self) -> Self::Output {
        OnClickConsume {
            element: self.element.build(),
            source: self.source.build(),
            f: self.f,
        }
    }
}

impl<E: ResolveAnchors, Src: ResolveAnchors, F> ResolveAnchors for OnClickConsume<E, Src, F> {
    type AnchorsSet = (E::AnchorsSet, Src::AnchorsSet);

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        (self.element.get_anchor::<A>()).or_else(|| self.source.get_anchor::<A>())
    }

    fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
        self.element.resolve_anchor::<A>(anchor);
        self.source.resolve_anchor::<A>(anchor);
    }
}

// impl<E: Element, Src, F> Element for OnClickConsume<E, Src, F> where Self: HandleEvent {}
impl<E: Element, Src: ArgsSource> Element for OnClickConsume<E, Src, Src::Fn> {}

impl<E: HandleEvent, Src: ArgsSource> HandleEvent for OnClickConsume<E, Src, Src::Fn> {
    fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
        match event {
            Event::Click => {
                self.source.apply_to(&self.f);
                consume()
            }

            _ => self.element.handle_event(event),
        }
    }
}

pub const fn on_click_consume<E: BuildElement, Src: Build<Output = ArgSrc>, ArgSrc: ArgsSource>(
    ra_fixture_element: E,
    ra_fixture_source: Src,
    ra_fixture_f: ArgSrc::Fn,
) -> OnClickConsume<E, Src, ArgSrc::Fn> {
    OnClickConsume {
        element: ra_fixture_element,
        source: ra_fixture_source,
        f: ra_fixture_f,
    }
}
