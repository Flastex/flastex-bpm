// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

#[cfg(test)]
mod tests {
    use crate::bpmn::model::connecting_objects::sequence_flows::{ConditionalFlow, NormalFlow};
    use crate::bpmn::model::flow_objects::activity::ActivityType;
    use crate::bpmn::model::flow_objects::event::{self, EventType};
    use crate::bpmn::model::flow_objects::gateway::{self, GatewayDirection, GatewayType};
    use crate::bpmn::model::flow_objects::{task, FlowObjectType};
    use crate::bpmn::model::script::ScriptType;
    use crate::bpmn::model::xml_parser::parse_bpmn_from_str;

    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_simple_process_v1() {
        // ARRANGE: Load the BPMN XML file from the resources/tests directory
        let mut file =
            File::open("../resources/tests/simple_process_v1.bpmn").expect("File not found");

        let mut xml_content = String::new();
        file.read_to_string(&mut xml_content)
            .expect("Error reading file");
        // ACT: Parse the BPMN file
        match parse_bpmn_from_str(&xml_content) {
            Ok(process) => {
                // ASSERT: Verify that the process contains all the expected flow objects and sequence flows
                assert_eq!(
                    process.flow_objects().len(),
                    5,
                    "Flow objects count mismatch"
                );

                // Assert specific flow objects exist
                let start_event = process
                    .flow_objects()
                    .get("startEvent")
                    .expect("Start event not found");
                assert_eq!(
                    start_event.flow_object_type,
                    FlowObjectType::Event(EventType::StartEvent(event::Event::new(
                        "Start Event",
                        event::Type::StartEvent
                    ))),
                    "Incorrect start event type"
                );

                let user_task = process
                    .flow_objects()
                    .get("userTask")
                    .expect("User task not found");
                assert_eq!(
                    user_task.flow_object_type,
                    FlowObjectType::Activity(ActivityType::Task(task::TaskType::UserTask(
                        task::Task::new("User Task", task::Type::UserTask)
                    ))),
                    "Incorrect task type"
                );

                let gateway = process
                    .flow_objects()
                    .get("gateway1")
                    .expect("Gateway not found");
                assert_eq!(
                    gateway.flow_object_type,
                    FlowObjectType::Gateway(GatewayType::ExclusiveGateway(
                        gateway::ExclusiveGateway::new(
                            "Decision Gateway",
                            GatewayDirection::Diverging,
                            None
                        )
                    )),
                    "Incorrect gateway type"
                );

                let end_event1 = process
                    .flow_objects()
                    .get("endEvent1")
                    .expect("End event 1 not found");
                assert_eq!(
                    end_event1.flow_object_type,
                    FlowObjectType::Event(EventType::EndEvent(event::Event::new(
                        "End Event 1",
                        event::Type::EndEvent
                    ))),
                    "Incorrect event type for endEvent1"
                );

                let end_event2 = process
                    .flow_objects()
                    .get("endEvent2")
                    .expect("End event 2 not found");
                assert_eq!(
                    end_event2.flow_object_type,
                    FlowObjectType::Event(EventType::EndEvent(event::Event::new(
                        "End Event 2",
                        event::Type::EndEvent
                    ))),
                    "Incorrect event type for endEvent2"
                );

                // Assert sequence flows
                let sequence_flows = process.sequence_flows();
                assert_eq!(sequence_flows.len(), 4, "Sequence flows count mismatch");

                let flow1 = sequence_flows
                    .iter()
                    .find(|flow| flow.id() == "flow1")
                    .expect("Sequence flow 'flow1' not found");
                assert_eq!(
                    flow1.source_ref(),
                    "startEvent",
                    "Incorrect source for flow1"
                );
                assert_eq!(flow1.target_ref(), "userTask", "Incorrect target for flow1");

                let flow2 = sequence_flows
                    .iter()
                    .find(|flow| flow.id() == "flow2")
                    .expect("Sequence flow 'flow2' not found");
                assert_eq!(flow2.source_ref(), "userTask", "Incorrect source for flow2");
                assert_eq!(flow2.target_ref(), "gateway1", "Incorrect target for flow2");

                let flow3 = sequence_flows
                    .iter()
                    .find(|flow| flow.id() == "flow3")
                    .expect("Sequence flow 'flow3' not found");
                assert_eq!(flow3.source_ref(), "gateway1", "Incorrect source for flow3");
                assert_eq!(
                    flow3.target_ref(),
                    "endEvent1",
                    "Incorrect target for flow3"
                );

                let flow4 = sequence_flows
                    .iter()
                    .find(|flow| flow.id() == "flow4")
                    .expect("Sequence flow 'flow4' not found");
                assert_eq!(flow4.source_ref(), "gateway1", "Incorrect source for flow4");
                assert_eq!(
                    flow4.target_ref(),
                    "endEvent2",
                    "Incorrect target for flow4"
                );
            }
            Err(e) => {
                panic!("Failed to parse BPMN XML: {:?}", e);
            }
        }
    }

