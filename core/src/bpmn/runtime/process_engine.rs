// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use crate::bpmn::model::process::ProcessId;
use crate::bpmn::runtime::process_instance::ProcessInstance;
use crate::bpmn::runtime::token::{Token, TokenState};
use std::collections::{HashMap, VecDeque};

use super::behaviors::flow_object_behavior::{FlowObjectInstance, Initializable};
use super::errors::ProcessEngineError;
use super::path_identifier::{PathElement, PathIdentifier};
use super::process_instance::{ProcessInstanceFactory, ProcessInstanceId, ProcessInstanceState};
use super::token::TokenId;

use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use crate::bpmn::model::process::Process;

// === Detail structs ===============================================================================

#[derive(Debug, Clone)]
pub struct TokenDetails {
    /// The token ID within the process instance.
    pub token_id: TokenId,

    /// The path identifier for the token.
    pub path_identifier: PathIdentifier,

    /// The current state of the token.
    pub state: TokenState,

    /// Parent token ID if this token was spawned as part of a subprocess or parallel flow.
    pub parent_token_id: Option<TokenId>,

    /// Active tokens within the process, each representing a point of progress.
    /// Child tokens may also sit on subprocesses or parallel flows.
    pub child_tokens: Vec<TokenDetails>,
}

#[derive(Debug, Clone)]
pub struct ProcessInstanceDetails {
    /// The unique identifier for this process instance.
    pub instance_id: ProcessInstanceId,

    /// The current state of the process instance (e.g., Active, Suspended, Terminated).
    pub state: ProcessInstanceState,

    /// Active root tokens within the process, each representing a point of progress.
    pub root_tokens: Vec<TokenDetails>,

    // Next section should be replaced by a stats struct instance
    /// Creation time of the process instance.
    pub creation_time: std::time::SystemTime,

    /// Start time of the process instance for tracking duration.
    pub start_time: Option<std::time::SystemTime>,

    /// Last update time of the process instance, for tracking activity.
    pub last_update_time: std::time::SystemTime,

    /// End time of the process instance, if completed.
    pub end_time: Option<std::time::SystemTime>,
}

// === ProcessEngine Trait ==========================================================================

/// Main API for interacting with the BPMN Process Engine
/// We may let it extend `Send` and `Sync` in the future and turn methods into async
pub trait ProcessEngine {
    /// Creates a new process instance from a process definition.
    /// Returns the created process instance ID.
    fn create_instance(
        &mut self,
        process_id: &ProcessId,
    ) -> Result<ProcessInstanceId, ProcessEngineError>;

    /// Starts the execution of a process instance, transitioning it to an active state.
    fn start_instance(&mut self, instance_id: &ProcessInstanceId)
        -> Result<(), ProcessEngineError>;

    /// Suspends the execution of a process instance.
    /// Useful for pausing long-running processes that wait for external triggers.
    fn suspend_instance(
        &mut self,
        instance_id: &ProcessInstanceId,
    ) -> Result<(), ProcessEngineError>;

    /// Resumes a suspended process instance, reactivating it.
    fn resume_instance(
        &mut self,
        instance_id: &ProcessInstanceId,
    ) -> Result<(), ProcessEngineError>;

    /// Terminates a process instance, marking it as completed or terminated.
    fn terminate_instance(
        &mut self,
        instance_id: &ProcessInstanceId,
    ) -> Result<(), ProcessEngineError>;

    /// Cancels a process instance, initiating compensation tasks if necessary.
    fn cancel_instance(
        &mut self,
        instance_id: &ProcessInstanceId,
    ) -> Result<(), ProcessEngineError>;

    /// Triggers a specific event within a process instance, such as starting a task.
    fn trigger_event(
        &mut self,
        instance_id: &ProcessInstanceId,
        event_id: &str,
    ) -> Result<(), ProcessEngineError>;

    /// Fetches the current state of a process instance.
    fn get_instance_state(
        &self,
        instance_id: &ProcessInstanceId,
    ) -> Result<ProcessInstanceState, ProcessEngineError>;

    /// Retrieves details of a process instance, including active tokens and flow states.
    fn get_instance_details(
        &self,
        instance_id: &ProcessInstanceId,
    ) -> Result<ProcessInstanceDetails, ProcessEngineError>;

    /// Maps all active process instances by their unique IDs.
    fn list_process_instance_data_by_state(
        &self,
        states: &[ProcessInstanceState],
    ) -> Result<HashMap<ProcessInstanceState, Vec<ProcessInstanceId>>, ProcessEngineError>;
}

// === ProcessEngine Factory ========================================================================

