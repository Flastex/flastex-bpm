// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use crate::bpmn::model::connecting_objects::sequence_flows::{ImmediateFlag, SequenceFlow};
use crate::bpmn::model::errors::BPMNParseError;
use crate::bpmn::model::script::{Script, ScriptType};
use crate::commons::xml::constants::XmlElementType;
use log::debug;
use quick_xml::events::{BytesStart, Event as XmlEvent};
use quick_xml::name::QName;
use quick_xml::reader::Reader;

use crate::bpmn::model::process::Process;

use crate::commons::xml::utils::{
    extract_attribute, extract_optional_attribute, extract_optional_bool_attribute,
};

/// Parses a sequence flow element and adds it to the process.
pub fn parse_sequence_flow_element(
    process: &mut Process,
    xml_reader: &mut Reader<impl std::io::BufRead>,
    element: &BytesStart<'_>,
    xml_element_type: XmlElementType,
) -> Result<(), BPMNParseError> {
    debug!("Parsing sequence flow element");
    debug!("Element type: {:?}", xml_element_type);
    let mut sequence_flow = SequenceFlow::new();

    let id = extract_attribute(element, &QName(b"id"))?;
    sequence_flow.set_id(id);
    let source_ref = extract_attribute(element, &QName(b"sourceRef"))?;
    sequence_flow.set_source_ref(source_ref);
    let target_ref = extract_attribute(element, &QName(b"targetRef"))?;
    sequence_flow.set_target_ref(target_ref);
    let is_immediate = extract_optional_bool_attribute(element, &QName(b"isImmediate"))?;
    sequence_flow.set_is_immediate(match is_immediate {
        Some(b) => match b {
            true => ImmediateFlag::Immediate,
            false => ImmediateFlag::NonImmediate,
        },
        None => ImmediateFlag::NonImmediate,
    });

    let script_condition = extract_conditional_flow(xml_reader, xml_element_type)?;
    if script_condition.is_some() {
        // If a condition is present, set the sequence flow as ConditionalFlow
        sequence_flow.as_conditional(script_condition);
    } else {
        // Otherwise, default to NormalFlow (it can be a default flow at runtime)
        sequence_flow.as_normal();
    }

    process.add_sequence_flow(sequence_flow);
    return Ok(());
}

pub fn extract_conditional_flow(
    xml_reader: &mut Reader<impl std::io::BufRead>,
    xml_element_type: XmlElementType,
) -> Result<Option<Script>, BPMNParseError> {
    if xml_element_type == XmlElementType::Empty {
        return Ok(None);
    }
    debug!("Parsing conditional flow");

    let mut sequence_flow_element_buf = Vec::new();

    let mut language: Option<ScriptType> = None;
    loop {
        let event = xml_reader.read_event_into(&mut sequence_flow_element_buf);
        debug!("<sequenceFlow> child event: {:?}", event);
        match event {
            Ok(XmlEvent::Start(ref element)) => {
                debug!("Found start element: {:?}", element);
                let name = element.name();
                if name == QName(b"conditionExpression") {
                    debug!("Found conditionExpression");
                    if let Some(language_schema_ref) =
                        extract_optional_attribute(element, &QName(b"language"))?
                    {
                        debug!("Found language: {:?}", language_schema_ref);
                        language = Some(ScriptType::from_schema_ref(&language_schema_ref)?);
                    }

                    let _evaluates_to_type_ref =
                        extract_optional_attribute(element, &QName(b"evaluatesToTypeRef"))?;
                    if (_evaluates_to_type_ref).is_some() {
                        return Err(BPMNParseError::XmlParseError(
                            "evaluatesToTypeRef not supported".to_string(),
                        ));
                    }
                    break;
                } else {
                    debug!(
                        "Expected Event::Start for conditionExpression, got: {:?}",
                        &event
                    );
                    let name_string = xml_reader
                        .decoder()
                        .decode(name.as_ref())
                        .map_err(|_| {
                            BPMNParseError::XmlParseError("Failed to decode name".to_string())
                        })?
                        .into_owned();
                    return Err(BPMNParseError::XmlParseError(format!(
                        "Expected conditionExpression, got {name_string}"
                    )));
                }
            }
            Ok(XmlEvent::Comment(_)) => {
                continue;
            }
            Ok(XmlEvent::PI(_)) => {
                continue;
            }
            Err(_) => {
                return Err(BPMNParseError::XmlParseError(
                    "Failed to read event".to_string(),
                ));
            }
            _ => {
                break;
            }
        }
    }

    let script_language = language.take().unwrap_or_else(ScriptType::default);
    debug!("Script language: {:?}", script_language);
    loop {
        let event = xml_reader.read_event_into(&mut sequence_flow_element_buf);
        debug!("<sequenceFlow> child event: {:?}", event);
        match event {
            Ok(XmlEvent::CData(ref cdata)) => {
                debug!("Found CDATA: {:?}", cdata);
                let condition = String::from_utf8(cdata.to_vec())
                    .map_err(|_| BPMNParseError::XmlParseError("Failed to read CDATA".to_string()))?
                    .trim() // This will remove leading and trailing whitespace and newlines
                    .to_string();
                return Ok(Some(Script::new(&condition, &script_language)));
            }
            Ok(XmlEvent::Comment(_)) => {
                continue;
            }
            Ok(XmlEvent::PI(_)) => {
                continue;
            }
            Err(_) => {
                return Err(BPMNParseError::XmlParseError(
                    "Failed to read event".to_string(),
                ));
            }
            _ => {
                break;
            }
        }
    }

    return Ok(Some(Script::new(
        String::default().as_str(),
        &script_language,
    )));
}
