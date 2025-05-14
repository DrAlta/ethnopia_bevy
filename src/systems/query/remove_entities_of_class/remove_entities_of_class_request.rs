use std::sync::Arc;

use bevy::prelude::*;

use ethnolib::sandbox::{EntityId, Item, ai::TableInterior};

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RemoveEntitiesOfClassRequest {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub item_class: Item,
    pub table: Arc<TableInterior>,
}
