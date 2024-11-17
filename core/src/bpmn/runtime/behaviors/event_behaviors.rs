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
    FlowObjectBehavior, FlowObjectBehaviorError, Initializable, TokenHandler,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StartEventBehavior {
    pub id: FlowObjectId,
}

impl TokenHandler for StartEventBehavior {
    fn handle_token(
        &self,
        _parent_scope: &mut Scope,
        token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError> {
        debug!("Handling token {token} for StartEventBehavior");
        Ok(())
    }
}

impl Initializable for StartEventBehavior {
    fn initialize(
        &self,
        _parent_scope: &mut Scope,
        token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError> {
        debug!("Initializing StartEventBehavior");
        token.set_state(TokenState::Completed);
        Ok(())
    }

    fn finalize(
        &self,
        _parent_scope: &mut Scope,
        _token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError> {
        debug!("Finalizing StartEventBehavior");
        Ok(())
    }
}

impl FlowObjectBehavior for StartEventBehavior {
    fn flow_object_id(&self) -> FlowObjectId {
        self.id.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndEventBehavior {
    pub id: FlowObjectId,
}

impl TokenHandler for EndEventBehavior {
    fn handle_token(
        &self,
        _parent_scope: &mut Scope,
        token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError> {
        debug!("Handling token {token} for EndEventBehavior");
        Ok(())
    }
}

impl Initializable for EndEventBehavior {
    fn initialize(
        &self,
        _parent_scope: &mut Scope,
        token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError> {
        debug!("Initializing EndEventBehavior");
        token.set_state(TokenState::Consumed);
        Ok(())
    }

    fn finalize(
        &self,
        _parent_scope: &mut Scope,
        _token: &mut Token,
    ) -> Result<(), FlowObjectBehaviorError> {
        debug!("Finalizing EndEventBehavior");
        Ok(())
    }
}

impl FlowObjectBehavior for EndEventBehavior {
    fn flow_object_id(&self) -> FlowObjectId {
        self.id.clone()
    }
}
