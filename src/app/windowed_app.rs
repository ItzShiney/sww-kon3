use crate::window::*;
use crate::App;
use crate::AppInfo;
use ouroboros::self_referencing;

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
