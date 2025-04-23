use bevy::prelude::*;
use ethnolib::sandbox::world::Energy;

use crate::systems::query::{GetEnergyRequest, GetEnergyResult};

pub fn get_energy_system(
    query: Query<&Energy>,
    mut requests: EventReader<GetEnergyRequest>,
    mut result: EventWriter<GetEnergyResult>,
) {
    for GetEnergyRequest {
        agent_id,
        prayer_id,
        subject_id,
    } in requests.read()
    {
        let energy_maybe = if let Ok(Energy(energy)) = query.get(*subject_id) {
            Some(*energy)
        } else {
            None
        };
        let event = GetEnergyResult {
            agent_id: *agent_id,
            prayer_id: *prayer_id,
            energy_maybe,
        };
        result.send(event);
    }
}
