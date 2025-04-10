use bevy::prelude::*;

use ethnolib::{sandbox::EntityId, types::ActionId};

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosibleActionsResponce {
    pub agent_id: EntityId,
    pub target_id: EntityId,
    pub action_id: ActionId,
}
