use std::str::FromStr;

use log::debug;
use quick_xml::events::Event as XmlEvent;
use quick_xml::{events::BytesStart, name::QName, Reader};
use serde::de;

use crate::bpmn::model::{
    errors::BPMNParseError,
    process::{
        AuditingInfo, CollaborationRef, ExecutableStatus, MonitoringInfo, Process, ProcessState,
        ProcessType,
    },
};
use crate::commons::xml::constants::XmlElementType;

use super::flow_objects::event_xml_parsers::parse_event_element;
use super::flow_objects::gateway_xml_parser::parse_gateway_element;
use super::flow_objects::task_xml_parsers::parse_task_element;
use super::sequence_flow_xml_parser::parse_sequence_flow_element;
use crate::commons::xml::utils::{extract_optional_attribute, extract_optional_bool_attribute};

/// Parses the `<process>` element.
pub(super) fn parse_process_element(
    process: &mut Process,
    xml_reader: &mut Reader<impl std::io::BufRead>,
    element: &BytesStart,
) -> Result<(), BPMNParseError> {
    debug!("Parsing <process> element");
    // Parse attributes
    let process_type_str = extract_optional_attribute(element, &QName(b"processType"))?;
    let process_type = match process_type_str {
        Some(s) => ProcessType::from_str(&s)?,
        None => ProcessType::default(),
    };
    debug!("Process Type: {:?}", process_type);

    let is_executable = extract_optional_bool_attribute(element, &QName(b"isExecutable"))?;
    debug!("Is Executable: {:?}", is_executable);

    let is_closed = extract_optional_bool_attribute(element, &QName(b"isClosed"))?.unwrap_or(false);
    debug!("Is Closed: {:?}", is_closed);

    let collaboration_ref =
        extract_optional_attribute(element, &QName(b"definitionalCollaborationRef"))?;
    debug!("Collaboration Ref: {:?}", collaboration_ref);

    // Update the process object with parsed attributes
    process
        .set_process_type(process_type)
        .set_executable_status(if is_executable.unwrap_or(false) {
            ExecutableStatus::Executable
        } else {
            ExecutableStatus::NonExecutable
        })
        .set_process_state(if is_closed {
            ProcessState::Closed
        } else {
            ProcessState::Open
        });

    if let Some(collab_ref) = collaboration_ref {
        process.set_definitional_collaboration_ref(CollaborationRef::Defined(collab_ref));
    }

    // Parse child elements
    parse_child_elements(process, xml_reader)?;

    Ok(())
}

