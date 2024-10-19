use crate::app::SignalSender;
use crate::drawer::resources::Resources;
use crate::drawer::DrawPass;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::IntoEventResult;
use crate::LocationRect;
use sww::window::event::MouseButton;

pub struct OnClick<E, F> {
    element: E,
    f: F,
}

impl<E: Element, F: Fn(&SignalSender) -> U, U: IntoEventResult> Element for OnClick<E, F> {
    fn draw(&self, pass: &mut DrawPass, resources: &Resources, location: LocationRect) {
        self.element.draw(pass, resources, location);
    }
}

impl<E: HandleEvent, F: Fn(&SignalSender) -> U, U: IntoEventResult> HandleEvent for OnClick<E, F> {
    fn handle_event(
        &self,
        signal_sender: &crate::prelude::SignalSender,
        event: &Event,
    ) -> EventResult {
        if let Event::Click { point: _, button } = *event {
            // TODO && location.contains(point)
            if button == MouseButton::Left {
                (self.f)(signal_sender).into_event_result()?;
            }
        }

        self.element.handle_event(signal_sender, event)
    }
}

pub const fn on_click<E, F: Fn(&SignalSender) -> U, U: IntoEventResult>(
    ra_fixture_element: E,
    ra_fixture_f: F,
) -> OnClick<E, F> {
    OnClick {
        element: ra_fixture_element,
        f: ra_fixture_f,
    }
}
