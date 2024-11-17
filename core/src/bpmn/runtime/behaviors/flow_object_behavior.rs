// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use crate::bpmn::{
    model::flow_objects::FlowObjectId,
    runtime::{scope::Scope, token::Token},
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use thiserror::Error;

use super::{
    event_behaviors::{EndEventBehavior, StartEventBehavior},
    task_behaviors::TaskBehavior,
};

#[derive(Error, Debug)]
pub enum FlowObjectBehaviorError {
    #[error("Initialization Error: {0}.")]
    InitError(String),
    #[error("Finalization Error: {0}.")]
    FinalizeError(String),
    #[error("External Event Error: {0}.")]
    ExternalEventError(String),
    #[error("Generic Error:  {0}.")]
    GenericError(Box<dyn Error + Send + Sync>),
}

pub trait Initializable {
    /// Initializes the flow object behavior
    /// Token must be in the `READY` state
    fn initialize(
        &self,
        parent_scope: &mut Scope,
        token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError>;
    /// Finalizes the flow object behavior
    /// Token must be in the `COMPLETED` state
    fn finalize(
        &self,
        parent_scope: &mut Scope,
        token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError>;
}

pub trait TokenHandler {
    //// Handles a token
    fn handle_token(
        &self,
        parent_scope: &mut Scope,
        token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError>;
}

pub trait EventHandler {
    /// Handles an external event
    fn handle_external_event<T: 'static>(
        &self,
        event: T,
        parent_scope: &mut Scope,
        token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError>;
}

pub trait FlowObjectBehavior:
    Clone + Initializable + TokenHandler + Serialize + for<'de> Deserialize<'de>
{
    /// Tells to which Flow Object this behavior belongs to.
    fn flow_object_id(&self) -> FlowObjectId;
}

pub trait FlowObjectEventHandlerBehavior: FlowObjectBehavior + EventHandler {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FlowObjectInstance {
    // - Events
    StartEvent(StartEventBehavior),
    EndEvent(EndEventBehavior),
    // IntermediateCatchEvent(IntermediateCatchEventBehavior),
    // IntermediateThrowEvent(IntermediateThrowEventBehavior),
    // BoundaryEvent(BoundaryEventBehavior),
    // TimerEvent(TimerEventBehavior),
    // MessageEvent(MessageEventBehavior),
    // SignalEvent(SignalEventBehavior),
    // ErrorEvent(ErrorEventBehavior),
    // CompensationEvent(CompensationEventBehavior),
    // ConditionalEvent(ConditionalEventBehavior),
    // LinkEvent(LinkEventBehavior),
    // TerminateEvent(TerminateEventBehavior),
    // - Activities
    // -- Tasks
    Task(TaskBehavior),
    // CallActivity(CallActivityBehavior),
    // ReceiveTask(ReceiveTaskBehavior),
    // SendTask(SendTaskBehavior),
    // EventSubProcess(EventSubProcessBehavior),
    // SubProcess(SubProcessBehavior),
    // - Gateways
    // ExclusiveGateway(ExclusiveGatewayBehavior),
    // ParallelGateway(ParallelGatewayBehavior),
    // InclusiveGateway(InclusiveGatewayBehavior),
    // EventBasedGateway(EventBasedGatewayBehavior),

    // MultiInstanceLoopCharacteristics(MultiInstanceLoopCharacteristicsBehavior),
    // Complex
}
