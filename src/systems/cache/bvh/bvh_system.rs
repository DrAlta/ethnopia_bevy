use std::collections::HashMap;

use bevy::prelude::*;

use ethnolib::{sandbox::{world::Size, EntityId, Location}, types::AARect};

use crate::systems::cache::bvh::BVH;

pub fn bvh_system(
    query: Query<(Entity, &Location, &Size)>,
    mut bvh: ResMut<BVH>,
) {
    let mut aa_rects = HashMap::new();
    let mut list = Vec::new();
    for (id, location, Size { width, height }) in query.iter(){
        let Location::World { x, y } = location else {
            continue;
        };
//        let aa_rect = AARect{ min_x: x.into(), min_y: y.into(), width: width.into(), height: height.into() };
        let aa_rect = AARect{ min_x: x.clone(), min_y: y.clone(), width: (*width).into(), height: (*height).into() };
        aa_rects.insert(id, aa_rect);
        list.push(id);
    }
    bvh.0 =  ethnolib::Node::<EntityId>::create_tree(
        list,
        &move |id| {Some(aa_rects.get(id)?.clone())}
    ).ok();
}