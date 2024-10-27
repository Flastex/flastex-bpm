use std::collections::HashMap;

use log::debug;

use crate::bpmn::model::connecting_objects::sequence_flows::SequenceFlow;
use crate::bpmn::model::flow_objects::event::{Event, EventType};
use crate::bpmn::model::flow_objects::gateway::{GatewayBehavior, GatewayType};
use crate::bpmn::model::flow_objects::task::{is_automatic_task, Task};
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
            .find(|node| matches!(node.flow_object_type, FlowObjectType::Event))
            .and_then(|flow_object| {
                let event = flow_object
                    .flow_object_behavior
                    .as_any()
                    .downcast_ref::<Event>()
                    .expect("Failed to downcast to Event");
                if event.event_type == EventType::StartEvent {
                    Some(flow_object)
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
            FlowObjectType::Task => {
                let task = current_node
                    .flow_object_behavior
                    .as_any()
                    .downcast_ref::<Task>()
                    .expect("Failed to downcast FlowObjectBehavior to Task");
                if is_automatic_task(task) {
                    self.execute_task(task, token);
                    self.move_token_to_next_node(token);
                } else {
                    self.wait_for_user_task(task);
                }
            }
            FlowObjectType::Gateway => {
                let gateway = current_node
                    .flow_object_behavior
                    .as_any()
                    .downcast_ref::<Box<dyn GatewayBehavior>>()
                    .expect("Failed to downcast FlowObjectBehavior to GatewayBehavior");
                self.handle_gateway(gateway, token);
            }
            FlowObjectType::Event => {
                let event = current_node
                    .flow_object_behavior
                    .as_any()
                    .downcast_ref::<Event>()
                    .expect("Failed to downcast FlowObjectBehavior to Event");
                self.handle_event(event, token);
            }
        }
    }

    /// Executes an automatic task (e.g., service or script task)
    fn execute_task(&self, task: &Task, _token: &Token) {
        println!("Executing task: {:?}", task);
    }

    /// Pauses the process and waits for user input
    fn wait_for_user_task(&mut self, task: &Task) {
        println!("Waiting for user input for task: {:?}", task);
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

    fn handle_gateway(&mut self, gateway: &Box<dyn GatewayBehavior>, _token: &Token) {
        match gateway.gateway_type() {
            GatewayType::ComplexGateway => {
                println!("Handling complex gateway: {:?}", gateway);
            }
            GatewayType::EventBasedGateway => {
                println!("Handling event-based gateway: {:?}", gateway);
            }
            GatewayType::ExclusiveGateway => {
                println!("Handling exclusive gateway: {:?}", gateway);
            }
            GatewayType::InclusiveGateway => {
                println!("Handling inclusive gateway: {:?}", gateway);
            }
            GatewayType::ParallelGateway => {
                println!("Handling parallel gateway: {:?}", gateway);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, _token: &Token) {
        // Logic for handling start, end, and intermediate events
        println!("Handling event: {:?}", event);
    }
}
