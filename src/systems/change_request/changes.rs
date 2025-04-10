use bevy::prelude::*;
use macros::Structs;

use crate::systems::change_request::Dispatch;
use ethnolib::sandbox::{EntityId, Item, Location};

//use super::dispatch::Dispatch2;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Structs)]
pub enum Changes {
    Despawn(EntityId),
    Energy { entity_id: EntityId, delta: i32 },
    Hp { entity_id: EntityId, delta: i32 },
    SpawnLocationType { location: Location, tyep: Item },
}
