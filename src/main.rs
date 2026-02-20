mod services;
mod ui;

use gtk::prelude::*;

fn main() {
    let app = gtk::Application::builder()
        .application_id("io.github.imakealol.characterai-client-linux")
        .build();

    app.connect_activate(|app| {
        let services = services::AppServices::new();
        let window = ui::build_main_window(app, services);
        window.present();
    });

    app.run();
}
