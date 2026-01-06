use gtk4::Button;
use gtk4::ApplicationWindow;
use gtk4::prelude::*;
use gtk4::{gio, glib, Application};

const APP_ID: &str = "com.github.jeremy-compost.tweaker";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("tweaker.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);

    window.set_title(Some("Tweaker"));
    window.set_default_size(350, 70);

    let button = Button::with_label("Click me!");

    button.connect_clicked(|_| {
        eprintln!("Clicked!");
    });

    window.set_child(Some(&button));
    window.present();
}
