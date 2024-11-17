// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use log::debug;
use serde::{Deserialize, Serialize};

use crate::bpmn::{
    model::flow_objects::FlowObjectId,
    runtime::{
        scope::Scope,
        token::{Token, TokenState},
    },
};

use super::flow_object_behavior::{
    EventHandler, FlowObjectBehavior, FlowObjectBehaviorError, Initializable, TokenHandler,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskBehavior {
    pub id: FlowObjectId,
}

impl TokenHandler for TaskBehavior {
    fn handle_token(
        &self,
        _parent_scope: &mut Scope,
        token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError> {
        debug!("Handling token {token} for TaskBehavior");
        Ok(())
    }
}

impl EventHandler for TaskBehavior {
    fn handle_external_event<T: 'static>(
        &self,
        _event: T,
        _parent_scope: &mut Scope,
        _token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError> {
        debug!("Handling external event for TaskBehavior");
        Ok(())
    }
}

impl Initializable for TaskBehavior {
    fn initialize(
        &self,
        _parent_scope: &mut Scope,
        token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError> {
        debug!("Initializing TaskBehavior");
        token.set_state(TokenState::Consumed);
        Ok(())
    }

    fn finalize(
        &self,
        _parent_scope: &mut Scope,
        _token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError> {
        debug!("Finalizing TaskBehavior");
        Ok(())
    }
}

impl FlowObjectBehavior for TaskBehavior {
    fn flow_object_id(&self) -> FlowObjectId {
        self.id.clone()
    }
}
