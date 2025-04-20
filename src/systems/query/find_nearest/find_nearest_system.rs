use std::sync::OnceLock;

use bevy::prelude::*;
use ethnolib::{Number, sandbox::Location};
use qol::{logy, placeholder};

use crate::systems::{BVH, FindNearestRequest, FindNearestResult};

pub fn default_search_step() -> i32 {
    static DEFAULT_SEARCH_STEP: OnceLock<i32> = OnceLock::new();
    DEFAULT_SEARCH_STEP
        .get_or_init(|| placeholder!(16 * 2))
        .clone()
}

pub fn find_nearest_system(
    query: Query<&Location>,
    mut requests: EventReader<FindNearestRequest>,
    mut result: EventWriter<FindNearestResult>,
    bvh: Res<BVH>,
) {
    let Some(bvh) = &bvh.0 else {
        logy!(
            "error",
            "No BVH not responding to any request. does this drop them or will they still be there next time?"
        );
        return;
    };
    'request_loop: for FindNearestRequest {
        agent_id,
        prayer_id,
        min_radius,
        x,
        y,
    } in requests.read()
    {
        let mut closest_maybe = None;
        for item_id in bvh.qurry(
            &(x - default_search_step()).into(),
            &(y - default_search_step()).into(),
            &(x + default_search_step()).into(),
            &(y + default_search_step()).into(),
        ) {
            let Ok(Location::World {
                x: item_x,
                y: item_y,
            }) = query.get(*agent_id)
            else {
                continue;
            };
            if let Some((_closest_id, closest_squared_distance)) = closest_maybe.as_ref() {
                let squared_distance = (Into::<Number>::into(*x) - item_x).powi(2)
                    + (Into::<Number>::into(*y) - item_y).powi(2);
                if &squared_distance > closest_squared_distance {
                    closest_maybe = Some((item_id.clone(), squared_distance))
                } else {
                    closest_maybe = Some((item_id.clone(), squared_distance))
                }
            }
        }

        let mut center = default_search_step();
        while center <= *min_radius {
            if closest_maybe.is_some() {
                result.send(FindNearestResult {
                    agent_id: *agent_id,
                    prayer_id: *prayer_id,
                    found_item_id_maybe: closest_maybe,
                });
                continue 'request_loop;
            };
            for ethnolib::Box {
                min_x,
                min_y,
                width,
                height,
            } in ethnolib::ring(*x, *y, center, default_search_step())
            {
                for item_id in bvh.qurry(
                    &Into::<Number>::into(min_x),
                    &Into::<Number>::into(min_y),
                    &Into::<Number>::into(min_x + width),
                    &Into::<Number>::into(min_y + height),
                ) {
                    let Ok(Location::World {
                        x: item_x,
                        y: item_y,
                    }) = query.get(*agent_id)
                    else {
                        continue;
                    };
                    if let Some((_closest_id, closest_squared_distance)) = closest_maybe.as_ref() {
                        let squared_distance = (Into::<Number>::into(*x) - item_x).powi(2)
                            + (Into::<Number>::into(*y) - item_y).powi(2);
                        if &squared_distance > closest_squared_distance {
                            closest_maybe = Some((item_id.clone(), squared_distance))
                        } else {
                            closest_maybe = Some((item_id.clone(), squared_distance))
                        }
                    }
                }
            }
            center += default_search_step();
        }
        result.send(FindNearestResult {
            agent_id: *agent_id,
            prayer_id: *prayer_id,
            found_item_id_maybe: None,
        });
    }
}
