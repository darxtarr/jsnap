mod brand;
mod core;
mod ui;
mod export;
mod history;
mod util;

use log::info;
use ui::hotkey::HotkeyListener;
use core::policy::Policy;

fn main() {
    env_logger::init();
    info!("{} starting…", brand::BRAND);

    let policy = Policy::load_global().unwrap_or_default();
    let mut listener = HotkeyListener::new(policy);
    listener.run_event_loop();
}
