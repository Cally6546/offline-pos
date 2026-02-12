slint::include_modules!();
mod controller;


fn main() -> Result<(), slint::PlatformError> {
    let ui = MainController::new()?;

    // Initialize your controller
    controller::login_controller::setup(&ui);

    // --- FULLSCREEN LOGIC ---
    // Option A: Fullscreen (Covers taskbars/panels)
    //ui.window().set_fullscreen(true);

    // Option B: Maximized (Keeps taskbars visible - common for POS)
   ui.window().set_maximized(true);
    // ------------------------

    ui.run()
}
