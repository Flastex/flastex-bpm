// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::str::FromStr;

use quick_xml::name::QName;

use crate::{bpmn::model::{errors::BPMNParseError, flow_objects::{event::{Event, EventType}, gateway::{Gateway, GatewayType}, task::{Task, TaskType}, FlowObject, FlowObjectType}, process::Process}, commons::xml::utils::extract_tag_name};
use crate::commons::xml::utils::extract_attribute;

const ID_QNAME: QName = QName(b"id");
const NAME_QNAME: QName = QName(b"name");
const TYPE_QNAME: QName = QName(b"type");

/// function to parse task elements and add them to the process
pub(super) fn parse_task_element(process: &mut Process, element: &quick_xml::events::BytesStart) -> Result<(), BPMNParseError> {
    let id = extract_attribute(element, &ID_QNAME);
    let name = extract_attribute(element, &NAME_QNAME);
    let task_type_str = extract_attribute(element, &TYPE_QNAME);
    let task_type = TaskType::from_str(&task_type_str)?;

    let task = Task::new(&id, &name, &task_type);
    let flow_object = FlowObject {
        id: id.clone(),
        flow_object_type: FlowObjectType::Task,
        flow_object_behavior: Box::new(task),
    };
    process.add_flow_object(flow_object)?;
    Ok(())
}

/// function to parse event elements and add them to the process
pub(crate) fn parse_event_element(process: &mut Process, element: &quick_xml::events::BytesStart) -> Result<(), BPMNParseError> {
    let tag_name = extract_tag_name(element)
        .map_err(|err| BPMNParseError::XmlParseError(err.to_string()))?;  
    let id = extract_attribute(element, &ID_QNAME);
    let name = extract_attribute(element, &NAME_QNAME);
    let event_type = EventType::from_str(&tag_name)?;

    let event = Event::new(&id, &name, &event_type);
    let flow_object = FlowObject {
        id: id.clone(),
        flow_object_type: FlowObjectType::Event,
        flow_object_behavior: Box::new(event),
    };
    process.add_flow_object(flow_object)?;
    Ok(())
}

/// function to parse gateway elements and add them to the process
pub(crate) fn parse_gateway_element(process: &mut Process, element: &quick_xml::events::BytesStart) -> Result<(), BPMNParseError> {
    let id = extract_attribute(element, &ID_QNAME);
    let name = extract_attribute(element, &NAME_QNAME);
    let gateway_type_str = extract_attribute(element, &TYPE_QNAME);
    let gateway_type = GatewayType::from_str(&gateway_type_str)?;

    let gateway = Gateway::new(&id, &name, &gateway_type);
    let flow_object = FlowObject {
        id: id.clone(),
        flow_object_type: FlowObjectType::Gateway,
        flow_object_behavior: Box::new(gateway),
    };
    process.add_flow_object(flow_object)?;
    Ok(())
}
