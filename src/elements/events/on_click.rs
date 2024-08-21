use crate::resources::Resources;
use crate::Drawer;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::IntoEventResult;
use crate::Location;

pub struct OnClick<E, F> {
    element: E,
    f: F,
}

impl<E: Element, F: FnMut() -> R, R: IntoEventResult> Element for OnClick<E, F> {
    fn draw<'e>(&self, drawer: &mut Drawer<'e>, resources: &'e Resources, location: Location) {
        self.element.draw(drawer, resources, location);
    }
}

impl<E: HandleEvent, F: FnMut() -> R, R: IntoEventResult> HandleEvent for OnClick<E, F> {
    fn handle_event(&mut self, event: &Event) -> EventResult {
        #[allow(clippy::equatable_if_let)]
        if let Event::Click = event {
            (self.f)().into_event_result()?;
        }

        self.element.handle_event(event)
    }
}

pub const fn on_click<E: Element, F: FnMut() -> R, R: IntoEventResult>(
    ra_fixture_element: E,
    ra_fixture_f: F,
) -> OnClick<E, F> {
    OnClick {
        element: ra_fixture_element,
        f: ra_fixture_f,
    }
}
