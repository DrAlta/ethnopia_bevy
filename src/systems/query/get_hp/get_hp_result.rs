use bevy::prelude::*;

use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetHpResult {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub hp_maybe: Option<i32>,
}
