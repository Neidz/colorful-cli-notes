use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Options {
    pub default_theme_color: String,
}

#[derive(Deserialize, Serialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub links: Vec<String>,
    pub theme: String,
}

#[derive(Deserialize, Serialize)]
pub struct JsonFileStructure {
    pub options: Options,
    pub notes: Vec<Note>,
}

