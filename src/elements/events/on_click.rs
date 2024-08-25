use crate::drawer::DrawPass;
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

impl<R, E: Element<R>, F: FnMut() -> U, U: IntoEventResult> Element<R> for OnClick<E, F> {
    fn draw(&self, pass: &mut DrawPass, resources: &R, location: Location) {
        self.element.draw(pass, resources, location);
    }
}

impl<E: HandleEvent, F: FnMut() -> U, U: IntoEventResult> HandleEvent for OnClick<E, F> {
    fn handle_event(&mut self, event: &Event) -> EventResult {
        #[allow(clippy::equatable_if_let)]
        if let Event::Click = event {
            (self.f)().into_event_result()?;
        }

        self.element.handle_event(event)
    }
}

pub const fn on_click<E, F: FnMut() -> U, U: IntoEventResult>(
    ra_fixture_element: E,
    ra_fixture_f: F,
) -> OnClick<E, F> {
    OnClick {
        element: ra_fixture_element,
        f: ra_fixture_f,
    }
}
