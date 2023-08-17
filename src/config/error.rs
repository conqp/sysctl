use rocket::serde::json::serde_json;
use std::fmt::{Display, Formatter};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    ChromiumDefaultPreferencesNotFound,
    IoError(std::io::Error),
    NotAJsonObject(&'static str),
    SerdeError(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChromiumDefaultPreferencesNotFound => {
                write!(f, "Chrome / Chromium default preferences not found")
            }
            Self::IoError(error) => <std::io::Error as Display>::fmt(error, f),
            Self::NotAJsonObject(key) => write!(f, "not a JSON object: {key}"),
            Self::SerdeError(error) => <serde_json::Error as Display>::fmt(error, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SerdeError(error) => Some(error),
            Self::IoError(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeError(error)
    }
}

impl From<&'static str> for Error {
    fn from(key: &'static str) -> Self {
        Self::NotAJsonObject(key)
    }
}
