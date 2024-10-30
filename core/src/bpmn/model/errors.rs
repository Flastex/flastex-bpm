// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

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

#[cfg(test)]
mod tests {
    use quick_xml::events::attributes::AttrError;
    use quick_xml::Error as XmlError;

    use super::*;

    #[test]
    fn test_attr_error_conversion() {
        let attr_error = AttrError::ExpectedEq(123);
        let bpmn_error: BPMNParseError = attr_error.into();
        match bpmn_error {
            BPMNParseError::XmlParseError(msg) => assert_eq!(
                msg,
                "position 123: attribute key must be directly followed by `=` or space"
            ),
            _ => panic!("Expected XmlParseError"),
        }
    }

    #[test]
    fn test_xml_error_conversion() {
        let attr_error = AttrError::ExpectedEq(123);
        let xml_error = XmlError::InvalidAttr(attr_error);
        let bpmn_error: BPMNParseError = xml_error.into();
        match bpmn_error {
            BPMNParseError::XmlParseError(msg) => assert_eq!(msg, "error while parsing attribute: position 123: attribute key must be directly followed by `=` or space"),
            _ => panic!("Expected XmlParseError"),
        }
    }

    #[test]
    fn test_enum_parse_error_conversion() {
        let enum_error = EnumParseError {
            message: "VariantNotFound".to_string(),
        };
        assert_eq!(enum_error.message, "VariantNotFound");

        let bpmn_error: BPMNParseError = enum_error.into();
        match bpmn_error {
            BPMNParseError::EnumParseError(e) => {
                assert_eq!(e.message, "VariantNotFound")
            }
            _ => panic!("Expected EnumParseError"),
        }
    }

    #[test]
    fn test_utf8_conversion_error_from_utf8() {
        let utf8_error = String::from_utf8(vec![0, 159, 146, 150]).unwrap_err();
        let bpmn_error: BPMNParseError = utf8_error.into();
        match bpmn_error {
            BPMNParseError::Utf8ConversionError(Utf8ConversionError::FromUtf8(_)) => (),
            _ => panic!("Expected Utf8ConversionError::FromUtf8"),
        }
    }

    #[test]
    fn test_utf8_conversion_error_utf8() {
        let utf8_error = std::str::from_utf8(&vec![0, 159, 146, 150]).unwrap_err();
        let bpmn_error: BPMNParseError = utf8_error.into();
        match bpmn_error {
            BPMNParseError::Utf8ConversionError(Utf8ConversionError::Utf8(_)) => (),
            _ => panic!("Expected Utf8ConversionError::Utf8"),
        }
    }

    #[test]
    fn test_process_error_conversion() {
        let process_error = ProcessError::SequenceFlowNotFound("test process error".to_string());
        let bpmn_error: BPMNParseError = process_error.into();
        match bpmn_error {
            BPMNParseError::ProcessError(_) => (),
            _ => panic!("Expected ProcessError"),
        }
    }

    #[test]
    fn test_display_enum_parse_error() {
        let error = EnumParseError {
            message: "test error".to_string(),
        };
        assert_eq!(format!("{}", error), "Enum parse error: test error");
    }

    #[test]
    fn test_display_utf8_conversion_error() {
        let from_utf8_error =
            Utf8ConversionError::FromUtf8(String::from_utf8(vec![0, 159, 146, 150]).unwrap_err());
        assert_eq!(
            format!("{}", from_utf8_error),
            "UTF-8 conversion error: invalid utf-8 sequence of 1 bytes from index 1"
        );

        let utf8_error =
            Utf8ConversionError::Utf8(std::str::from_utf8(&vec![0, 159, 146, 150]).unwrap_err());
        assert_eq!(
            format!("{}", utf8_error),
            "UTF-8 error: invalid utf-8 sequence of 1 bytes from index 1"
        );
    }

    #[test]
    fn test_display_bpmn_parse_error() {
        let enum_error = BPMNParseError::EnumParseError(EnumParseError {
            message: "enum error".to_string(),
        });
        assert_eq!(format!("{}", enum_error), "Enum parse error: enum error");

        let process_error = BPMNParseError::ProcessError(ProcessError::FlowObjectAlreadyExists(
            "process error".to_string(),
        ));
        assert_eq!(
            format!("{}", process_error),
            "Process error: FlowObjectAlreadyExists"
        );

        let unsupported_element = BPMNParseError::UnsupportedElement("unsupported".to_string());
        assert_eq!(
            format!("{}", unsupported_element),
            "Unsupported node: unsupported"
        );

        let utf8_conversion_error = BPMNParseError::Utf8ConversionError(
            Utf8ConversionError::FromUtf8(String::from_utf8(vec![0, 159, 146, 150]).unwrap_err()),
        );
        assert_eq!(
            format!("{}", utf8_conversion_error),
            "UTF-8 conversion error: UTF-8 conversion error: invalid utf-8 sequence of 1 bytes from index 1"
        );

        let xml_parse_error = BPMNParseError::XmlParseError("xml parse error".to_string());
        assert_eq!(
            format!("{}", xml_parse_error),
            "XML parsing error: xml parse error"
        );

        let xml_content_error = BPMNParseError::XmlContentError("xml content error".to_string());
        assert_eq!(
            format!("{}", xml_content_error),
            "XML content error: xml content error"
        );
    }
}
