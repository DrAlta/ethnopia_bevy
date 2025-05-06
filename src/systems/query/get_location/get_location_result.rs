use bevy::prelude::*;

use ethnolib::sandbox::{EntityId, Location};

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetLocationResult {
    pub agent_id: EntityId,
    pub prayer_id: u64,
    pub location_maybe: Option<Location>,
}
