use bevy::prelude::*;
use ethnolib::sandbox::EntityId;

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CacheInventory{
    pub inventory: Vec<EntityId>
}