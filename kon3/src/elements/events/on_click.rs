use crate::drawer::DrawPass;
use crate::prelude::Resources;
use crate::shared;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::IntoEventResult;
use crate::InvalidateCache;
use crate::LocationRect;
use sww::window::event::MouseButton;

pub struct OnClick<E, F> {
    element: E,
    f: F,
}

impl<E: Element, F: Fn() -> U, U: IntoEventResult> Element for OnClick<E, F> {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: LocationRect) {
        self.element.draw(pass, resources, location);
    }
}

impl<E: HandleEvent, F: Fn() -> U, U: IntoEventResult> HandleEvent for OnClick<E, F> {
    fn handle_event(&self, event: &Event) -> EventResult {
        if let Event::Click { point: _, button } = *event {
            // TODO && location.contains(point)
            if button == MouseButton::Left {
                (self.f)().into_event_result()?;
            }
        }

        self.element.handle_event(event)
    }
}

impl<E: InvalidateCache, F> InvalidateCache for OnClick<E, F> {
    fn invalidate_cache(&self, addr: shared::Addr) -> bool {
        self.element.invalidate_cache(addr)
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
