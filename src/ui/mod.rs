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
