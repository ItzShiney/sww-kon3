use crate::drawer::DrawPass;
use crate::shared::Shared;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::IntoEventResult;
use crate::InvalidateCache;
use crate::Location;

pub struct OnClick<E, F> {
    element: E,
    f: F,
}

impl<R, E: Element<R>, F: Fn() -> U, U: IntoEventResult> Element<R> for OnClick<E, F> {
    fn draw(&self, pass: &mut DrawPass, resources: &R, location: Location) {
        self.element.draw(pass, resources, location);
    }
}

impl<E: HandleEvent, F: Fn() -> U, U: IntoEventResult> HandleEvent for OnClick<E, F> {
    fn handle_event(&self, event: &Event) -> EventResult {
        #[allow(clippy::equatable_if_let)]
        if let Event::Click = event {
            (self.f)().into_event_result()?;
        }

        self.element.handle_event(event)
    }
}

impl<T: ?Sized, E: InvalidateCache<T>, F> InvalidateCache<T> for OnClick<E, F> {
    fn invalidate_cache(&self, shared: &Shared<T>) -> bool {
        self.element.invalidate_cache(shared)
    }
}

pub const fn on_click<E, F: Fn() -> U, U: IntoEventResult>(
    ra_fixture_element: E,
    ra_fixture_f: F,
) -> OnClick<E, F> {
    OnClick {
        element: ra_fixture_element,
        f: ra_fixture_f,
    }
}