    #[test]
    fn test_parse_all_sequence_flows_process_v1() {
        // ARRANGE: Load the BPMN XML file from the resources/tests directory
        let mut file = File::open("../resources/tests/all_sequence_flows_process_v1.bpmn")
            .expect("File not found");

        let mut xml_content = String::new();
        file.read_to_string(&mut xml_content)
            .expect("Error reading file");

        // ACT: Parse the BPMN file
        match parse_bpmn_from_str(&xml_content) {
            Ok(process) => {
                // ASSERT: Verify that the process contains all the expected flow objects and sequence flows
                assert_eq!(
                    process.flow_objects().len(),
                    11,
                    "Flow objects count mismatch"
                );

                // Assert specific flow objects exist
                let start_event = process
                    .flow_objects()
                    .get("startEvent")
                    .expect("Start event not found");
                assert_eq!(
                    start_event.flow_object_type,
                    FlowObjectType::Event(EventType::StartEvent(event::Event::new(
                        "Start Event",
                        event::Type::StartEvent
                    ))),
                    "Incorrect start event type"
                );

                for i in 1..=8 {
                    let task = process
                        .flow_objects()
                        .get(&format!("task{}", i))
                        .expect(&format!("Task{} not found", i));
                    assert_eq!(
                        task.flow_object_type,
                        FlowObjectType::Activity(ActivityType::Task(task::TaskType::UserTask(
                            task::Task::new(format!("Task {i}").as_str(), task::Type::UserTask)
                        ))),
                        "Incorrect task type"
                    );
                }

                let gateway = process
                    .flow_objects()
                    .get("gateway")
                    .expect("Gateway not found");
                assert_eq!(
                    gateway.flow_object_type,
                    FlowObjectType::Gateway(GatewayType::ParallelGateway(
                        gateway::ParallelGateway::new(
                            "Decision Gateway",
                            GatewayDirection::Unspecified
                        )
                    )),
                    "Incorrect gateway type"
                );

                let end_event = process
                    .flow_objects()
                    .get("endEvent1")
                    .expect("End event not found");
                assert_eq!(
                    end_event.flow_object_type,
                    FlowObjectType::Event(EventType::EndEvent(event::Event::new(
                        "End Event 1",
                        event::Type::EndEvent
                    ))),
                    "Incorrect end event type"
                );

                // Assert sequence flows
                let sequence_flows = process.sequence_flows();
                assert_eq!(sequence_flows.len(), 17, "Sequence flows count mismatch");

                // Assert conditions for flows 2-9
                let expected_conditions = vec![
                    ("flow2", "${resendRequest == 'Yes'}", None),
                    (
                        "flow3",
                        "return myVariable == true;",
                        Some("http://www.javascript.com"),
                    ),
                    (
                        "flow4",
                        "my_variable == 'approved'",
                        Some("http://www.python.org"),
                    ),
                    ("flow5", "myVariable > 10", Some("http://groovy-lang.org")),
                    (
                        "flow6",
                        "myVariable.equals(\"yes\")",
                        Some("http://www.java.com"),
                    ),
                    (
                        "flow7",
                        "return myVariable == \"approved\"",
                        Some("http://www.lua.org"),
                    ),
                    (
                        "flow8",
                        "#{myVariable == 'yes'}",
                        Some("http://www.juel.org"),
                    ),
                    (
                        "flow9",
                        "${resendRequest == 'Yes'}",
                        Some("http://www.w3.org/1999/XPath"),
                    ),
                ];

                for (flow_id, expected_expression, expected_language) in expected_conditions {
                    let flow = sequence_flows
                        .iter()
                        .find(|flow| flow.id() == flow_id)
                        .expect(&format!("Sequence flow '{}' not found", flow_id));
                    let sequence_flow_behavior = flow.sequence_flow_behavior();
                    let conditional_flow: ConditionalFlow = sequence_flow_behavior
                        .as_any()
                        .downcast_ref::<ConditionalFlow>()
                        .expect("Expected ConditionalFlow")
                        .clone();
                    assert!(conditional_flow.condition_expression().is_some());
                    let condition_expression = conditional_flow.condition_expression().unwrap();
                    // Assert the script content
                    assert_eq!(
                        condition_expression.script(),
                        expected_expression,
                        "Incorrect condition expression for {}",
                        flow_id
                    );

                    // Assert the language by matching the Script variant to the expected ScriptType's schema reference
                    let script_type = match expected_language {
                        Some(language) => ScriptType::from_schema_ref(language).unwrap(),
                        None => ScriptType::default(),
                    };
                    assert_eq!(
                        condition_expression.script_type().schema_ref(),
                        script_type.schema_ref(),
                        "Incorrect language for {}",
                        flow_id
                    );
                }

                // Assert other flows without conditions
                let other_flows = vec![
                    "flow1", "flow10", "flow11", "flow12", "flow13", "flow14", "flow15", "flow16",
                    "flow17",
                ];
                for flow_id in other_flows {
                    let flow = sequence_flows
                        .iter()
                        .find(|flow| flow.id() == flow_id)
                        .expect(&format!("Sequence flow '{}' not found", flow_id));
                    let sequence_flow_behavior = flow.sequence_flow_behavior();
                    assert!(sequence_flow_behavior
                        .as_any()
                        .downcast_ref::<NormalFlow>()
                        .is_some());
                }
            }
            Err(e) => {
                panic!("Failed to parse BPMN XML: {:?}", e);
            }
        }
    }
}
