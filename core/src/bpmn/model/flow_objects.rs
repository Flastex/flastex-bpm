// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use activity::ActivityType;
/// Flow Objects are the main graphical elements to define the behavior
/// of a Business Process. There are three Flow Objects:
/// 1. Events
/// 2. Activities
/// 3. Gateways
use event::EventType;
use gateway::GatewayType;
use std::any::Any;
use std::fmt::Debug;

pub mod activity;
pub mod event;
pub mod gateway;
pub mod task;

/// Alias for Flow Object IDs used in BPMN elements.
pub type FlowObjectId = String;

#[derive(Clone, Debug, PartialEq)]
pub struct FlowObject {
    pub id: FlowObjectId,
    pub flow_object_type: FlowObjectType,
}

#[derive(Clone, strum::Display, strum::AsRefStr, strum::EnumDiscriminants, PartialEq, Debug)]
#[strum_discriminants(derive(strum::EnumString), name(Type))]
pub enum FlowObjectType {
    Activity(ActivityType),
    Event(EventType),
    Gateway(GatewayType),
}

impl FlowObjectType {
    pub fn type_name(&self) -> &str {
        match self {
            FlowObjectType::Activity(activity_type) => activity_type.as_ref(),
            FlowObjectType::Event(event_type) => event_type.as_ref(),
            FlowObjectType::Gateway(gateway_type) => gateway_type.as_ref(),
        }
    }
}

pub trait FlowObjectBehavior: Any + Debug + PartialEq {
    fn flow_object_type(&self) -> Type;
    fn as_any(&self) -> &dyn Any;
}

pub fn flowobject_is_start_event(flow_object: &FlowObject) -> bool { 
    if let FlowObjectType::Event(event_type) = &flow_object.flow_object_type {
         if let EventType::StartEvent(_) = event_type {
            return true;
         }
    }
    false
}

#[cfg(test)]
mod tests {
    use event::Event;

    use super::*;

    #[test]
    fn test_flowobject_is_start_event() {
        let flow_object = FlowObject {
            id: "1".to_string(),
            flow_object_type: FlowObjectType::Event(EventType::StartEvent(Event::new("Start Event", event::Type::StartEvent))),
        };
        assert_eq!(flowobject_is_start_event(&flow_object), true);
    }

    #[test]
    fn test_flowobject_is_not_start_event() {
        let flow_object = FlowObject {
            id: "1".to_string(),
            flow_object_type: FlowObjectType::Event(EventType::IntermediateEvent(Event::new("Intermediate Event", event::Type::IntermediateEvent))),
        };
        assert_eq!(flowobject_is_start_event(&flow_object), false);
    }
}
