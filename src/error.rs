use std::fmt;

#[derive(Debug)]
pub enum ConversionError {
    Json(serde_json::Error),
    Xml(quick_xml::Error),
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    Toml(toml::de::Error),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::Json(e) => write!(f, "JSON error: {}", e),
            ConversionError::Xml(e) => write!(f, "XML error: {}", e),
            ConversionError::Io(e) => write!(f, "IO error: {}", e),
            ConversionError::Utf8(e) => write!(f, "UTF-8 conversion error: {}", e),
            ConversionError::Toml(e) => write!(f, "TOML error: {}", e),
        }
    }
}

impl From<serde_json::Error> for ConversionError {
    fn from(err: serde_json::Error) -> ConversionError {
        ConversionError::Json(err)
    }
}

impl From<quick_xml::Error> for ConversionError {
    fn from(err: quick_xml::Error) -> ConversionError {
        ConversionError::Xml(err)
    }
}

impl From<std::io::Error> for ConversionError {
    fn from(err: std::io::Error) -> ConversionError {
        ConversionError::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for ConversionError {
    fn from(err: std::string::FromUtf8Error) -> ConversionError {
        ConversionError::Utf8(err)
    }
}

impl From<toml::de::Error> for ConversionError {
    fn from(err: toml::de::Error) -> ConversionError {
        ConversionError::Toml(err)
    }
}
