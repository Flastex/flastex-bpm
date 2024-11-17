// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

use crate::bpmn::model::{flow_objects::FlowObjectId, script::Script};

/// Alias for Sequence Flow IDs used in BPMN elements.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SequenceFlowId(String);

impl SequenceFlowId {
    pub fn new(id: &str) -> Self {
        SequenceFlowId(id.to_string())
    }
}

impl fmt::Display for SequenceFlowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SequenceFlowId {
    fn from(id: String) -> Self {
        SequenceFlowId(id)
    }
}

impl From<&str> for SequenceFlowId {
    fn from(id: &str) -> Self {
        SequenceFlowId(id.to_string())
    }
}

impl std::ops::Deref for SequenceFlowId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<str> for SequenceFlowId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for SequenceFlowId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

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

/// Enum representing the type and behavior of sequence flow.
#[derive(Clone, Debug)]
pub enum SequenceFlowType {
    /// Represents a normal flow without any conditions.
    Normal(NormalSequenceFlow),
    /// Represents a conditional flow with an optional script condition.
    Conditional(ConditionalSequenceFlow),
}

/// Represents a BPMN sequence flow.
#[derive(Clone, Debug)]
pub struct SequenceFlow {
    id: SequenceFlowId,
    source_ref: FlowObjectId,
    target_ref: FlowObjectId,
    is_immediate: ImmediateFlag,
    sequence_flow_type: SequenceFlowType,
}

impl SequenceFlow {
    /// Accessor for the sequence flow ID.
    pub fn id(&self) -> &SequenceFlowId {
        &self.id
    }

    /// Accessor for the source reference ID.
    pub fn source_ref(&self) -> &FlowObjectId {
        &self.source_ref
    }

    /// Accessor for the target reference ID.
    pub fn target_ref(&self) -> &FlowObjectId {
        &self.target_ref
    }

    /// Accessor for the "isImmediate" flag.
    pub fn is_immediate(&self) -> &ImmediateFlag {
        &self.is_immediate
    }

    /// Accessor for the flow type.
    pub fn flow_type(&self) -> &SequenceFlowType {
        &self.sequence_flow_type
    }

    /// Creates a new builder for `SequenceFlow`.
    pub fn builder() -> SequenceFlowBuilder {
        SequenceFlowBuilder::new()
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum SequenceFlowBuilderError {
    #[error("SequenceFlow ID must be set")]
    MissingId,
    #[error("Source reference must be set")]
    MissingSourceRef,
    #[error("Target reference must be set")]
    MissingTargetRef,
    #[error("Sequence flow type must be set")]
    MissingSequenceFlowType,
}

/// Builder for `SequenceFlow`.
#[derive(Clone, Debug, Default)]
pub struct SequenceFlowBuilder {
    id: Option<SequenceFlowId>,
    source_ref: Option<FlowObjectId>,
    target_ref: Option<FlowObjectId>,
    is_immediate: ImmediateFlag,
    sequence_flow_type: Option<SequenceFlowType>,
}

impl SequenceFlowBuilder {
    pub fn new() -> Self {
        SequenceFlowBuilder {
            id: None,
            source_ref: None,
            target_ref: None,
            is_immediate: ImmediateFlag::default(),
            sequence_flow_type: None,
        }
    }

    pub fn id(&mut self, id: SequenceFlowId) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn source_ref(&mut self, source_ref: FlowObjectId) -> &mut Self {
        self.source_ref = Some(source_ref);
        self
    }

    pub fn target_ref(&mut self, target_ref: FlowObjectId) -> &mut Self {
        self.target_ref = Some(target_ref);
        self
    }

    pub fn is_immediate(&mut self, is_immediate: ImmediateFlag) -> &mut Self {
        self.is_immediate = is_immediate;
        self
    }

    pub fn normal(&mut self) -> &mut Self {
        self.sequence_flow_type = Some(SequenceFlowType::Normal(NormalSequenceFlow {}));
        self
    }

    pub fn conditional(&mut self, condition: Option<Script>) -> &mut Self {
        self.sequence_flow_type = Some(SequenceFlowType::Conditional(ConditionalSequenceFlow {
            condition,
        }));
        self
    }

    /// Finalizes the builder, returning a `SequenceFlow`.
    pub fn build(self) -> Result<SequenceFlow, SequenceFlowBuilderError> {
        Ok(SequenceFlow {
            id: self.id.ok_or(SequenceFlowBuilderError::MissingId)?,
            source_ref: self
                .source_ref
                .ok_or(SequenceFlowBuilderError::MissingSourceRef)?,
            target_ref: self
                .target_ref
                .ok_or(SequenceFlowBuilderError::MissingTargetRef)?,
            is_immediate: self.is_immediate,
            sequence_flow_type: self
                .sequence_flow_type
                .ok_or(SequenceFlowBuilderError::MissingSequenceFlowType)?,
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct NormalSequenceFlow;

#[derive(Clone, PartialEq, Debug)]
pub struct ConditionalSequenceFlow {
    condition: Option<Script>,
}

impl ConditionalSequenceFlow {
    pub fn condition(&self) -> Option<&Script> {
        self.condition.as_ref()
    }
}
