use crate::systems::{FindInInventoryRequest, agent::AgentState};
use bevy::prelude::*;
use ethnolib::{
    Number,
    sandbox::{
        actions::{GotoRequest, UseOnRequest},
        ai::{CPU, StackItem, Status},
        world::Movement,
    },
    vec2,
};
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn handle_prayer(
    agent_id: Entity,
    ok: Status,
    cpu: &mut CPU,
    goto_requests: &mut EventWriter<GotoRequest>,
    find_in_inventory_requests: &mut EventWriter<FindInInventoryRequest>,
    use_on_requests: &mut EventWriter<UseOnRequest>,
    state: &mut AgentState,
) -> () {
    let salt = 0;
    match ok {
        Status::Success => todo!(),
        Status::Failure => todo!(),
        Status::FindInInventory { item_class } => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "FindInInventory".hash(&mut s);
            agent_id.hash(&mut s);
            item_class.hash(&mut s);
            let prayer_id = s.finish();
            let request = FindInInventoryRequest{prayer_id, agent_id, item_class};

            find_in_inventory_requests.send(request);
        },
        Status::UseOn(tool_id, target_id) => {
            let request = UseOnRequest{ agent_id, tool_id, target_id };

            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "UseOn".hash(&mut s);
            agent_id.hash(&mut s);
            tool_id.hash(&mut s);
            target_id.hash(&mut s);
            //movement.hash(&mut s);
            let prayer_id = s.finish();

            use_on_requests.send(request);
            *state = AgentState::WaitForAction(prayer_id);

        },
        Status::FindNearest {../* x, y, item_class*/ } => todo!(),
        Status::GetEnergy(_entity) => todo!(),
        Status::GetLocation(_entity) => todo!(),
        Status::GetHp(_entity) => todo!(),
        Status::GetIsInventoryGE { ../*agent, item_class, amount*/ } => todo!(),
        Status::GetEntities { ../*min_x, min_y, max_x, max_y*/ } => todo!(),
        Status::RemoveEntitiesOfType(_item) => todo!(),
        Status::RetainEntitiesOfType(_item) => todo!(),
        Status::Running(inpulse_id) => {
            match inpulse_id{
                ethnolib::sandbox::ai::InpulseId::Act1 => todo!(),
                ethnolib::sandbox::ai::InpulseId::Act2 => todo!(),
                ethnolib::sandbox::ai::InpulseId::Act3 => todo!(),
                ethnolib::sandbox::ai::InpulseId::GoTo => {
                    if let Some(StackItem::Coord{ x, y }) = cpu.stack.pop() {
                        let movement = Movement{
                            target: vec2(
                                Into::<Number>::into(x),
                                Into::<Number>::into(y),
                            ),
                            speed: Number::FIVE
                        };
                        let salt = 0;
                        let mut s = DefaultHasher::new();
                        salt.hash(&mut s);
                        "Goto".hash(&mut s);
                        agent_id.hash(&mut s);
                        movement.hash(&mut s);
                        let prayer_id = s.finish();
                        goto_requests.send(GotoRequest {
                            agent_id,
                            prayer_id,
                            movement,
                        });
                        *state = AgentState::WaitForAction(prayer_id);
                    }
                },
                ethnolib::sandbox::ai::InpulseId::Plant => todo!(),
                ethnolib::sandbox::ai::InpulseId::Take => todo!(),
                ethnolib::sandbox::ai::InpulseId::Use => todo!(),
                ethnolib::sandbox::ai::InpulseId::EatClass(_food_class) => {
                },
            }
        },
        Status::None => todo!(),
    }
}
