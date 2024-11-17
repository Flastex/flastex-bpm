// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use quick_xml::events::BytesStart;
use quick_xml::name::QName;
use std::str;

use crate::bpmn::model::errors::BPMNParseError;

/// Extracts the tag name from a BytesStart element
/// Returns a Result with the tag name as a String
pub(crate) fn extract_tag_name(element: &BytesStart) -> Result<String, BPMNParseError> {
    let name = element.name();
    let tag_bytes = name.as_ref();
    let tag_name = str::from_utf8(tag_bytes)
        .map_err(|e| BPMNParseError::XmlParseError(format!("Tag name parsing error: {:?}", e)))?;
    Ok(tag_name.to_string())
}

/// Extracts an attribute value from an XML element
/// Returns the attribute value as a Result<String>
pub(crate) fn extract_attribute(
    element: &BytesStart<'_>,
    qname: &QName,
) -> Result<String, BPMNParseError> {
    let attr = element
        .attributes()
        .find(|attr| match attr {
            Ok(attr) => attr.key == *qname,
            Err(_) => false,
        })
        .ok_or_else(|| attribute_not_found_error(qname))??;

    let value = attr
        .unescape_value()
        .map_err(|_| BPMNParseError::XmlParseError("Failed to unescape value".into()))?;

    Ok(value.to_string())
}

/// Extracts an optional attribute value from an XML element
/// Returns the attribute value as Result<Option<String>>
pub(crate) fn extract_optional_attribute(
    element: &BytesStart<'_>,
    qname: &QName,
) -> Result<Option<String>, BPMNParseError> {
    if let Some(attr) = element.attributes().find(|attr| match attr {
        Ok(attr) => attr.key == *qname,
        Err(_) => false,
    }) {
        let value = attr?.unescape_value().map_err(|_| {
            BPMNParseError::XmlParseError("Failed to unescape optional attribute".into())
        })?;
        return Ok(Some(value.to_string()));
    }
    Ok(None)
}

/// Extracts a boolean attribute value from an XML element.
/// Returns a Result with the attribute value as a boolean.
pub(crate) fn extract_bool_attribute(
    element: &BytesStart<'_>,
    qname: &QName,
) -> Result<bool, BPMNParseError> {
    let attr = element
        .attributes()
        .find(|attr| match attr {
            Ok(attr) => attr.key == *qname,
            Err(_) => false,
        })
        .ok_or_else(|| attribute_not_found_error(qname))??; // Return early if attribute is not found or malformed.

    let value = attr
        .unescape_value()
        .map_err(|_| BPMNParseError::XmlParseError("Failed to unescape value".into()))?
        .to_ascii_lowercase();

    match value.as_ref() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(invalid_boolean_value_error(value.as_ref())),
    }
}

/// Extracts an optional boolean attribute value from an XML element
/// Returns a Result<Option<bool>> with the attribute value
pub(crate) fn extract_optional_bool_attribute(
    element: &BytesStart<'_>,
    qname: &QName,
) -> Result<Option<bool>, BPMNParseError> {
    if let Some(attr) = element.attributes().find(|attr| match attr {
        Ok(attr) => attr.key == *qname,
        Err(_) => false,
    }) {
        let value = attr?
            .unescape_value()
            .map_err(|_| {
                BPMNParseError::XmlParseError(
                    "Failed to unescape optional boolean attribute".into(),
                )
            })?
            .to_ascii_lowercase();

        match value.as_ref() {
            "true" => Ok(Some(true)),
            "false" => Ok(Some(false)),
            _ => Ok(None), // Invalid values are treated as None
        }
    } else {
        Ok(None) // Attribute not found is also treated as None
    }
}

fn attribute_not_found_error(qname: &QName) -> BPMNParseError {
    BPMNParseError::XmlParseError(format!("Attribute {:?} not found", qname))
}

