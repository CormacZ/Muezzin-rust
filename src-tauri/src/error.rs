use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not initialized")]
    NotInitialized,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Audio error: {0}")]
    Rodio(String),

    #[error("Prayer calculation error: {0}")]
    Prayer(String),

    #[error("Timezone parse error: {0}")]
    Timezone(String),

    #[error("Store error: {0}")]
    Store(#[from] tauri_plugin_store::Error),

    #[error("DateTime parse error: {0}")]
    ChronoParse(#[from] chrono::ParseError),

    #[error("{0}")]
    Custom(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
