use std::str::FromStr;

use crate::bpmn::model::flow_objects::{
    FlowObject, FlowObjectBehavior, FlowObjectId, FlowObjectType,
};

use crate::bpmn::model::flow_objects::gateway::{
    EventBasedGatewayType, ExclusiveGateway, GatewayDirection, ParallelGateway,
};
use crate::bpmn::model::{
    errors::BPMNParseError,
    flow_objects::gateway::{ComplexGateway, EventBasedGateway, GatewayType, InclusiveGateway},
    process::Process,
};
use crate::commons::xml::utils::{
    extract_attribute, extract_bool_attribute, extract_optional_attribute, extract_tag_name,
};
use log::debug;
use quick_xml::events::{BytesStart, Event as XmlEvent};
use quick_xml::name::QName;
use quick_xml::Reader;
use serde::de;

/// Function to parse any gateway element and add it to the process.
pub(crate) fn parse_gateway_element(
    process: &mut Process,
    xml_reader: &mut Reader<impl std::io::BufRead>,
    element: &BytesStart,
) -> Result<(), BPMNParseError> {
    debug!("Parsing <***gateway> element");
    let tag_name =
        extract_tag_name(element).map_err(|err| BPMNParseError::XmlParseError(err.to_string()))?;
    let id: FlowObjectId = extract_attribute(element, &QName(b"id"))?;
    let name = extract_attribute(element, &QName(b"name"))?;
    let gateway_type = GatewayType::from_str(&tag_name)?;
    debug!("Gateway type: {:?}", gateway_type);
    let gateway_direction_str = extract_optional_attribute(element, &QName(b"gatewayDirection"))?;
    debug!("Gateway direction: {:?}", gateway_direction_str);
    let gateway_direction = match gateway_direction_str {
        Some(s) => GatewayDirection::from_str(&s)?,
        None => GatewayDirection::Unspecified,
    };

    let gateway: Box<dyn FlowObjectBehavior> = match gateway_type {
        // ComplexGateway with multiple activation conditions
        GatewayType::ComplexGateway => {
            debug!("Parsing ComplexGateway");
            let mut activation_conditions = Vec::new();
            let mut buf = Vec::new();

            // Read the child elements to extract activationConditions
            loop {
                match xml_reader.read_event_into(&mut buf) {
                    Ok(XmlEvent::Start(ref e)) if e.name().as_ref() == b"activationCondition" => {
                        if let Ok(XmlEvent::Text(text)) = xml_reader.read_event_into(&mut buf) {
                            activation_conditions.push(text.unescape()?.to_string());
                        }
                    }
                    Ok(XmlEvent::End(ref e)) if e.name() == element.name() => {
                        break;
                    }
                    Ok(XmlEvent::Eof) => {
                        println!("Unexpected EOF");
                        return Err(BPMNParseError::XmlParseError("Unexpected EOF".to_string()));
                    }
                    Err(e) => {
                        return Err(BPMNParseError::XmlParseError(format!("Error: {:?}", e)));
                    }
                    _ => (),
                }
                buf.clear();
            }

            Box::new(ComplexGateway::new(
                id.clone(),
                &name,
                gateway_direction,
                activation_conditions,
            ))
        }
        // ExclusiveGateway with optional default flow
        GatewayType::ExclusiveGateway => {
            debug!("Parsing ExclusiveGateway");
            let default_flow = extract_optional_attribute(element, &QName(b"default"))?;
            Box::new(ExclusiveGateway::new(
                id.clone(),
                &name,
                gateway_direction,
                default_flow,
            ))
        }
        // EventBasedGateway with specific attributes
        GatewayType::EventBasedGateway => {
            debug!("Parsing EventBasedGateway");
            let event_gateway_type_str = extract_attribute(element, &QName(b"eventGatewayType"))?;
            let event_gateway_type = EventBasedGatewayType::from_str(&event_gateway_type_str)
                .map_err(|_| BPMNParseError::XmlParseError("Invalid eventGatewayType".into()))?;
            let instantiate = extract_bool_attribute(element, &QName(b"instantiate"))?;
            Box::new(EventBasedGateway::new(
                id.clone(),
                &name,
                gateway_direction,
                event_gateway_type,
                instantiate,
            ))
        }
        // InclusiveGateway with optional default flow
        GatewayType::InclusiveGateway => {
            debug!("Parsing InclusiveGateway");
            let default_flow = extract_optional_attribute(element, &QName(b"default"))?;
            Box::new(InclusiveGateway::new(
                id.clone(),
                &name,
                gateway_direction,
                default_flow,
            ))
        }
        // ParallelGateway with optional default flow
        GatewayType::ParallelGateway => {
            debug!("Parsing ParallelGateway");
            Box::new(ParallelGateway::new(id.clone(), &name, gateway_direction))
        }
    };

    let flow_object = FlowObject {
        id: id.clone(),
        flow_object_type: FlowObjectType::Gateway,
        flow_object_behavior: gateway as Box<dyn FlowObjectBehavior>,
    };
    // Add the parsed gateway to the process
    process.add_flow_object(flow_object)?;

    Ok(())
}
