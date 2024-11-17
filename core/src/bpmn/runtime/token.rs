// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use crate::bpmn::model::flow_objects::FlowObjectId;

use std::fmt;
use uuid::Uuid;

use super::process_instance::ProcessInstanceId;

/// Represents a unique identifier for a token in the process execution.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TokenId {
    id: String,
    // An optional user-defined identifier to prepend to the token ID for debugging
    debug_identifier: Option<String>,
}

impl TokenId {
    /// Creates a new `TokenId` with an optional debug identifier.
    pub fn new(debug_identifier: Option<String>) -> Self {
        TokenId {
            id: Uuid::new_v4().to_string().replace('-', ""),
            debug_identifier,
        }
    }

    /// Creates a new `TokenId` with an optional debug identifier.
    #[cfg(test)]
    pub fn new_with_id(id: String, debug_identifier: Option<String>) -> Self {
        TokenId {
            id,
            debug_identifier,
        }
    }

    /// Returns the full string representation of the `TokenId`.
    pub fn to_string(&self) -> String {
        if let Some(ref debug_id) = self.debug_identifier {
            format!("{}-{}", debug_id, self.id)
        } else {
            self.id.clone()
        }
    }

    /// Returns the unique ID of the token.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the debug identifier if set.
    pub fn debug_identifier(&self) -> Option<&String> {
        self.debug_identifier.as_ref()
    }
}

// Implement `Display` for easy printing
impl fmt::Display for TokenId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Tokens represent the state of a process instance as it moves through the process.
/// They are used to track the progress of the process and manage the flow of execution.
/// Tokens are created when a process instance is started and are moved through the process
/// by the process engine.
/// Tokens can be in one of several states. They move from one flow object to another by passing
/// through sequence flows.
#[derive(Debug, Clone)]
pub struct Token {
    id: TokenId,
    process_instance_id: ProcessInstanceId,
    flow_object_id: FlowObjectId,
    state: TokenState,
    parent_token_id: Option<TokenId>,
    child_tokens: Vec<Token>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token(id={}, state={}, flow_object_id={}, process_instance_id={})",
            self.id, self.state, self.flow_object_id, self.process_instance_id
        )
    }
}

impl Token {
    /// Returns the unique ID of the token.
    pub fn id(&self) -> &TokenId {
        &self.id
    }

    /// Returns the ID of the process instance the token is part of.
    pub fn process_instance_id(&self) -> &ProcessInstanceId {
        &self.process_instance_id
    }

    /// Returns the ID of the current node the token is at.
    pub fn flow_object_id(&self) -> &FlowObjectId {
        &self.flow_object_id
    }

    /// Returns the current state of the token.
    pub fn state(&self) -> &TokenState {
        &self.state
    }

    /// Sets the state of the token.
    pub fn set_state(&mut self, state: TokenState) {
        self.state = state;
    }

    /// Returns an optional reference to the parent token ID.
    pub fn parent_token_id(&self) -> Option<&TokenId> {
        self.parent_token_id.as_ref()
    }

    /// Returns a reference to the child tokens.
    pub fn child_tokens(&self) -> &Vec<Token> {
        &self.child_tokens
    }

    /// Creates a new builder for constructing a Token.
    pub fn builder() -> TokenBuilder {
        TokenBuilder::default()
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum TokenBuilderError {
    #[error("Token ID is required")]
    MissingTokenId,
    #[error("Process instance ID is required")]
    MissingProcessInstanceId,
    #[error("Flow object ID is required")]
    MissingFlowObjectId,
}

/// Builder for `Token`.
#[derive(Clone, Debug, Default)]
pub struct TokenBuilder {
    id: Option<TokenId>,
    process_instance_id: Option<ProcessInstanceId>,
    flow_object_id: Option<FlowObjectId>,
    state: Option<TokenState>,
    parent_token_id: Option<TokenId>,
    child_tokens: Vec<Token>,
}

impl TokenBuilder {
    /// Sets the unique ID of the token.
    pub fn id(mut self, id: TokenId) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the ID of the process instance the token is part of.
    pub fn process_instance_id(mut self, process_instance_id: ProcessInstanceId) -> Self {
        self.process_instance_id = Some(process_instance_id);
        self
    }

    /// Sets the ID of the current node the token is at.
    pub fn flow_object_id(mut self, flow_object_id: &str) -> Self {
        self.flow_object_id = Some(flow_object_id.into());
        self
    }

    /// Sets the current state of the token.
    pub fn state(mut self, state: TokenState) -> Self {
        self.state = Some(state);
        self
    }

    /// Sets an optional reference to the parent token ID.
    pub fn parent_token_id(mut self, parent_token_id: TokenId) -> Self {
        self.parent_token_id = Some(parent_token_id);
        self
    }

    /// Sets the child tokens of this token.
    pub fn child_tokens(mut self, child_tokens: Vec<Token>) -> Self {
        self.child_tokens = child_tokens;
        self
    }

    /// Builds the Token with the provided values.
    pub fn build(self) -> Result<Token, TokenBuilderError> {
        Ok(Token {
            id: self.id.ok_or(TokenBuilderError::MissingTokenId)?,
            process_instance_id: self
                .process_instance_id
                .ok_or(TokenBuilderError::MissingProcessInstanceId)?,
            flow_object_id: self
                .flow_object_id
                .ok_or(TokenBuilderError::MissingFlowObjectId)?,
            state: self.state.unwrap_or(TokenState::Active),
            parent_token_id: self.parent_token_id,
            child_tokens: self.child_tokens,
        })
    }
}

#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum TokenState {
    /// The state in which a flow object is prepared to start execution but hasn't yet begun.
    /// E.g., a token sitting at the start event waiting to be triggered.
    Ready,
    /// The state where a flow object (like a task) is actively executing.
    Active,
    /// An optional state where a flow object is waiting for an external trigger,
    /// input, or condition to proceed.
    Paused,
    /// The state indicating the flow object has finished executing.
    Completed,
    /// Commonly used for tokens rather than flow objects, this state can imply
    /// that a token has reached the end of a process flow path or been merged by a gateway, effectively ending its journey.
    Consumed,
    /// The state where a flow object has ended prematurely,
    /// often due to process cancellation or an interrupting event.
    Terminated,
}
