/// Flow Objects are the main graphical elements to define the behavior
/// of a Business Process. There are three Flow Objects:
/// 1. Events
/// 2. Activities
/// 3. Gateways
use event::{Event, EventType};
use std::any::Any;
use std::fmt::Debug;
use strum::EnumString;

pub mod activity;
pub mod event;
pub mod gateway;
pub mod task;

/// Alias for Flow Object IDs used in BPMN elements.
pub type FlowObjectId = String;

#[derive(Clone, Debug)]
pub struct FlowObject {
    pub id: FlowObjectId,
    pub flow_object_type: FlowObjectType,
    pub flow_object_behavior: Box<dyn FlowObjectBehavior>,
}

#[derive(Clone, EnumString, Debug, PartialEq)]
pub enum FlowObjectType {
    #[strum(ascii_case_insensitive)]
    Task,
    #[strum(ascii_case_insensitive)]
    Event,
    #[strum(ascii_case_insensitive)]
    Gateway,
}

pub trait FlowObjectBehavior: Any + Debug + FlowObjectCloneBoxed {
    fn r#type(&self) -> FlowObjectType;

    fn as_any(&self) -> &dyn Any;
}

pub trait FlowObjectCloneBoxed {
    fn clone_box(&self) -> Box<dyn FlowObjectBehavior>;
}

impl<T> FlowObjectCloneBoxed for T
where
    T: 'static + FlowObjectBehavior + Clone,
{
    fn clone_box(&self) -> Box<dyn FlowObjectBehavior> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn FlowObjectBehavior> {
    fn clone(&self) -> Box<dyn FlowObjectBehavior> {
        self.clone_box()
    }
}

pub fn flowobject_is_start_event(flow_object: &FlowObject) -> bool {
    if flow_object.flow_object_type == FlowObjectType::Event {
        let event = flow_object
            .flow_object_behavior
            .as_any()
            .downcast_ref::<Event>()
            .expect("Failed to downcast FlowObjectBehavior to Event");
        return event.event_type == EventType::StartEvent;
    }
    false
}
