use bevy::prelude::*;
use ethnolib::Number;
use qol::logy;

use crate::systems::{
    BVH,
    query::{GetEntitiesRequest, GetEntitiesResult},
};

pub fn get_entities_system(
    mut requests: EventReader<GetEntitiesRequest>,
    mut result: EventWriter<GetEntitiesResult>,
    bvh: Res<BVH>,
) {
    let Some(bvh) = &bvh.0 else {
        logy!(
            "error",
            "No BVH not responding to any request. does this drop them or will they still be there next time?"
        );
        return;
    };

    for GetEntitiesRequest {
        agent_id,
        prayer_id,
        min_x,
        min_y,
        max_x,
        max_y,
    } in requests.read()
    {
        let entities = bvh.qurry(
            &Into::<Number>::into(*min_x),
            &Into::<Number>::into(*min_y),
            &Into::<Number>::into(*max_x),
            &Into::<Number>::into(*max_y),
        );
        result.send(GetEntitiesResult {
            agent_id: *agent_id,
            prayer_id: *prayer_id,
            entities,
        });
    }
}