fn invalid_boolean_value_error(value: &str) -> BPMNParseError {
    BPMNParseError::XmlParseError(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::events::BytesStart;
    use quick_xml::name::QName;

    // Helper function to create a `BytesStart` element with attributes
    fn create_element_with_attributes() -> BytesStart<'static> {
        let mut element = BytesStart::new("testElement");
        element.push_attribute(("attribute1", "value1"));
        element.push_attribute(("attribute2", "true"));
        element.push_attribute(("attribute3", "42"));
        element
    }

    #[test]
    fn test_extract_tag_name() {
        // ARRANGE: Create a `BytesStart` element
        let element = BytesStart::new("exampleTag");

        // ACT: Extract the tag name using the function
        let tag_name = extract_tag_name(&element).unwrap();

        // ASSERT: Verify the extracted tag name is correct
        assert_eq!(tag_name, "exampleTag");
    }

    #[test]
    fn test_extract_attribute() {
        // ARRANGE: Create an element with attributes
        let element = create_element_with_attributes();

        // ACT: Extract a known attribute
        let value = extract_attribute(&element, &QName(b"attribute1")).unwrap();

        // ASSERT: Verify that the extracted attribute is correct
        assert_eq!(value, "value1");

        // ACT: Try extracting a non-existent attribute and expect a panic
        let result = extract_attribute(&element, &QName(b"nonExistentAttr"));

        // ASSERT: Ensure the function panics as expected
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_optional_attribute() {
        // ARRANGE: Create an element with attributes
        let element = create_element_with_attributes();

        // ACT: Extract a known optional attribute
        let value = extract_optional_attribute(&element, &QName(b"attribute1")).unwrap();

        // ASSERT: Verify the extracted optional attribute is correct
        assert_eq!(value, Some("value1".to_string()));

        // ACT: Try extracting a non-existent optional attribute
        let missing_value =
            extract_optional_attribute(&element, &QName(b"nonExistentAttr")).unwrap();

        // ASSERT: Verify that the result is None for non-existent attribute
        assert_eq!(missing_value, None);
    }

    #[test]
    fn test_extract_bool_attribute() {
        // ARRANGE: Create an element with attributes
        let element = create_element_with_attributes();

        // ACT: Extract a boolean attribute
        let value = extract_bool_attribute(&element, &QName(b"attribute2")).unwrap();

        // ASSERT: Verify the boolean attribute is correct
        assert!(value);

        // ARRANGE: Create an element with invalid boolean value
        let mut element_invalid_bool = BytesStart::new("elementWithInvalidBool");
        element_invalid_bool.push_attribute(("attribute", "not_a_bool"));

        // ACT: Try extracting an invalid boolean value
        let result = extract_bool_attribute(&element_invalid_bool, &QName(b"attribute"));

        // ASSERT: Verify the extraction fails with an error
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_optional_bool_attribute() {
        // ARRANGE: Create an element with attributes
        let element = create_element_with_attributes();

        // ACT: Extract an optional boolean attribute
        let value = extract_optional_bool_attribute(&element, &QName(b"attribute2")).unwrap();

        // ASSERT: Verify the optional boolean attribute is correct
        assert_eq!(value, Some(true));

        // ACT: Try extracting a non-existent optional boolean attribute
        let missing_value =
            extract_optional_bool_attribute(&element, &QName(b"nonExistentAttr")).unwrap();

        // ASSERT: Verify that the result is None for non-existent attribute
        assert_eq!(missing_value, None);

        // ARRANGE: Create an element with invalid boolean value
        let mut element_invalid_bool = BytesStart::new("elementWithInvalidBool");
        element_invalid_bool.push_attribute(("attribute", "not_a_bool"));

        // ACT: Try extracting an invalid optional boolean attribute
        let invalid_value =
            extract_optional_bool_attribute(&element_invalid_bool, &QName(b"attribute")).unwrap();

        // ASSERT: Verify the invalid boolean returns None
        assert_eq!(invalid_value, None);
    }
}
