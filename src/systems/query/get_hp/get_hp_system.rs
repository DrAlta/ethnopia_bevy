use bevy::prelude::*;
use ethnolib::sandbox::world::Hp;

use crate::systems::query::{GetHpRequest, GetHpResult};

pub fn get_hp_system(
    query: Query<&Hp>,
    mut requests: EventReader<GetHpRequest>,
    mut result: EventWriter<GetHpResult>,
) {
    for GetHpRequest {
        agent_id,
        prayer_id,
        subject_id,
    } in requests.read()
    {
        let hp_maybe = if let Ok(Hp(hp)) = query.get(*subject_id) {
            Some(*hp)
        } else {
            None
        };
        let event = GetHpResult {
            agent_id: *agent_id,
            prayer_id: *prayer_id,
            hp_maybe,
        };
        result.send(event);
    }
}
