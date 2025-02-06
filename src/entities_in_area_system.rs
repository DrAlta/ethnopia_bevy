use std::collections::HashMap;

use bevy::{math::{bounding::{Aabb2d, IntersectsVolume}, vec2}, prelude::*};
use qol::{logy, PushOrInsert};
use crate::{EntitiesInAreaReponse, EntitiesInAreaRequest, Size};

/// what we want to od ia make a HashMap<EntityId, Vec<EntityId>>
/// then loop over the query and for each entiti make it's aabb 
/// then comparw it to the all the queries aabbs 
/// and if it intersects then put in the HashMap under the qurie's key
/// >
//
pub fn entities_in_area_system(
    mut entities_in_area_events: EventWriter<EntitiesInAreaReponse>,
    mut entities_requests: EventReader<EntitiesInAreaRequest>,
    query: Query<(Entity, &Transform, &Size)>,
) {
    println!("entities-----");
    // gathering up the (AI) queries so that we can loop over then for each 
    // entity in the (bevy) query. 
    let queries: HashMap<Entity, Vec<Aabb2d>> = entities_requests.read().fold(
        HashMap::new(),
        |mut acc, EntitiesInAreaRequest { agent_id, pos, size }|
        {
            let half_size = vec2(size.0 as f32 * 0.5, size.1 as f32 * 0.5);
            let aabb= Aabb2d::new(
            vec2(pos.0 as f32 , pos.1 as f32) + half_size,
                half_size
            );
            logy!("debug", "{agent_id:?}:{aabb:?}");
            acc.push_or_insert(agent_id.clone(), aabb);
            acc
        }
    );
    // building a HashMap of the EntityIds of the querying agents and their queries
    let mut reponses: HashMap<Entity, Vec<Vec<Entity>>> = queries.iter().map(|(k, v)| {
        let mut x = Vec::<Vec<Entity>>::new(); 
        for _ in 0 .. v.len() {
            x.push(Vec::new());
        }
        (k.clone(), x)
    }).collect();

    // this is where we loops over the entities in the (Bevy) query and put
    // then in the appropratie (AI) queries
    for (id, pos, size) in &query {
        // building the Aabb2d for this entity
        let half_size = vec2(size.0.0 as f32 * 0.5, size.0.0 as f32 * 0.5);
        let aabb= Aabb2d::new(
        pos.translation.truncate() + half_size,
            half_size
        );

        // looping over the querying agents
        for (k, outer) in &queries {
            // looping over their queries
            for (idx, v) in outer.iter().enumerate() {
                // does the query and the entity's aabb2d intersect?
                if v.intersects(&aabb) {
                    // if they do intersect added the entitie's id to 
                    // the approrap response
                    reponses.get_mut(k).unwrap()[idx].push(id.clone());
                }
            }
        }

    }
 
    // looping over the agents
    for (agent_id, outer) in reponses {
        // looping over the responses to their quries
        for response in outer {
            // sending the response to the agent
            entities_in_area_events.send(
                EntitiesInAreaReponse { agent_id, entitie_ids: response }
            );
        }
    }
}
