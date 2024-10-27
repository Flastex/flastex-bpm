#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum FlowObjectState {
    // The Flow Object has been instantiated but not yet started
    Created,
    // The Flow Object is ready for execution
    Ready,
    // The Flow Object is currently executing
    Active,
    // The Flow Object has been paused (e.g., waiting for a user task)
    Suspended,
    // The Flow Object has finished executing
    Completed,
    // The Flow Object encountered an error
    Failed,
    // The Flow Object was canceled before it could complete
    Canceled,
}
