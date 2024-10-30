// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use log::debug;
use quick_xml::name::QName;

use crate::bpmn::model::{
    errors::BPMNParseError,
    flow_objects::{
        activity::ActivityType,
        task::{self, Task, TaskType},
        FlowObject, FlowObjectId, FlowObjectType,
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
    let id: FlowObjectId = extract_attribute(element, &QName(b"id"))?;
    let name = extract_attribute(element, &QName(b"name"))?;
    // let task_type_str = extract_attribute(element, &QName(b"type"))?;
    // let task_type = TaskType::from_str(&task_type_str)?;

    let task = Task::new(&name, task::Type::UserTask);
    let flow_object = FlowObject {
        id: id.clone(),
        flow_object_type: FlowObjectType::Activity(ActivityType::Task(TaskType::UserTask(task))),
    };
    process.add_flow_object(flow_object)?;
    Ok(())
}
