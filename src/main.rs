mod backend;
mod models;
mod ui;

use adw::prelude::*;

fn main() {
    let app = adw::Application::builder()
        .application_id("com.jon.nmwifigui")
        .build();

    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    app.run();
}
