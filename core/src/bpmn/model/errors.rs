use std::{fmt, str::Utf8Error, string::FromUtf8Error};

use log::debug;

use super::process::ProcessError;

/// Converts Quick XML attribute error
impl From<quick_xml::events::attributes::AttrError> for BPMNParseError {
    fn from(error: quick_xml::events::attributes::AttrError) -> Self {
        debug!("Quick XML error parsing attribute: {:?}", error);
        BPMNParseError::XmlParseError(error.to_string())
    }
}
/// Converts Quick XML error
impl From<quick_xml::errors::Error> for BPMNParseError {
    fn from(error: quick_xml::errors::Error) -> Self {
        debug!("Quick XML error: {:?}", error);
        BPMNParseError::XmlParseError(error.to_string())
    }
}

// Enum Parse Error
#[derive(Debug, PartialEq, Eq)]
pub struct EnumParseError {
    pub message: String,
}

impl From<strum::ParseError> for EnumParseError {
    fn from(error: strum::ParseError) -> Self {
        EnumParseError {
            message: format!("Failed to parse enum: {}", error),
        }
    }
}

impl From<strum::ParseError> for BPMNParseError {
    fn from(error: strum::ParseError) -> Self {
        BPMNParseError::EnumParseError(EnumParseError::from(error))
    }
}

impl From<EnumParseError> for BPMNParseError {
    fn from(error: EnumParseError) -> Self {
        BPMNParseError::EnumParseError(error)
    }
}

impl fmt::Display for EnumParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Enum parse error: {}", self.message)
    }
}

impl From<ProcessError> for BPMNParseError {
    fn from(err: ProcessError) -> Self {
        BPMNParseError::ProcessError(err)
    }
}

#[derive(Debug)]
pub enum Utf8ConversionError {
    FromUtf8(FromUtf8Error),
    Utf8(Utf8Error),
}

impl fmt::Display for Utf8ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Utf8ConversionError::FromUtf8(e) => write!(f, "UTF-8 conversion error: {}", e),
            Utf8ConversionError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
        }
    }
}

impl From<FromUtf8Error> for BPMNParseError {
    fn from(error: FromUtf8Error) -> Self {
        BPMNParseError::Utf8ConversionError(Utf8ConversionError::FromUtf8(error))
    }
}

impl From<Utf8Error> for BPMNParseError {
    fn from(error: Utf8Error) -> Self {
        BPMNParseError::Utf8ConversionError(Utf8ConversionError::Utf8(error))
    }
}

// Define the main error for BPMN parsing
#[derive(Debug)]
pub enum BPMNParseError {
    EnumParseError(EnumParseError),
    ProcessError(ProcessError),
    UnsupportedElement(String),
    Utf8ConversionError(Utf8ConversionError),
    XmlParseError(String),
    XmlContentError(String),
}

impl std::error::Error for BPMNParseError {}

impl fmt::Display for BPMNParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BPMNParseError::EnumParseError(e) => write!(f, "Enum parse error: {}", e.message),
            BPMNParseError::ProcessError(e) => write!(f, "Process error: {}", e),
            BPMNParseError::UnsupportedElement(node) => write!(f, "Unsupported node: {}", node),
            BPMNParseError::Utf8ConversionError(e) => write!(f, "UTF-8 conversion error: {}", e),
            BPMNParseError::XmlParseError(e) => write!(f, "XML parsing error: {}", e),
            BPMNParseError::XmlContentError(e) => write!(f, "XML content error: {}", e),
        }
    }
}
