use gtk::prelude::*;
use crate::services::AppServices;

pub struct ChatPage {
    root: gtk::Box,
}

impl ChatPage {
    pub fn new(_services: AppServices) -> Self {
        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        let transcript = gtk::TextView::builder()
            .editable(false)
            .cursor_visible(false)
            .vexpand(true)
            .build();

        let input = gtk::Entry::builder()
            .placeholder_text("Type a message…")
            .hexpand(true)
            .build();

        let send = gtk::Button::with_label("Send");

        let bottom = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .build();
        bottom.append(&input);
        bottom.append(&send);

        send.connect_clicked(glib::clone!(@weak transcript, @weak input => move |_| {
            let text = input.text().to_string();
            if text.trim().is_empty() { return; }
            let buffer = transcript.buffer();
            buffer.insert(&mut buffer.end_iter(), &format!("You: {}\n", text));
            input.set_text("");
        }));

        root.append(&transcript);
        root.append(&bottom);

        Self { root }
    }

    pub fn widget(&self) -> gtk::Widget {
        self.root.clone().upcast()
    }
}
