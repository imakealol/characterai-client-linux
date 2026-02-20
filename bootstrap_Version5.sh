#!/usr/bin/env bash
set -euo pipefail

if [[ ! -d .git ]]; then
  echo "ERROR: Run from repo root (where .git exists)."
  exit 1
fi

mkdir -p src/services src/ui/pages

cat > .gitignore <<'__EOF__GITIGNORE__'
/target
**/*.rs.bk
.DS_Store
.idea
.vscode
__EOF__GITIGNORE__

cat > rust-toolchain.toml <<'__EOF__TOOLCHAIN__'
[toolchain]
channel = "stable"
__EOF__TOOLCHAIN__

cat > Cargo.toml <<'__EOF__CARGO__'
[package]
name = "characterai-client-linux"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
glib = "0.19"
gtk = { package = "gtk4", version = "0.8", features = ["v4_8"] }

[profile.release]
lto = true
codegen-units = 1
opt-level = 3
__EOF__CARGO__

cat > README.md <<'__EOF__README__'
# characterai-client-linux (Debian/Kali, KDE/Wayland)

Native desktop client in Rust + GTK4.

Build deps (Debian/Kali):
  sudo apt-get update
  sudo apt-get install -y build-essential pkg-config libgtk-4-dev

Run:
  cargo run
__EOF__README__

cat > src/main.rs <<'__EOF__MAIN__'
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
__EOF__MAIN__

cat > src/services/mod.rs <<'__EOF__SERVICES_MOD__'
pub mod characterai;
pub mod secrets;

#[derive(Clone)]
pub struct AppServices {
    pub secrets: secrets::SecretsService,
    pub cai: characterai::CharacterAiService,
}

impl AppServices {
    pub fn new() -> Self {
        Self {
            secrets: secrets::SecretsService::new(),
            cai: characterai::CharacterAiService::new(),
        }
    }
}
__EOF__SERVICES_MOD__

cat > src/services/secrets.rs <<'__EOF__SECRETS__'
use anyhow::Result;

/// Placeholder secrets layer.
/// Next step: implement org.freedesktop.secrets (Secret Service),
/// which on KDE typically maps to KWallet.
#[derive(Clone, Default)]
pub struct SecretsService;

impl SecretsService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_auth_blob(&self) -> Result<Option<String>> {
        Ok(None)
    }

    pub async fn set_auth_blob(&self, _value: String) -> Result<()> {
        Ok(())
    }

    pub async fn clear_auth_blob(&self) -> Result<()> {
        Ok(())
    }
}
__EOF__SECRETS__

cat > src/services/characterai.rs <<'__EOF__CAI__'
use anyhow::Result;

/// Placeholder CharacterAI service.
/// Next step: spawn a Node bridge that wraps `realcoloride/node_characterai`
/// and talk via stdin/stdout JSON messages.
#[derive(Clone, Default)]
pub struct CharacterAiService;

impl CharacterAiService {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_message(&self, _chat_id: &str, _text: &str) -> Result<String> {
        Ok("Not implemented yet".to_string())
    }
}
__EOF__CAI__

cat > src/ui/mod.rs <<'__EOF__UI_MOD__'
pub mod pages;

use gtk::prelude::*;
use crate::services::AppServices;

pub fn build_main_window(app: &gtk::Application, services: AppServices) -> gtk::ApplicationWindow {
    let header = gtk::HeaderBar::builder()
        .title_widget(&gtk::Label::new(Some("CharacterAI Client")))
        .show_title_buttons(true)
        .build();

    let sidebar = build_sidebar();
    let stack = build_stack(services);
    stack.set_visible_child_name("chat");

    let paned = gtk::Paned::builder()
        .orientation(gtk::Orientation::Horizontal)
        .start_child(&sidebar)
        .end_child(&stack)
        .wide_handle(true)
        .position(260)
        .build();

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("CharacterAI Client")
        .default_width(1100)
        .default_height(750)
        .build();

    window.set_titlebar(Some(&header));
    window.set_child(Some(&paned));

    sidebar.connect_row_selected(glib::clone!(@weak stack => move |_lb, row| {
        let Some(row) = row else { return; };
        match row.index() {
            0 => stack.set_visible_child_name("chat"),
            1 => stack.set_visible_child_name("settings"),
            _ => {}
        }
    }));

    window
}

fn build_sidebar() -> gtk::ListBox {
    let list = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::Single)
        .build();

    let row_chat = gtk::ListBoxRow::new();
    row_chat.set_child(Some(&gtk::Label::new(Some("Chat"))));
    list.append(&row_chat);

    let row_settings = gtk::ListBoxRow::new();
    row_settings.set_child(Some(&gtk::Label::new(Some("Settings"))));
    list.append(&row_settings);

    list
}

fn build_stack(services: AppServices) -> gtk::Stack {
    let stack = gtk::Stack::builder().hexpand(true).vexpand(true).build();

    let chat = pages::chat::ChatPage::new(services.clone()).widget();
    let settings = pages::settings::SettingsPage::new(services.clone()).widget();

    stack.add_named(&chat, Some("chat"));
    stack.add_named(&settings, Some("settings"));
    stack
}
__EOF__UI_MOD__

cat > src/ui/pages/mod.rs <<'__EOF__PAGES_MOD__'
pub mod chat;
pub mod settings;
__EOF__PAGES_MOD__

cat > src/ui/pages/chat.rs <<'__EOF__CHAT__'
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
__EOF__CHAT__

cat > src/ui/pages/settings.rs <<'__EOF__SETTINGS__'
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
__EOF__SETTINGS__

git add .
if git diff --cached --quiet; then
  echo "No changes to commit."
  exit 0
fi

git commit -m "Add GTK4 app skeleton (Debian/Kali, KDE/Wayland-friendly)"
git push origin main

echo "OK. Now run: cargo run"
