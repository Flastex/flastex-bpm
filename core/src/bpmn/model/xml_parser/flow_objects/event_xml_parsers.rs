use std::str::FromStr;

use log::debug;
use quick_xml::name::QName;

use crate::commons::xml::utils::extract_attribute;
use crate::{
    bpmn::model::{
        errors::BPMNParseError,
        flow_objects::{
            event::{Event, EventType},
            FlowObject, FlowObjectType,
        },
        process::Process,
    },
    commons::xml::utils::extract_tag_name,
};

/// function to parse event elements and add them to the process
pub(crate) fn parse_event_element(
    process: &mut Process,
    element: &quick_xml::events::BytesStart,
) -> Result<(), BPMNParseError> {
    debug!("Parsing <event> element");
    let tag_name =
        extract_tag_name(element).map_err(|err| BPMNParseError::XmlParseError(err.to_string()))?;
    let id = extract_attribute(element, &QName(b"id"))?;
    let name = extract_attribute(element, &QName(b"name"))?;
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
