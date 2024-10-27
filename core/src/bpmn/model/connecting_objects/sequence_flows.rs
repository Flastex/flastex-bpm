use std::{any::Any, fmt::Debug};

use strum::EnumString;

use crate::bpmn::model::{flow_objects::FlowObjectId, script::Script};

/// Alias for Sequence Flow IDs used in BPMN elements.
pub type SequenceFlowId = String;

/// Enum to represent the "isImmediate" attribute.
///
/// - Immediate Triggering: If isImmediate is set to true, the sequence flow is triggered
/// immediately when the source activity is completed. This is typically used in
/// scenarios where there is a strict requirement that the process must transition to
/// the next step without any delay
/// - Non-Immediate Flow: If isImmediate is false or omitted, the flow can be delayed,
/// potentially allowing other conditions or events to occur before the process moves
/// to the next activity.
///
/// This attribute is more relevant when dealing with gateways or conditional flows,
/// where you might want to ensure that certain paths are executed as soon as a condition is met,
/// without waiting for other evaluations.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum ImmediateFlag {
    Immediate,
    NonImmediate,
}

impl Default for ImmediateFlag {
    fn default() -> Self {
        ImmediateFlag::NonImmediate
    }
}

/// Represents a BPMN sequence flow.
#[derive(Clone, Debug)]
pub struct SequenceFlow {
    id: SequenceFlowId,
    source_ref: FlowObjectId,
    target_ref: FlowObjectId,
    is_immediate: ImmediateFlag,
    sequence_flow_behavior: Box<dyn SequenceFlowBehavior>,
}

impl SequenceFlow {
    /// Creates a new sequence flow with default values.
    pub fn new() -> Self {
        SequenceFlow {
            id: FlowObjectId::default(),
            source_ref: FlowObjectId::default(),
            target_ref: FlowObjectId::default(),
            is_immediate: ImmediateFlag::default(),
            sequence_flow_behavior: Box::new(NormalFlow),
        }
    }

    /// Returns the ID of the sequence flow.
    pub fn id(&self) -> &SequenceFlowId {
        &self.id
    }

    /// Sets the ID of the sequence flow.
    pub fn set_id(&mut self, id: FlowObjectId) -> &mut Self {
        self.id = id;
        self
    }

    ///  Returns the source reference ID.
    pub fn source_ref(&self) -> &FlowObjectId {
        &self.source_ref
    }

    /// Sets the source reference ID.
    pub fn set_source_ref(&mut self, source_ref: FlowObjectId) -> &mut Self {
        self.source_ref = source_ref;
        self
    }

    /// Returns the target reference ID.
    pub fn target_ref(&self) -> &FlowObjectId {
        &self.target_ref
    }

    /// Sets the target reference ID.
    pub fn set_target_ref(&mut self, target_ref: FlowObjectId) -> &mut Self {
        self.target_ref = target_ref;
        self
    }

    /// Returns the "isImmediate" flag.
    pub fn is_immediate(&self) -> ImmediateFlag {
        self.is_immediate.clone()
    }

    /// Sets the "isImmediate" flag.
    pub fn set_is_immediate(&mut self, is_immediate: ImmediateFlag) -> &mut Self {
        self.is_immediate = is_immediate;
        self
    }

    /// Returns the sequence flow behavior.
    pub fn sequence_flow_behavior(&self) -> Box<&dyn SequenceFlowBehavior> {
        Box::new(self.sequence_flow_behavior.as_ref())
    }

    /// Sets the sequence flow behavior.
    /// See specific methods for setting the behavior.
    /// - `as_default()`
    /// - `as_normal()`
    /// - `as_conditional()`
    pub fn set_behavior(&mut self, behavior: Box<dyn SequenceFlowBehavior>) -> &mut Self {
        self.sequence_flow_behavior = behavior;
        self
    }

    /// Sets the behavior to `NormalFlow`.
    pub fn as_normal(&mut self) -> &mut Self {
        self.sequence_flow_behavior = Box::new(NormalFlow);
        self
    }

    /// Sets the behavior to `ConditionalFlow` with the specified condition expression.
    pub fn as_conditional(&mut self, condition: Option<Script>) -> &mut Self {
        self.sequence_flow_behavior = Box::new(ConditionalFlow::new(condition));
        self
    }
}

/// Enum representing the type of sequence flow.
#[derive(Clone, EnumString, PartialEq, Debug)]
pub enum SequenceFlowType {
    Normal,
    Conditional,
}

/// Trait for defining the behavior of sequence flows.
pub trait SequenceFlowBehavior: Any + Debug + SequenceFlowCloneBoxed {
    fn r#type(&self) -> SequenceFlowType;
    fn as_any(&self) -> &dyn Any;
}

pub trait SequenceFlowCloneBoxed {
    fn clone_box(&self) -> Box<dyn SequenceFlowBehavior>;
}

impl<T> SequenceFlowCloneBoxed for T
where
    T: 'static + SequenceFlowBehavior + Clone,
{
    fn clone_box(&self) -> Box<dyn SequenceFlowBehavior> {
        Box::new(self.clone())
    }
}

/// Clone implementation for `Box<dyn SequenceFlowBehavior>`.
impl Clone for Box<dyn SequenceFlowBehavior> {
    fn clone(&self) -> Box<dyn SequenceFlowBehavior> {
        self.clone_box()
    }
}

// Define a default Sequence Flow implementation.
#[derive(Clone, Debug)]
pub struct NormalFlow;

impl SequenceFlowBehavior for NormalFlow {
    fn r#type(&self) -> SequenceFlowType {
        SequenceFlowType::Normal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Conditional Flow implementation with a condition expression.
#[derive(Clone, Debug)]
pub struct ConditionalFlow {
    condition: Option<Script>,
}

impl ConditionalFlow {
    pub fn new(condition: Option<Script>) -> Self {
        ConditionalFlow { condition }
    }

    pub fn condition_expression(&self) -> Option<&Script> {
        self.condition.as_ref()
    }
}

impl SequenceFlowBehavior for ConditionalFlow {
    fn r#type(&self) -> SequenceFlowType {
        SequenceFlowType::Conditional
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
