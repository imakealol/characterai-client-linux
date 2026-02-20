use gtk::prelude::*;
use crate::services::AppServices;

pub struct SettingsPage {
    root: gtk::Box,
}

impl SettingsPage {
    pub fn new(_services: AppServices) -> Self {
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

        let auth = gtk::PasswordEntry::builder()
            .placeholder_text("Paste CharacterAI cookie/token…")
            .show_peek_icon(true)
            .build();

        let save = gtk::Button::with_label("Save (placeholder)");
        save.connect_clicked(glib::clone!(@weak auth => move |_| {
            let _value = auth.text().to_string();
        }));

        root.append(&title);
        root.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
        root.append(&gtk::Label::new(Some("Auth")));
        root.append(&auth);
        root.append(&save);

        Self { root }
    }

    pub fn widget(&self) -> gtk::Widget {
        self.root.clone().upcast()
    }
}
