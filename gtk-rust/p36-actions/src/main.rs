/**
 * actions(app/win group)
 * 2024.05.27 by dralee
 */
use gtk::{gio::ActionEntry, glib, prelude::*, Application, ApplicationWindow};

const APP_ID:&str = "org.gtk_rs.Actions1";

fn main() -> glib::ExitCode {
    // create a pp
    let app = Application::builder().application_id(APP_ID).build();

    // connect activate signal
    app.connect_activate(build_ui);

    // set keyboard accelerator to trigger "win.close"
    app.set_accels_for_action("win.close", &["<Ctrl>W"]); // app for application, win for window

    // run the app
    app.run()
}

fn build_ui(app:&Application) {
    // create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Actions for keyboard")
        .width_request(320)
        .build();

    // add action "close" to window taking no parameter
    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();

    // add the action entry
    window.add_action_entries([action_close]);

    // present window
    window.present();
}
