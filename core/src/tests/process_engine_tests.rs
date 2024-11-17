// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use crate::bpmn::model::connecting_objects::sequence_flows::{ImmediateFlag, SequenceFlow};
    use crate::bpmn::model::flow_objects::activity::ActivityType;
    use crate::bpmn::model::flow_objects::event::{self, Event, EventType};
    use crate::bpmn::model::flow_objects::task::{self, Task, TaskType};
    use crate::bpmn::model::flow_objects::{FlowObject, FlowObjectType};
    use crate::bpmn::model::process::{Process, ProcessId};
    use crate::bpmn::runtime::process_engine::{self, ProcessEngine};

    #[test]
    fn test_execute_simple_process() {
        // ARRANGE: Define the process
        // Define the process elements
        let start_event = FlowObject::new(
            "start_event".into(),
            FlowObjectType::Event(EventType::StartEvent(Event::new(
                "Start Event",
                event::Type::StartEvent,
            ))),
        );

        let task = FlowObject::new(
            "task_1".into(),
            FlowObjectType::Activity(ActivityType::Task(TaskType::UserTask(Task::new(
                "User Task",
                task::Type::UserTask,
            )))),
        );

        let end_event = FlowObject::new(
            "end_event".into(),
            FlowObjectType::Event(EventType::EndEvent(Event::new(
                "End Event",
                event::Type::EndEvent,
            ))),
        );

        let mut sequence_flow_from_start_event_to_task_builder = SequenceFlow::builder();
        sequence_flow_from_start_event_to_task_builder.id("sequence_flow_1".into());
        sequence_flow_from_start_event_to_task_builder.source_ref(start_event.id().clone());
        sequence_flow_from_start_event_to_task_builder.target_ref(task.id().clone());
        sequence_flow_from_start_event_to_task_builder.normal();
        let sequence_flow_from_start_event_to_task = sequence_flow_from_start_event_to_task_builder
            .build()
            .unwrap();

        let mut sequence_flow_from_task_to_end_task_builder = SequenceFlow::builder();
        sequence_flow_from_task_to_end_task_builder.id("sequence_flow_2".into());
        sequence_flow_from_task_to_end_task_builder.source_ref(task.id().clone());
        sequence_flow_from_task_to_end_task_builder.target_ref(end_event.id().clone());
        sequence_flow_from_task_to_end_task_builder.normal();
        let sequence_flow_from_task_to_end_task =
            sequence_flow_from_task_to_end_task_builder.build().unwrap();

        // Create the process definition
        let mut process = Process::new();
        process.set_name("simple_process");
        process
            .add_flow_object(start_event.clone())
            .expect("Failed to add start_event flow object to process");
        process
            .add_flow_object(task.clone())
            .expect("Failed to add task flow object to process");
        process
            .add_flow_object(end_event.clone())
            .expect("Failed to add end_event flow object to process");

        // Link the elements
        process.add_sequence_flow(sequence_flow_from_start_event_to_task);
        process.add_sequence_flow(sequence_flow_from_task_to_end_task);

        // Define the Process Engine
        let process_definitions: Arc<Mutex<HashMap<ProcessId, Process>>> =
            Arc::new(Mutex::new(HashMap::new()));
        process_definitions
            .lock()
            .unwrap()
            .insert(process.id().clone(), process.clone());
        let process_engine_factory = process_engine::ProcessEngineFactory::new();
        let mut process_engine = process_engine_factory.create_process_engine(process_definitions);

        // ACT: Execute the process

        // Create a new instance of the process
        let process_instance_id = process_engine.create_instance(&process.id()).unwrap();

        // Initialize the process instance
        let start_instance_result = process_engine.start_instance(&process_instance_id.clone());

        // Assertions to verify process execution
        assert!(
            start_instance_result.is_ok(),
            "Execution should succeed without errors"
        );
    }
}
