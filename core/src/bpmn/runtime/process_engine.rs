// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::collections::HashMap;

use log::debug;

use crate::bpmn::model::connecting_objects::sequence_flows::SequenceFlow;
use crate::bpmn::model::flow_objects::activity::ActivityType;
use crate::bpmn::model::flow_objects::event::EventType;
use crate::bpmn::model::flow_objects::gateway::GatewayType;
use crate::bpmn::model::flow_objects::task::{is_automatic_task, TaskType};
use crate::bpmn::model::flow_objects::{FlowObject, FlowObjectType};
use crate::bpmn::runtime::process::ProcessInstance;
use crate::bpmn::runtime::token::{Token, TokenState};
use crate::bpmn::runtime::utils::token_id_utils::generate_token_id;

use super::path_identifier::PathIdentifier;

pub struct ProcessEngine {
    pub process_instance: ProcessInstance, // The running process instance
}

impl ProcessEngine {
    pub fn new(process: ProcessInstance) -> Self {
        let mut executor = ProcessEngine {
            process_instance: process,
        };
        executor.initialize_tokens();
        executor
    }

    /// Initialize tokens by finding the Start Event and creating a Token
    fn initialize_tokens(&mut self) {
        // Find the Start Event
        if let Some(start_event) = self.find_start_event() {
            // Create the initial token for the Start Event
            let path_identifier = PathIdentifier::new_for_event(start_event.id.clone(), None);
            debug!("Path identifier: {:?}", path_identifier);
            let token = Token {
                id: generate_token_id(path_identifier, None),
                current_node_id: start_event.id.clone(),
                state: TokenState::Active,
                parent_token_id: None,
                child_tokens: vec![],
                process_variables: HashMap::new(),
            };
            self.process_instance.add_token(token);
        } else {
            panic!("No start event found in the process.");
        }
    }

    /// Finds the Start Event in the process
    fn find_start_event(&self) -> Option<&FlowObject> {
        self.process_instance
            .process
            .flow_objects()
            .values()
            .find_map(|node| {
            if let FlowObjectType::Event(event_type) = &node.flow_object_type {
                if let EventType::StartEvent(_) = event_type {
                    Some(node)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    /// Run the process by moving tokens through tasks, events, and gateways
    pub fn run(&mut self) {
        while let Some(mut token) = self.process_instance.tokens.pop() {
            self.process_token(&mut token);
        }
    }

    fn process_token(&mut self, token: &mut Token) {
        let current_node_id = token.current_node_id.clone();
        let current_node = self
            .process_instance
            .process
            .flow_objects()
            .get(&current_node_id)
            .unwrap()
            .clone();

        match current_node.flow_object_type {
            FlowObjectType::Activity(ActivityType::Task(task_type)) => {
                if is_automatic_task(&task_type) {
                    self.execute_task(&task_type, token);
                    self.move_token_to_next_node(token);
                } else {
                    self.wait_for_user_task(&task_type);
                }
            }
            FlowObjectType::Gateway(gateway_type) => {
                self.handle_gateway(gateway_type, token);
            }
            FlowObjectType::Event(event_type) => {
                self.handle_event(&event_type, token);
            }
        }
    }

    /// Executes an automatic task (e.g., service or script task)
    fn execute_task(&self, task_type: &TaskType, _token: &Token) {
        println!("Executing task: {:?}", task_type);
    }

    /// Pauses the process and waits for user input
    fn wait_for_user_task(&mut self, task_type: &TaskType) {
        println!("Waiting for user input for task: {:?}", task_type);
    }

    /// Moves a token to the next node based on outgoing sequence flows
    fn move_token_to_next_node(&mut self, token: &mut Token) {
        let current_node = self
            .process_instance
            .process
            .flow_objects()
            .get(&token.current_node_id)
            .unwrap();
        let outgoing_flows: Vec<&SequenceFlow> = self
            .process_instance
            .process
            .sequence_flows()
            .iter()
            .filter(|flow| *flow.source_ref() == current_node.id)
            .collect();

        for flow in outgoing_flows {
            token.current_node_id.clone_from(&flow.target_ref());
            token.state = TokenState::Active;
            println!("Moved token to next node: {}", token.current_node_id);
        }
    }

    fn handle_gateway(&mut self, gateway_type: GatewayType, _token: &Token) {
        match gateway_type {
            GatewayType::ComplexGateway(gateway) => {
                println!("Handling complex gateway: {:?}", gateway);
            }
            GatewayType::EventBasedGateway(gateway) => {
                println!("Handling event-based gateway: {:?}", gateway);
            }
            GatewayType::ExclusiveGateway(gateway) => {
                println!("Handling exclusive gateway: {:?}", gateway);
            }
            GatewayType::InclusiveGateway(gateway) => {
                println!("Handling inclusive gateway: {:?}", gateway);
            }
            GatewayType::ParallelGateway(gateway) => {
                println!("Handling parallel gateway: {:?}", gateway);
            }
        }
    }

    fn handle_event(&mut self, event_type: &EventType, _token: &Token) {
        // Logic for handling start, end, and intermediate events
        println!("Handling event: {:?}", event_type);
    }
}
