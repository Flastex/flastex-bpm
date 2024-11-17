// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::{collections::HashMap, fmt};

use log::debug;
use uuid::Uuid;

use crate::bpmn::{
    model::{
        flow_objects::{
            event::{self},
            FlowObjectType,
        },
        process::{Process, ProcessId},
    },
    runtime::behaviors::event_behaviors::StartEventBehavior,
};

use super::{
    behaviors::flow_object_behavior::FlowObjectInstance,
    scope::Scope,
    token::{Token, TokenBuilderError, TokenId, TokenState},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ProcessInstanceId(String);

impl fmt::Display for ProcessInstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProcessInstanceId({})", self.0)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, strum::EnumString, strum::Display)]
pub enum ProcessInstanceState {
    /// The process instance is initialized but not yet started.
    Initialized,
    /// The process instance is currently running.
    Active,
    /// The process instance is suspended, waiting for an external trigger.
    Suspended,
    /// The process instance is completed successfully.
    Completed,
    /// The process instance was terminated before completion.
    Terminated,
    /// The process instance encountered an error and cannot continue.
    Error,
    /// The process instance is currently compensating due to an error or cancelation.
    Compensating,
    /// The process instance has been fully compensated.
    Compensated,
    /// The process instance was canceled and cannot proceed.
    Canceled,
}

#[derive(Clone, Debug)]
pub struct ProcessInstance {
    // Identifier of the process instance
    id: ProcessInstanceId,
    // Identifier of the process
    process_id: ProcessId,
    // The current state of the process instance
    pub(super) state: ProcessInstanceState,
    // Holds the process root scope
    process_scope: Scope,
    // Active tokens in the process
    pub(super) tokens: Vec<Token>,
    // Active flow object instances
    pub(super) flow_object_instances: HashMap<TokenId, FlowObjectInstance>,
    // Reference to the static BPMN model
    process: Process,
}

impl ProcessInstance {
    pub fn new(process: Process) -> Self {
        ProcessInstance {
            id: ProcessInstanceId(Uuid::new_v4().to_string()),
            process_id: process.id().clone(),
            state: ProcessInstanceState::Initialized,
            // FOR NOW, NO ITEMS TO BE INITIALIZED
            process_scope: Scope::new("root".to_string(), HashMap::new(), Vec::new()),
            tokens: Vec::new(),
            flow_object_instances: HashMap::new(),
            process,
        }
    }

    /// Returns the unique ID of the process instance.
    pub fn id(&self) -> &ProcessInstanceId {
        &self.id
    }

    /// Returns the unique ID of the process.
    pub fn process_id(&self) -> &ProcessId {
        &self.process_id
    }

    /// Returns the current state of the process instance.
    pub fn state(&self) -> &ProcessInstanceState {
        &self.state
    }

    /// Returns the process scope.
    pub fn process_scope(&self) -> &Scope {
        &self.process_scope
    }

    pub fn process_scope_mut(&mut self) -> &mut Scope {
        &mut self.process_scope
    }

    /// Adds a new token to the process
    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    /// Gets all tokens in the process with a given state
    pub fn get_tokens(&self, token_state: TokenState) -> Vec<&Token> {
        self.tokens
            .iter()
            .filter(|t| *t.state() == token_state)
            .collect()
    }

    /// Gets all active tokens
    pub fn get_ready_tokens(&self) -> Vec<&Token> {
        self.get_tokens(TokenState::Ready)
    }

    /// Gets all active tokens
    pub fn get_active_tokens(&self) -> Vec<&Token> {
        self.get_tokens(TokenState::Active)
    }

    pub fn add_flow_object_instance(
        &mut self,
        token_id: &TokenId,
        flow_object_instance: FlowObjectInstance,
    ) {
        self.flow_object_instances
            .insert(token_id.clone(), flow_object_instance);
    }

    pub fn flow_object_instance(&self, token_id: &TokenId) -> Option<&FlowObjectInstance> {
        self.flow_object_instances.get(token_id)
    }

    pub fn process(&self) -> &Process {
        &self.process
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ProcessInstanceFactoryError {
    #[error("Token ID is required")]
    TokenBuilderError(#[from] TokenBuilderError),
}

pub struct ProcessInstanceFactory;

impl ProcessInstanceFactory {
    /// Creates a new `ProcessInstanceFactory`.
    fn new() -> Self {
        ProcessInstanceFactory
    }

    /// Creates a new `ProcessInstance` from a given `Process`.
    /// Initializes tokens and scopes as required.
    pub fn create_instance(
        process: &Process,
    ) -> Result<ProcessInstance, ProcessInstanceFactoryError> {
        let mut process_instance = ProcessInstance::new(process.clone());
        for (id, flow_object) in process.flow_objects() {
            match flow_object.flow_object_type() {
                FlowObjectType::Event(ref event_type)
                    if event_type.to_type() == event::Type::StartEvent =>
                {
                    debug!(
                        "Creating token for {:?} with id: {:?}",
                        event::Type::StartEvent,
                        id
                    );
                    // Start events are ready to be executed
                    let token_id = TokenId::new(None);
                    let start_event =
                        FlowObjectInstance::StartEvent(StartEventBehavior { id: id.clone() });
                    process_instance.add_flow_object_instance(&token_id, start_event);
                    let start_token = Token::builder()
                        .id(token_id)
                        .process_instance_id(process_instance.id.clone())
                        .flow_object_id(flow_object.id())
                        .state(TokenState::Ready)
                        .build()
                        .map_err(|e| ProcessInstanceFactoryError::from(e))?;
                    process_instance.add_token(start_token);
                }
                _ => (),
            };
        }

        Ok(process_instance)
    }
}
