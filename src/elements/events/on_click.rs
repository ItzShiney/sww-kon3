use crate::resources::Resources;
use crate::values::ArgSource;
use crate::Drawer;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::IntoEventResult;
use crate::Location;

pub struct OnClick<E, Src, F> {
    element: E,
    source: Src,
    f: F,
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
    E: Element,
    Src: ArgSource,
    F: FnMut(Src::Arg<'_>) -> R,
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
