use std::collections::BTreeSet;

use bevy::prelude::*;

use crate::systems::change_request::Changes; 
use ethnolib::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChangeRequest {
    pub hash: u64,
    pub contentious_entities: BTreeSet<EntityId>,
    pub changes: Vec<Changes>,
}
