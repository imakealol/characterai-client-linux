use gtk::prelude::*;
use crate::services::AppServices;

pub struct SettingsPage {
    root: gtk::Box,
}

impl SettingsPage {
    pub fn new(services: AppServices) -> Self {
        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        let title = gtk::Label::new(Some("Settings"));
        title.set_xalign(0.0);

        let hint = gtk::Label::new(Some(
            "Paste your CharacterAI auth token (from the __Secure-next-auth.session-token \
             browser cookie or a CAI Plus token).",
        ));
        hint.set_xalign(0.0);
        hint.set_wrap(true);

        let auth = gtk::PasswordEntry::builder()
            .placeholder_text("Paste CharacterAI auth token…")
            .show_peek_icon(true)
            .hexpand(true)
            .build();

        // Pre-fill from keyring
        if let Ok(Some(stored)) = services.secrets.get_auth_token() {
            auth.set_text(&stored);
        }

        let status = gtk::Label::new(None);
        status.set_xalign(0.0);

        let buttons = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .build();

        let save = gtk::Button::with_label("Save");
        let clear = gtk::Button::with_label("Clear");
        buttons.append(&save);
        buttons.append(&clear);

        let svc_save = services.clone();
        save.connect_clicked(glib::clone!(@weak auth, @weak status => move |_| {
            let value = auth.text().to_string();
            match svc_save.secrets.set_auth_token(&value) {
                Ok(()) => status.set_label("✓ Token saved."),
                Err(e) => status.set_label(&format!("Error: {e}")),
            }
        }));

        let svc_clear = services.clone();
        clear.connect_clicked(glib::clone!(@weak auth, @weak status => move |_| {
            match svc_clear.secrets.clear_auth_token() {
                Ok(()) => {
                    auth.set_text("");
                    status.set_label("✓ Token cleared.");
                }
                Err(e) => status.set_label(&format!("Error: {e}")),
            }
        }));

        root.append(&title);
        root.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
        root.append(&gtk::Label::new(Some("Auth")));
        root.append(&hint);
        root.append(&auth);
        root.append(&buttons);
        root.append(&status);

        Self { root }
    }

    pub fn widget(&self) -> gtk::Widget {
        self.root.clone().upcast()
    }
}
