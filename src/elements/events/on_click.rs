use crate::resources::Resources;
use crate::shared::Shared;
use crate::values::ArgSource;
use crate::Anchor;
use crate::Build;
use crate::BuildElement;
use crate::Drawer;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::IntoEventResult;
use crate::Location;
use crate::ResolveAnchors;

pub struct OnClick<E, Src, F> {
    element: E,
    source: Src,
    f: F,
}

impl<E: Build, Src: Build, F> Build for OnClick<E, Src, F> {
    type Built = OnClick<E::Built, Src::Built, F>;

    fn build(self) -> Self::Built {
        OnClick {
            element: self.element.build(),
            source: self.source.build(),
            f: self.f,
        }
    }
}

impl<E: ResolveAnchors, Src: ResolveAnchors, F> ResolveAnchors for OnClick<E, Src, F> {
    type AnchorsSet = (E::AnchorsSet, Src::AnchorsSet);

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        (self.element.get_anchor::<A>()).or_else(|| self.source.get_anchor::<A>())
    }

    fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
        self.element.resolve_anchor::<A>(anchor);
        self.source.resolve_anchor::<A>(anchor);
    }
}

impl<E: Element, Src: ArgSource, F: FnMut(Src::Arg<'_>) -> R, R: IntoEventResult> Element
    for OnClick<E, Src, F>
{
    fn draw<'e>(&self, drawer: &mut Drawer<'e>, resources: &'e Resources, location: Location) {
        self.element.draw(drawer, resources, location);
    }
}

impl<E: HandleEvent, Src: ArgSource, F: FnMut(Src::Arg<'_>) -> R, R: IntoEventResult> HandleEvent
    for OnClick<E, Src, F>
{
    fn handle_event(&mut self, event: &Event) -> EventResult {
        #[allow(clippy::equatable_if_let)]
        if let Event::Click = event {
            self.source.apply_to(&mut self.f)?;
        }

        self.element.handle_event(event)
    }
}

pub const fn on_click<
    E: BuildElement,
    Src: Build<Built: ArgSource>,
    F: FnMut(<Src::Built as ArgSource>::Arg<'_>) -> R,
    R: IntoEventResult,
>(
    ra_fixture_element: E,
    ra_fixture_source: Src,
    ra_fixture_f: F,
) -> OnClick<E, Src, F> {
    OnClick {
        element: ra_fixture_element,
        source: ra_fixture_source,
        f: ra_fixture_f,
    }
}
