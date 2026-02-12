slint::include_modules!();
mod controller;
mod logic;

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainController::new()?;

    // --- NEW CODE: Send version to UI ---
    // This reads the version directly from your Cargo.toml
    let version = env!("CARGO_PKG_VERSION");
    ui.global::<AppState>().set_app_version(format!("v{}", version).into());
    // ------------------------------------

    // Start updater in background
    std::thread::spawn(|| {
        let _ = logic::updater::check_for_updates();
    });

    controller::login_controller::setup(&ui);
    ui.run()
}
