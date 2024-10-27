use std::collections::HashMap;

use crate::bpmn::model::process::Process;

use super::{
    flow_objects::FlowObjectState,
    token::{Token, TokenState},
};

#[derive(Debug, Clone)]
pub struct ProcessInstance {
    // Active tokens in the process
    pub tokens: Vec<Token>,
    // Track the state of flow object instances in the process
    pub flow_object_states: HashMap<String, FlowObjectState>,
    // Reference to the static BPMN model
    pub process: Process,
}

impl ProcessInstance {
    pub fn new(process: Process) -> Self {
        ProcessInstance {
            tokens: Vec::new(),
            flow_object_states: HashMap::new(),
            process,
        }
    }

    /// Adds a new token to the process
    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    /// Get all active tokens
    pub fn get_active_tokens(&self) -> Vec<&Token> {
        self.tokens
            .iter()
            .filter(|t| t.state == TokenState::Active)
            .collect()
    }

    /// Updates the state of a task
    pub fn update_task_state(&mut self, task_id: &str, state: FlowObjectState) {
        self.flow_object_states.insert(task_id.to_string(), state);
    }
}
