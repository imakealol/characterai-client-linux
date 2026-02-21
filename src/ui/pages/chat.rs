use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use crate::services::AppServices;

const DEFAULT_AUTHOR_ID: &str = "0";

pub struct ChatPage {
    root: gtk::Box,
}

impl ChatPage {
    pub fn new(services: AppServices) -> Self {
        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(8)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        // Character ID row
        let char_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .build();
        let char_label = gtk::Label::new(Some("Character ID:"));
        let char_entry = gtk::Entry::builder()
            .placeholder_text("e.g. _nJzR…")
            .hexpand(true)
            .build();
        char_row.append(&char_label);
        char_row.append(&char_entry);

        // Chat transcript
        let transcript = gtk::TextView::builder()
            .editable(false)
            .cursor_visible(false)
            .vexpand(true)
            .wrap_mode(gtk::WrapMode::WordChar)
            .build();

        // Message input row
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

        // chat_id is cached per character session
        let chat_id: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

        send.connect_clicked(glib::clone!(
            @weak transcript, @weak input, @weak char_entry, @weak send => move |_btn| {
            let text = input.text().to_string();
            if text.trim().is_empty() { return; }
            let char_id = char_entry.text().to_string();
            if char_id.trim().is_empty() {
                let buf = transcript.buffer();
                buf.insert(&mut buf.end_iter(), "Error: please enter a Character ID first.\n");
                return;
            }

            let token = match services.secrets.get_auth_token() {
                Ok(Some(t)) => t,
                Ok(None) => {
                    let buf = transcript.buffer();
                    buf.insert(&mut buf.end_iter(),
                        "Error: no auth token saved. Go to Settings and save your token.\n");
                    return;
                }
                Err(e) => {
                    let buf = transcript.buffer();
                    buf.insert(&mut buf.end_iter(), &format!("Error reading token: {e}\n"));
                    return;
                }
            };

            // Show the user's message immediately
            {
                let buf = transcript.buffer();
                buf.insert(&mut buf.end_iter(), &format!("You: {text}\n"));
            }
            input.set_text("");
            send.set_sensitive(false);

            let chat_id_arc = Arc::clone(&chat_id);
            let cai = services.cai.clone();
            let (tx, rx) = glib::MainContext::channel::<Result<String, String>>(glib::Priority::DEFAULT);

            std::thread::spawn(move || {
                // Resolve or create chat_id
                let resolved_chat_id = {
                    let mut guard = chat_id_arc.lock().unwrap();
                    if let Some(id) = guard.as_deref() {
                        id.to_owned()
                    } else {
                        match cai.create_chat(&token, &char_id) {
                            Ok(id) => { *guard = Some(id.clone()); id }
                            Err(e) => {
                                let _ = tx.send(Err(format!("create_chat failed: {e}")));
                                return;
                            }
                        }
                    }
                };

                let result = cai
                    .send_message(&token, &resolved_chat_id, DEFAULT_AUTHOR_ID, &text)
                    .map_err(|e| e.to_string());
                let _ = tx.send(result);
            });

            rx.attach(None, glib::clone!(@weak transcript, @weak send =>
                @default-return glib::ControlFlow::Break,
                move |result| {
                    let buf = transcript.buffer();
                    match result {
                        Ok(reply) => buf.insert(&mut buf.end_iter(), &format!("AI: {reply}\n")),
                        Err(e)    => buf.insert(&mut buf.end_iter(), &format!("Error: {e}\n")),
                    }
                    send.set_sensitive(true);
                    let mut end = buf.end_iter();
                    transcript.scroll_to_iter(&mut end, 0.0, false, 0.0, 0.0);
                    glib::ControlFlow::Break
                }
            ));
        }));

        root.append(&char_row);
        root.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
        root.append(&transcript);
        root.append(&bottom);

        Self { root }
    }

    pub fn widget(&self) -> gtk::Widget {
        self.root.clone().upcast()
    }
}
