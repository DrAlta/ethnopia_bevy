use bevy::prelude::*;
use ethnolib::sandbox::Item;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FindInInventoryRequest {
    pub agent_id: Entity,
    pub prayer_id: u64,
    pub item_class: Item,
}
