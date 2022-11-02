mod window;

use gtk::{gio, prelude::*};
use window::Window;

fn main() {
    gio::resources_register_include!("resources.gresource").expect("Failed to register ressource");

    let app = adw::Application::builder()
        .application_id("org.pug.app")
        .build();

    app.connect_startup(setup_shortcut);
    app.connect_activate(build_ui);
    app.run();
}

fn setup_shortcut(_app: &adw::Application) {}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);
    window.present();
}
