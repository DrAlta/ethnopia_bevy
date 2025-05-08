use bevy::prelude::*;
use ethnolib::sandbox::{EntityId, Item};

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetIsInventoryGERequest {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub subject_id: EntityId,
    pub item_class: Item,
    pub amount: i32,
}
