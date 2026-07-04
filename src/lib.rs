#[cfg(feature = "editor")]
pub mod editor;
#[cfg(feature = "khipro")]
pub mod khipro;
pub mod parser;
pub mod phonetic;
#[cfg(feature = "regex")]
pub mod regex_suggestion;
