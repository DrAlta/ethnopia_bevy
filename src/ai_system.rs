use bevy::prelude::*;
use crate::{AI, EntitiesInAreaReponse, EntitiesInAreaRequest, Name};

pub fn ai_system(
    mut query: Query<(Entity, &Name, &mut AI)>,
    mut entities_in_area_events: EventReader<EntitiesInAreaReponse>,
    mut entities_requests: EventWriter<EntitiesInAreaRequest>,
) {
    println!("AI-----");
    for event in entities_in_area_events.read() {
        let name = if let Ok(thing) = query.get(event.agent_id) {
            format!("{}", thing.1.0)
        } else {
            format!("{:?}", event.agent_id)
 
        };
        println!("{} saw {:?}", name, event.entitie_ids);
    }
    for (agent_id, _, mut ai) in &mut query {
        let AI(count, limit ) = ai.as_mut();
        *count += 1;
        if count > limit {
            *count = 0;
            entities_requests.send(EntitiesInAreaRequest { agent_id: agent_id, pos: (0,0), size: (10,10) });
        }
    }
}
