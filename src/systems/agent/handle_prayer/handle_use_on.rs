use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::prelude::*;

use ethnolib::sandbox::EntityId;

use crate::systems::{Salt, actions::UseOnRequest, agent::AgentState};

pub fn handle_use_on(
    agent_id: EntityId,
    tool_id: EntityId,
    target_id: EntityId,
    made_world_query: &mut bool,
    state: &mut AgentState,
    salt: Salt,
    commands: &mut Commands,
) {
    let mut s = DefaultHasher::new();
    salt.hash(&mut s);
    "UseOn".hash(&mut s);
    agent_id.hash(&mut s);
    tool_id.hash(&mut s);
    target_id.hash(&mut s);
    //movement.hash(&mut s);
    let prayer_id = s.finish();

    let request = UseOnRequest {
        agent_id,
        prayer_id,
        tool_id,
        target_id,
    };

    commands.send_event(request);

    *made_world_query = true;
    *state = AgentState::WaitForAction(prayer_id);
}
