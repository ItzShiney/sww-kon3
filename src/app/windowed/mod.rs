use crate::app::App;
use crate::app::AppInfo;
use crate::window::*;
use ouroboros::self_referencing;

mod lazy_windowed;

pub use lazy_windowed::*;

#[self_referencing(pub_extras)]
pub struct WindowedApp {
    window: Window,

    #[borrows(window)]
    #[not_covariant]
    app_info: AppInfo<'this>,

    #[borrows(app_info)]
    #[not_covariant]
    app: Box<dyn App + 'this>,
}
