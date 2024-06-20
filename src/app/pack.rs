use crate::app::EventHandler;
use crate::window::*;
use ouroboros::self_referencing;

#[self_referencing(pub_extras)]
pub struct AppPack {
    window: Window,

    #[borrows(window)]
    #[not_covariant]
    rw: RenderWindow<'this>,

    #[borrows(rw)]
    #[not_covariant]
    event_handler: EventHandler<'this>,
}
