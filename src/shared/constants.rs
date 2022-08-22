use fluent_templates::LanguageIdentifier;
use unic_langid::langid;

pub const APP_NAME: &str = "Snap Up";

pub const COORDINATES_MULTIPLICATOR: f64 = 1000000.0;

pub const US_ENGLISH: LanguageIdentifier = langid!("en-US");

pub const LOCALES_PATH: &str = "./locales";
pub const FALLBACK_LANG: &str = "en-US";
