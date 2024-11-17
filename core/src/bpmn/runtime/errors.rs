// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::fmt;

use thiserror::Error;

use crate::bpmn::model::{flow_objects::FlowObjectId, process::ProcessId};

use super::{
    behaviors::flow_object_behavior::FlowObjectBehaviorError,
    process_instance::ProcessInstanceId,
    token::{TokenId, TokenState},
};

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptExecutionError {
    pub message: String,
}

impl fmt::Display for ScriptExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Script execution error: {}", self.message)
    }
}

// Define the main error for BPMN parsing
#[derive(Error, Debug)]
pub enum ProcessEngineError {
    // API-related errors
    #[error("Process {0} not found.")]
    ProcessNotFound(ProcessId),
    #[error("Process instance retrieval error: {0}.")]
    ProcessInstanceNotFound(ProcessInstanceId),
    #[error("Process instance state transition error: {0}.")]
    InvalidStateTransition(String),

    // Process Instance Creation errors
    #[error("Process instance creation failed: {0}.")]
    ProcessInstanceCreationError(String),

    // Process Model errors
    #[error("Flow-Object not found in Process error: {0}.")]
    FlowObjectNotFound(FlowObjectId),

    // Process Instance errors
    #[error("Flow-Object Instance not found for token: {0}.")]
    FlowObjectInstanceNotFound(TokenId),
    #[error("Flow-Object Behavior error: {0}.")]
    FlowObjectBehaviorError(#[from] FlowObjectBehaviorError),

    // Script-errors
    #[error("Script execution error: {0}.")]
    ScriptExecution(ScriptExecutionError),

    // Flow-related errors
    #[error("Process engine error: Missing start event in BPMN model.")]
    MissingStartEvent,
    #[error("Process engine error: Invalid transition - {0}")]
    InvalidTransition(String),
    #[error("Process engine error: Deadlock detected - {0}")]
    Deadlock(String),

    // Token management errors
    #[error("Token management error: Token not found - {0}")]
    TokenNotFound(TokenId),
    #[error("Token management error: Invalid token state - {0}")]
    InvalidTokenState(TokenState),

    // Validation errors
    #[error("Validation error: Invalid process structure - {0}")]
    InvalidProcessStructure(ProcessId),
    #[error("Validation error: Unsupported BPMN element - {element:?}")]
    UnsupportedElement { element: String },
}
