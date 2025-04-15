use bevy::prelude::*;

use ethnolib::sandbox::EntityId;

#[derive(Resource, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BVH(pub Option<ethnolib::Node<EntityId>>);