/// Parses the child elements of `<process>`.
fn parse_child_elements(
    process: &mut Process,
    xml_reader: &mut Reader<impl std::io::BufRead>,
) -> Result<(), BPMNParseError> {
    debug!("Parsing child elements of <process>");

    let mut buf = Vec::new(); // Buffer for reading XML events
    loop {
        let event = xml_reader.read_event_into(&mut buf);
        debug!("<process> child event: {:?}", event);
        match event {
            // Start tag (with attributes) `<tag attr="value">`.
            Ok(XmlEvent::Start(ref element)) => {
                if let Ok(tag_name) = std::str::from_utf8(element.name().as_ref()) {
                    debug!("Parsing element: <{}>", tag_name);
                    match tag_name {
                        "task" => parse_task_element(process, element)?, // Support for tasks
                        "sequenceFlow" => parse_sequence_flow_element(
                            process,
                            xml_reader,
                            element,
                            XmlElementType::Start,
                        )?, // Add support for sequence flows
                        _ => Err(BPMNParseError::UnsupportedElement(tag_name.to_string()))?,
                    }
                }
            }
            Ok(XmlEvent::Empty(ref element)) => {
                if let Ok(tag_name) = std::str::from_utf8(element.name().as_ref()) {
                    debug!("Parsing element: <{}>", tag_name);
                    match tag_name {
                        "startEvent" => parse_event_element(process, element)?, // Add support for startEvent
                        "endEvent" => parse_event_element(process, element)?, // Add support for endEvent
                        "task" => parse_task_element(process, element)?,      // Handle tasks
                        "exclusiveGateway" => parse_gateway_element(process, xml_reader, element)?, // Handle gateway
                        "parallelGateway" => parse_gateway_element(process, xml_reader, element)?, // Handle gateway
                        "sequenceFlow" => parse_sequence_flow_element(
                            process,
                            xml_reader,
                            element,
                            XmlElementType::Empty,
                        )?, // Add support for sequence flows
                        _ => Err(BPMNParseError::UnsupportedElement(tag_name.to_string()))?,
                    }
                }
            }
            Ok(XmlEvent::End(ref e)) => {
                debug!(
                    "End element: </{}>",
                    std::str::from_utf8(e.name().as_ref())?
                );
                if let Ok(tag_name) = std::str::from_utf8(e.name().as_ref()) {
                    if tag_name == "process" {
                        debug!("End of <process> element");
                        break;
                    }
                }
            }
            Err(e) => {
                return Err(BPMNParseError::XmlParseError(format!(
                    "Error at position {}: {:?}",
                    xml_reader.error_position(),
                    e,
                )))
            }
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}

/// Parses the `<auditing>` element.
fn parse_auditing(process: &mut Process, element: &BytesStart) -> Result<(), BPMNParseError> {
    debug!("Parsing <auditing> element");
    let optional_auditing_info_str = extract_optional_attribute(element, &QName(b"auditing"))?;
    let auditing_info = match optional_auditing_info_str {
        Some(s) => AuditingInfo::Present(s),
        None => AuditingInfo::Absent,
    };
    process.set_auditing(auditing_info);
    Ok(())
}

/// Parses the `<monitoring>` element.
fn parse_monitoring(process: &mut Process, element: &BytesStart) -> Result<(), BPMNParseError> {
    debug!("Parsing <monitoring> element");
    let optional_monitoring_info_str = extract_optional_attribute(element, &QName(b"monitoring"))?;
    let monitoring_info = match optional_monitoring_info_str {
        Some(s) => MonitoringInfo::Enabled(s),
        None => MonitoringInfo::Disabled,
    };
    process.set_monitoring(monitoring_info);
    Ok(())
}

/// Helper function to parse `<processRole>` elements
fn parse_process_role(process: &mut Process, element: &BytesStart) -> Result<(), BPMNParseError> {
    debug!("Parsing <processRole> element");
    let role = extract_optional_attribute(element, &QName(b"name"))?;
    if let Some(role) = role {
        process.add_process_role(role);
    }
    Ok(())
}

/// Helper function to parse `<property>` elements
fn parse_property(process: &mut Process, element: &BytesStart) -> Result<(), BPMNParseError> {
    debug!("Parsing <property> element");
    let property = extract_optional_attribute(element, &QName(b"property"))?;
    if let Some(property) = property {
        process.add_property(property);
    }
    Ok(())
}

/// Helper function to parse `<laneSet>` elements
fn parse_lane_set(process: &mut Process, element: &BytesStart) -> Result<(), BPMNParseError> {
    debug!("Parsing <laneSet> element");
    let lane_set = extract_optional_attribute(element, &QName(b"laneSet"))?;
    if let Some(lane_set) = lane_set {
        process.add_lane_set(lane_set);
    }
    Ok(())
}

/// Helper function to parse `<artifact>` elements
fn parse_artifact(process: &mut Process, element: &BytesStart) -> Result<(), BPMNParseError> {
    debug!("Parsing <artifact> element");
    let artifact = extract_optional_attribute(element, &QName(b"artifact"))?;
    if let Some(artifact) = artifact {
        process.add_artifact(artifact);
    }
    Ok(())
}

/// Helper function to parse `<resourceRole>` elements
fn parse_resource_role(process: &mut Process, element: &BytesStart) -> Result<(), BPMNParseError> {
    debug!("Parsing <resourceRole> element");
    let resource_role = extract_optional_attribute(element, &QName(b"resourceRole"))?;
    if let Some(resource_role) = resource_role {
        process.add_resource_role(resource_role);
    }
    Ok(())
}

/// Helper function to parse `<correlationSubscription>` elements
fn parse_correlation_subscription(
    process: &mut Process,
    element: &BytesStart,
) -> Result<(), BPMNParseError> {
    debug!("Parsing <correlationSubscription> element");
    let subscription = extract_optional_attribute(element, &QName(b"correlationSubscription"))?;
    if let Some(subscription) = subscription {
        process.add_correlation_subscription(subscription);
    }
    Ok(())
}

/// Helper function to parse `<supports>` elements
fn parse_support(process: &mut Process, element: &BytesStart) -> Result<(), BPMNParseError> {
    debug!("Parsing <supports> element");
    let support = extract_optional_attribute(element, &QName(b"supports"))?;
    if let Some(support) = support {
        process.add_support(support);
    }
    Ok(())
}
