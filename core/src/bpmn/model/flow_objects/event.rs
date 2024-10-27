use std::any::Any;

use super::{FlowObjectBehavior, FlowObjectId};

#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
#[strum(ascii_case_insensitive)]
pub enum EventType {
    StartEvent,
    EndEvent,
    IntermediateEvent,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Event {
    pub id: FlowObjectId,
    pub name: String,
    pub event_type: EventType,
}

impl Event {
    pub fn new(id: &FlowObjectId, name: &str, event_type: &EventType) -> Self {
        Event {
            id: id.clone(),
            name: name.to_string(),
            event_type: event_type.clone(),
        }
    }
}

impl FlowObjectBehavior for Event {
    fn r#type(&self) -> super::FlowObjectType {
        super::FlowObjectType::Event
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
