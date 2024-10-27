use super::FlowObjectId;

/// An Activity is a type of FlowObject that is tipically a Task or a SubProcess
pub trait Activity {
    fn id(&self) -> &FlowObjectId;
    fn name(&self) -> &str;
}