pub struct ProcessEngineFactory;

impl ProcessEngineFactory {
    pub fn new() -> Self {
        ProcessEngineFactory
    }

    pub fn create_process_engine(
        &self,
        process_definitions: Arc<Mutex<HashMap<ProcessId, Process>>>,
    ) -> impl ProcessEngine {
        InMemoryProcessEngine::new(process_definitions)
    }
}

// === Inner Implementation Process-related structs ================================================

#[derive(Clone, Debug)]
struct ProcessInstanceData {
    process_instance: ProcessInstance,
    path_identifiers: HashMap<TokenId, PathIdentifier>,
    process_instance_details: ProcessInstanceDetails,
}

#[derive(Debug)]
struct InMemoryProcessEngine {
    process_instance_data_by_id: Arc<Mutex<HashMap<ProcessInstanceId, ProcessInstanceData>>>,
    // TODO: Replace with a proper process definition storage
    process_definitions: Arc<Mutex<HashMap<ProcessId, Process>>>,
}

impl InMemoryProcessEngine {
    pub fn new(process_definitions: Arc<Mutex<HashMap<ProcessId, Process>>>) -> Self {
        InMemoryProcessEngine {
            process_instance_data_by_id: Arc::new(Mutex::new(HashMap::new())),
            process_definitions,
        }
    }
}

impl ProcessEngine for InMemoryProcessEngine {
    fn create_instance(
        &mut self,
        process_id: &ProcessId,
    ) -> Result<ProcessInstanceId, ProcessEngineError> {
        let process = {
            let processes = self.process_definitions.lock().unwrap();
            processes
                .get(process_id)
                .cloned()
                .ok_or_else(|| ProcessEngineError::ProcessNotFound(process_id.clone()))?
        };
        let process_instance = ProcessInstanceFactory::create_instance(&process)
            .map_err(|e| ProcessEngineError::ProcessInstanceCreationError(e.to_string()))?;
        let path_identifiers = self.define_path_identifiers(&process_instance);
        let creation_time = SystemTime::now();
        let process_instance_details = ProcessInstanceDetails {
            instance_id: process_instance.id().clone(),
            state: process_instance.state().clone(),
            root_tokens: process_instance
                .get_active_tokens()
                .iter()
                .map(|token| self.to_token_details(&token, &process_instance))
                .collect(),
            creation_time,
            start_time: None,
            last_update_time: creation_time,
            end_time: None,
        };

        let process_instance_id = process_instance.id().clone();
        let process_instance_data = ProcessInstanceData {
            process_instance,
            path_identifiers,
            process_instance_details,
        };

        self.process_instance_data_by_id
            .lock()
            .unwrap()
            .insert(process_instance_id.clone(), process_instance_data);
        Ok(process_instance_id)
    }

    fn start_instance(
        &mut self,
        instance_id: &ProcessInstanceId,
    ) -> Result<(), ProcessEngineError> {
        let _ = self.with_process_instance_data_mut(instance_id, |process_instance_data| {
            let process_instance = &mut process_instance_data.process_instance;
            process_instance.state = ProcessInstanceState::Active;
            let start_time = SystemTime::now();
            process_instance_data.process_instance_details.start_time = Some(start_time);

            process_instance_data
                .process_instance_details
                .last_update_time = start_time;
        });

        // Process all tokens with status Ready
        self.process_ready_tokens(instance_id)
    }

    fn suspend_instance(
        &mut self,
        instance_id: &ProcessInstanceId,
    ) -> Result<(), ProcessEngineError> {
        self.with_process_instance_data_mut(instance_id, |process_instance_data| {
            let process_instance = &mut process_instance_data.process_instance;
            process_instance.state = ProcessInstanceState::Suspended;
            process_instance_data
                .process_instance_details
                .last_update_time = SystemTime::now();
            Ok(())
        })?
    }

    fn resume_instance(
        &mut self,
        instance_id: &ProcessInstanceId,
    ) -> Result<(), ProcessEngineError> {
        self.with_process_instance_data_mut(instance_id, |process_instance_data| {
            let process_instance = &mut process_instance_data.process_instance;

            if process_instance.state != ProcessInstanceState::Suspended {
                return Err(ProcessEngineError::InvalidStateTransition(format!(
                    "Cannot resume process instance {instance_id}: it is not suspended."
                )));
            }

            process_instance.state = ProcessInstanceState::Active;
            process_instance_data
                .process_instance_details
                .last_update_time = SystemTime::now();
            Ok(())
        })?
    }

