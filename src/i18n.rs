use i18n_embed::{fluent::fluent_language_loader, DesktopLanguageRequester, LanguageLoader};
use rust_embed::RustEmbed;
use std::sync::Arc;
use unic_langid::LanguageIdentifier;

#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

static LANGUAGE_LOADER: std::sync::OnceLock<Arc<i18n_embed::fluent::FluentLanguageLoader>> =
    std::sync::OnceLock::new();

pub fn init_i18n() -> Arc<i18n_embed::fluent::FluentLanguageLoader> {
    let loader = fluent_language_loader!();

    let requested_languages = DesktopLanguageRequester::requested_languages();

    i18n_embed::select(&loader, &Localizations, &requested_languages)
        .expect("Error cargando traducciones");

    let result = Arc::new(loader);
    let _ = LANGUAGE_LOADER.set(result.clone());
    result
}

pub fn get_loader() -> Arc<i18n_embed::fluent::FluentLanguageLoader> {
    LANGUAGE_LOADER.get().expect("i18n not initialized").clone()
}

pub fn get_available_languages() -> Vec<LanguageIdentifier> {
    let loader = get_loader();
    loader
        .available_languages(&Localizations)
        .expect("Error obteniendo idiomas disponibles")
}

pub fn set_language(lang: LanguageIdentifier) {
    let loader = get_loader();
    loader
        .load_languages(&Localizations, &[lang])
        .expect("Error cambiando idioma");
}
