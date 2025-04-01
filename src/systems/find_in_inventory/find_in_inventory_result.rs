use bevy::prelude::*;
use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindInInventoryResult{
    pub agent_id: EntityId,
    pub action_id: u64,
    pub found_item_id_maybe: Option<EntityId>
}