use bevy::prelude::*;
use ethnolib::sandbox::{EntityId, ai::TableInterior};

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RemoveEntitiesOfClassResult {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub table: TableInterior,
}