    fn terminate_instance(
        &mut self,
        instance_id: &ProcessInstanceId,
    ) -> Result<(), ProcessEngineError> {
        self.with_process_instance_data_mut(instance_id, |process_instance_data| {
            let process_instance = &mut process_instance_data.process_instance;

            process_instance.state = ProcessInstanceState::Terminated;
            let end_time = SystemTime::now();
            process_instance_data
                .process_instance_details
                .last_update_time = end_time;
            process_instance_data.process_instance_details.end_time = Some(end_time);
            Ok(())
        })?
    }

    fn cancel_instance(
        &mut self,
        instance_id: &ProcessInstanceId,
    ) -> Result<(), ProcessEngineError> {
        self.with_process_instance_data_mut(instance_id, |process_instance_data| {
            let process_instance = &mut process_instance_data.process_instance;

            process_instance.state = ProcessInstanceState::Canceled;
            let end_time = SystemTime::now();
            process_instance_data
                .process_instance_details
                .last_update_time = end_time;
            process_instance_data.process_instance_details.end_time = Some(end_time);
            Ok(())
        })?
    }

    fn trigger_event(
        &mut self,
        _instance_id: &ProcessInstanceId,
        _event_id: &str,
    ) -> Result<(), ProcessEngineError> {
        // TODO Specific logic for handling events and updating tokens

        Ok(())
    }

    fn get_instance_state(
        &self,
        instance_id: &ProcessInstanceId,
    ) -> Result<ProcessInstanceState, ProcessEngineError> {
        self.with_process_instance_data(instance_id, |process_instance_data| {
            Ok(process_instance_data.process_instance.state.clone())
        })?
    }

    fn get_instance_details(
        &self,
        instance_id: &ProcessInstanceId,
    ) -> Result<ProcessInstanceDetails, ProcessEngineError> {
        self.with_process_instance_data(instance_id, |process_instance_data| {
            let process_instance_details = &process_instance_data.process_instance_details;
            Ok(process_instance_details.clone())
        })?
    }

    fn list_process_instance_data_by_state(
        &self,
        states: &[ProcessInstanceState],
    ) -> Result<HashMap<ProcessInstanceState, Vec<ProcessInstanceId>>, ProcessEngineError> {
        let process_instances_by_id = self.process_instance_data_by_id.lock().unwrap();
        let mut instances_by_state: HashMap<ProcessInstanceState, Vec<ProcessInstanceId>> =
            HashMap::new();

        // Initialize the map with empty vectors for each requested state
        for state in states {
            instances_by_state.insert(state.clone(), Vec::new());
        }

        // Populate the map with active instance IDs by state
        for (id, process_instance_data) in process_instances_by_id.iter() {
            if states.contains(&process_instance_data.process_instance.state) {
                instances_by_state
                    .entry(process_instance_data.process_instance.state.clone())
                    .or_insert_with(Vec::new)
                    .push(id.clone());
            }
        }

        Ok(instances_by_state)
    }
}

impl InMemoryProcessEngine {
    /// Helper function to retrieve a mutable reference to a process instance by its ID
    fn with_process_instance_data_mut<F, R>(
        &mut self,
        instance_id: &ProcessInstanceId,
        f: F,
    ) -> Result<R, ProcessEngineError>
    where
        F: FnOnce(&mut ProcessInstanceData) -> R,
    {
        let mut process_instances_by_id = self.process_instance_data_by_id.lock().unwrap();
        let process_instance_data = process_instances_by_id.get_mut(instance_id).ok_or(
            ProcessEngineError::ProcessInstanceNotFound(instance_id.clone()),
        )?;

        Ok(f(process_instance_data))
    }

    fn with_process_instance_data<F, R>(
        &self,
        instance_id: &ProcessInstanceId,
        f: F,
    ) -> Result<R, ProcessEngineError>
    where
        F: FnOnce(&ProcessInstanceData) -> R,
    {
        let process_instances_by_id = self.process_instance_data_by_id.lock().unwrap();
        let process_instance_data = process_instances_by_id.get(instance_id).ok_or(
            ProcessEngineError::ProcessInstanceNotFound(instance_id.clone()),
        )?;

        Ok(f(process_instance_data))
    }

