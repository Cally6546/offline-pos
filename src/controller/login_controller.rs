use slint::ComponentHandle;
use crate::{MainController, AuthStore, AppState}; // Imported generated code
pub fn setup(ui: &MainController) {
    let ui_handle = ui.as_weak();

    ui.global::<AuthStore>().on_login_requested(move |email, pin| {
        let ui = ui_handle.unwrap();
        let auth_store = ui.global::<AuthStore>();

        // --- 1. CLEANUP PHASE ---
        // Clear old errors so the user knows a new attempt started
        auth_store.set_email_error("".into());
        auth_store.set_pin_error("".into());
        auth_store.set_is_loading(true);

        let thread_handle = ui_handle.clone();

        std::thread::spawn(move || {
            // --- 2. VALIDATION PHASE (BACKEND) ---
            // In a real app, you'd call a function from your `logic` module here
            std::thread::sleep(std::time::Duration::from_secs(1));

            let success = email == "admin" && pin == "1234";

            // --- 3. UI UPDATE PHASE ---
            let result_handle = thread_handle.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui_instance) = result_handle.upgrade() {
                    let auth_store = ui_instance.global::<AuthStore>();
                    let app_state = ui_instance.global::<AppState>();

                    if success {
                        // Success: Reset state and move to dashboard
                        auth_store.set_email_error("".into());
                        auth_store.set_pin_error("".into());

                        app_state.set_user_name("Administrator".into());
                        app_state.set_active_screen("dashboard".into());
                    } else {
                        // Error: Specific feedback
                        if email != "admin" {
                            auth_store.set_email_error("User not found".into());
                        } else {
                            auth_store.set_pin_error("Incorrect PIN".into());
                        }
                    }
                    auth_store.set_is_loading(false);
                }
            }).unwrap();
        });
    });
}
