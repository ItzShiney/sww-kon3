use crate::Event;
use crate::EventLoop;
use crate::EventLoopResult;
use crate::EventLoopTarget;

pub trait App {
    fn handle_event(&mut self, event: Event, target: &EventLoopTarget);

    fn run(&mut self, event_loop: EventLoop) -> EventLoopResult {
        event_loop.run(|event, target| self.handle_event(event, target))
    }
}