    fn define_path_identifiers(
        &self,
        process_instance: &ProcessInstance,
    ) -> HashMap<TokenId, PathIdentifier> {
        let mut path_identifiers: HashMap<TokenId, PathIdentifier> = HashMap::new();
        let mut tokens_to_process = VecDeque::new();

        // Initialize the deque with root tokens
        for token in process_instance.get_active_tokens() {
            tokens_to_process.push_back(token.clone());
        }

        // Process tokens in the deque until all are consumed
        while let Some(token) = tokens_to_process.pop_front() {
            let process_instance_id = token.process_instance_id().clone();
            let process_process_instances_by_id_by_id =
                self.process_instance_data_by_id.lock().unwrap();
            let process_instance_data = process_process_instances_by_id_by_id
                .get(&process_instance_id)
                .unwrap();
            let process_instance_id = process_instance_data.process_instance.id();
            let process_id = process_instance_data.process_instance.process_id();
            let process_definitions = self.process_definitions.lock().unwrap();
            let process = process_definitions.get(process_id).unwrap();
            let process_id = process.id();
            let flow_object_id = token.flow_object_id();

            // Handle loops (if needed) or assign None
            // TODO handle loops
            let loop_id = None;

            // Create PathElement for this token
            let path_element = PathElement::new(
                process_id.clone(),
                flow_object_id.clone(),
                process_instance_id.clone(),
                loop_id,
            );

            // Create the path identifier with the path element
            let mut path_elements: Vec<PathElement> = Vec::new();
            if let Some(parent_token_id) = token.parent_token_id() {
                let parent_path_elements = path_identifiers
                    .get(parent_token_id)
                    .unwrap()
                    .path_elements()
                    .clone();
                path_elements.extend(parent_path_elements);
            }
            path_elements.push(path_element);
            let path_identifier = PathIdentifier::new(path_elements);

            // Insert the token's PathIdentifier into the map
            path_identifiers.insert(token.id().clone(), path_identifier);

            // Add child tokens to the deque for further processing
            for child_token in token.child_tokens() {
                tokens_to_process.push_back(child_token.clone());
            }
        }

        path_identifiers
    }

    fn to_token_details(&self, token: &Token, process_instance: &ProcessInstance) -> TokenDetails {
        let process_instance_id = token.process_instance_id();
        let process_process_instances_by_id = self.process_instance_data_by_id.lock().unwrap();
        let process_instance_data = process_process_instances_by_id
            .get(process_instance_id)
            .unwrap();
        let token_id = token.id().clone();
        let path_identifier = process_instance_data
            .path_identifiers
            .get(&token_id)
            .unwrap()
            .clone();

        let parent_token_id = match token.parent_token_id() {
            Some(token_id) => Some(token_id.clone()),
            None => None,
        };
        let child_tokens = token
            .child_tokens()
            .iter()
            .map(|child_token| self.to_token_details(child_token, process_instance))
            .collect();
        TokenDetails {
            token_id: token.id().clone(),
            path_identifier,
            state: token.state().clone(),
            parent_token_id,
            child_tokens,
        }
    }

    fn process_ready_tokens(
        &mut self,
        instance_id: &ProcessInstanceId,
    ) -> Result<(), ProcessEngineError> {
        let mut ready_tokens_to_process: VecDeque<Token> = VecDeque::new();

        // Lock and get mutable access to process_instance_data
        let mut process_instances_by_id = self.process_instance_data_by_id.lock().unwrap();
        let process_instance_data = process_instances_by_id.get_mut(instance_id).ok_or(
            ProcessEngineError::ProcessInstanceNotFound(instance_id.clone()),
        )?;

        // Collect the ready tokens
        let ready_tokens = process_instance_data.process_instance.get_ready_tokens();
        ready_tokens_to_process.extend(ready_tokens.into_iter().cloned());

        // Use a mutable reference to the process instance directly
        let process_instance = &mut process_instance_data.process_instance;

        // Iterate over ready tokens and initialize flow objects
        while let Some(mut token) = ready_tokens_to_process.pop_front() {
            let flow_object_instance = process_instance
                .flow_object_instance(token.id())
                .ok_or_else(|| ProcessEngineError::FlowObjectInstanceNotFound(token.id().clone()))?
                .clone();
            let process_scope = process_instance.process_scope_mut();

            match flow_object_instance {
                FlowObjectInstance::StartEvent(start_event_behavior) => {
                    start_event_behavior
                        .initialize(process_scope, &mut token)
                        .map_err(ProcessEngineError::FlowObjectBehaviorError)?;
                }
                FlowObjectInstance::EndEvent(end_event_behavior) => {
                    end_event_behavior
                        .initialize(process_scope, &mut token)
                        .map_err(ProcessEngineError::FlowObjectBehaviorError)?;
                }
                FlowObjectInstance::Task(task_behavior) => {
                    task_behavior
                        .initialize(process_scope, &mut token)
                        .map_err(ProcessEngineError::FlowObjectBehaviorError)?;
                }
            }
        }

        Ok(())
    }
}
