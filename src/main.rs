pub mod actions;
pub mod cli;
pub mod cli_handler;
pub mod commands;
pub mod config;
pub mod dns;
pub mod embed_data;
pub mod gresource;
pub mod gui;
pub mod i18n;
pub mod installer;
pub mod kwin_dbus;
pub mod localization;
pub mod logger;
pub mod pages;
pub mod systemd_units;
pub mod tweak;
pub mod ui;
pub mod utils;
pub mod window;

pub use config::{APP_ID, PROFILE};
pub use utils::{check_regular_file, fix_path, read_json, write_json, PacmanWrapper};
pub use window::HelloWindow;

pub use std::path::Path;
pub use std::str;
pub use std::sync::{Arc, LazyLock, Mutex};

pub use gtk::gio::ApplicationFlags;
pub use gtk::prelude::*;

pub use clap::Parser;
pub use gtk::glib;
pub use gtk::glib::clone;
pub use i18n_embed::DesktopLanguageRequester;
pub use i18n_embed::LanguageLoader;
pub use i18n_embed_fl::fl;
pub use serde_json::json;
pub use tracing::{debug, error};
pub use unic_langid::LanguageIdentifier;

fn main() {
    i18n::init_i18n();

    let app = gtk::Application::new(Some("com.coreteam.core-tools"), ApplicationFlags::default());

    app.connect_activate(|app| {
        let builder = gtk::Builder::from_file("ui/core-tools.glade");

        let window: gtk::ApplicationWindow = builder.object("main_window").unwrap();
        window.set_application(Some(app));

        let title_label: gtk::Label = builder.object("title_label").unwrap();
        let subtitle_label: gtk::Label = builder.object("subtitle_label").unwrap();

        title_label.set_label(&fl!(i18n::get_loader(), "welcome-title"));
        subtitle_label.set_label(&fl!(i18n::get_loader(), "welcome-body"));

        let update_button: gtk::Button = builder.object("update_button").unwrap();
        update_button.set_label(&fl!(i18n::get_loader(), "update_system"));

        update_button.connect_clicked(|_| {
            crate::commands::actualizar_sistema();
        });

        let clean_cache_button: gtk::Button = builder.object("clean_cache_button").unwrap();
        clean_cache_button.set_label(&fl!(i18n::get_loader(), "clean_cache"));

        clean_cache_button.connect_clicked(|_| {
            crate::commands::limpiar_cache();
        });

        let remove_orphans_button: gtk::Button = builder.object("remove_orphans_button").unwrap();
        remove_orphans_button.set_label(&fl!(i18n::get_loader(), "remove_orphans"));

        remove_orphans_button.connect_clicked(|_| {
            crate::commands::eliminar_huerfanos();
        });

        let rate_mirrors_button: gtk::Button = builder.object("rate_mirrors_button").unwrap();
        rate_mirrors_button.set_label(&fl!(i18n::get_loader(), "rate_mirrors"));

        rate_mirrors_button.connect_clicked(|_| {
            crate::commands::evaluar_mirrors();
        });

        let lang_combo: gtk::ComboBoxText = builder.object("languages").unwrap();

        let loader = i18n::get_loader();
        let current_lang = loader.current_language();

        let current_lang_code = current_lang.language.to_string();
        let current_display = match current_lang_code.as_str() {
            "en" => "English",
            "es" => "Español",
            _ => &current_lang_code,
        };

        let model = lang_combo.model();
        if let Some(model) = model {
            let n_rows = model.iter_n_children(None);
            for i in 0..n_rows {
                if let Some(iter) = model.iter_nth_child(None, i) {
                    let value = model.value(&iter, 0);
                    if let Ok(text) = value.get::<glib::GString>() {
                        if text == current_display {
                            lang_combo.set_active(Some(i as u32));
                            break;
                        }
                    }
                }
            }
        }

        lang_combo.connect_changed(
            clone!(@weak title_label, @weak subtitle_label, @weak update_button, @weak clean_cache_button, @weak remove_orphans_button, @weak rate_mirrors_button => move |combo| {
                if let Some(text) = combo.active_text() {
                    let display = text.to_string();
                    let lang_map: std::collections::HashMap<String, LanguageIdentifier> = [
                        ("English".to_string(), "en".parse().unwrap()),
                        ("Español".to_string(), "es".parse().unwrap()),
                    ].into_iter().collect();

                    if let Some(lang) = lang_map.get(&display) {
                        i18n::set_language(lang.clone());

                        title_label.set_label(&fl!(i18n::get_loader(), "welcome-title"));
                        subtitle_label.set_label(&fl!(i18n::get_loader(), "welcome-body"));
                        update_button.set_label(&fl!(i18n::get_loader(), "update_system"));
                        clean_cache_button.set_label(&fl!(i18n::get_loader(), "clean_cache"));
                        remove_orphans_button.set_label(&fl!(i18n::get_loader(), "remove_orphans"));
                        rate_mirrors_button.set_label(&fl!(i18n::get_loader(), "rate_mirrors"));
                    }
                }
            }),
        );

        window.show_all();
    });

    app.run();
}
