// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::any::Any;

use super::FlowObjectBehavior;

#[derive(Clone, strum::Display, strum::AsRefStr, strum::EnumDiscriminants, PartialEq, Debug)]
#[strum_discriminants(derive(strum::EnumString), name(Type), strum(ascii_case_insensitive))]
pub enum EventType {
    StartEvent(Event),
    EndEvent(Event),
    IntermediateEvent(Event),
}

impl Type {
    pub fn to_event_type(&self, event: Event) -> EventType {
        match self {
            Type::StartEvent => EventType::StartEvent(event),
            Type::EndEvent => EventType::EndEvent(event),
            Type::IntermediateEvent => EventType::IntermediateEvent(event),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Event {
    pub name: String,
     event_type: Type,
}

impl Event {
    pub fn new(name: &str, event_type: Type) -> Self {
        Event {
            name: name.to_string(),
            event_type: event_type,
        }
    }

    pub fn event_type(&self) -> &Type {
        &self.event_type
    }
}

impl FlowObjectBehavior for Event {
    fn flow_object_type(&self) -> super::Type {
        super::Type::Event
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
