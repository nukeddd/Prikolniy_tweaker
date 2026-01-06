use gtk4::prelude::*;
use gtk4::{gio, glib, Application};

mod window;
use window::TweakerWindow;

const APP_ID: &str = "com.github.jeremy-compost.tweaker";

fn main() -> glib::ExitCode {
    libadwaita::init().expect("libadwaita not initialized");
    gio::resources_register_include!("tweaker.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = TweakerWindow::new(app);
    window.present();
}
