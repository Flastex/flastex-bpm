use log::debug;
use quick_xml::name::QName;

use crate::bpmn::model::{
    errors::BPMNParseError,
    flow_objects::{
        task::{Task, TaskType},
        FlowObject, FlowObjectType,
    },
    process::Process,
};
use crate::commons::xml::utils::extract_attribute;

/// function to parse task elements and add them to the process
pub(crate) fn parse_task_element(
    process: &mut Process,
    element: &quick_xml::events::BytesStart,
) -> Result<(), BPMNParseError> {
    debug!("Parsing <task> element");
    let id = extract_attribute(element, &QName(b"id"))?;
    let name = extract_attribute(element, &QName(b"name"))?;
    // let task_type_str = extract_attribute(element, &QName(b"type"))?;
    // let task_type = TaskType::from_str(&task_type_str)?;
    let task_type = TaskType::UserTask;

    let task = Task::new(&id, &name, &task_type);
    let flow_object = FlowObject {
        id: id.clone(),
        flow_object_type: FlowObjectType::Task,
        flow_object_behavior: Box::new(task),
    };
    process.add_flow_object(flow_object)?;
    Ok(())
}
