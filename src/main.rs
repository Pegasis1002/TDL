use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};

fn main() {
    let app = Application::new(
        Some("com.example.gtk4-app"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("GTK4 App"));
        window.set_default_size(300, 200);
        window.show();
    });

    app.run();
}

