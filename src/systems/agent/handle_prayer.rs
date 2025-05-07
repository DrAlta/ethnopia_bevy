use crate::systems::{
    actions::{DropRequest, GotoRequest, UseOnRequest},
    agent::AgentState,
    query::{FindInInventoryRequest, FindNearestRequest, GetEnergyRequest, GetHpRequest, GetLocationRequest},
};
use bevy::prelude::*;
use ethnolib::{
    Number,
    sandbox::{
        ai::{CPU, StackItem, Status},
        world::Movement,
    },
    vec2,
};
use qol::placeholder;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn handle_prayer(
    made_world_query: &mut bool,

    agent_id: Entity,
    ok: Status,
    cpu: &mut CPU,
    drop_request: &mut EventWriter<DropRequest>,
    goto_request: &mut EventWriter<GotoRequest>,
    find_in_inventory_request: &mut EventWriter<FindInInventoryRequest>,
    find_nearest_request: &mut EventWriter<FindNearestRequest>,
    get_energy_request: &mut EventWriter<GetEnergyRequest>,
    get_hp_request: &mut EventWriter<GetHpRequest>,
    get_location_request: &mut EventWriter<GetLocationRequest>,

    use_on_request: &mut EventWriter<UseOnRequest>,
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

            find_in_inventory_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
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

            use_on_request.send(request);
            
            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);

        },
        Status::FindNearest { x, y, item_class } => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "FindNearest".hash(&mut s);
            agent_id.hash(&mut s);
            x.hash(&mut s);
            y.hash(&mut s);
            item_class.hash(&mut s);
            let prayer_id = s.finish();
            let request = FindNearestRequest{ agent_id, prayer_id, min_radius: placeholder!(16 * 50), x, y };

            find_nearest_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        },
        Status::GetEnergy(subject_id) => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "GetEnergy".hash(&mut s);
            agent_id.hash(&mut s);
            subject_id.hash(&mut s);
            let prayer_id = s.finish();
            let request = GetEnergyRequest{ agent_id, prayer_id, subject_id };

            get_energy_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        },
        Status::GetLocation(subject_id) => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "GetLocation".hash(&mut s);
            agent_id.hash(&mut s);
            subject_id.hash(&mut s);
            let prayer_id = s.finish();
            let request = GetLocationRequest{ agent_id, prayer_id, subject_id };

            get_location_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        },
        Status::GetHp(subject_id) => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "GetLocation".hash(&mut s);
            agent_id.hash(&mut s);
            subject_id.hash(&mut s);
            let prayer_id = s.finish();
            let request = GetHpRequest{ agent_id, prayer_id, subject_id };

            get_hp_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        },
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
                        let mut s = DefaultHasher::new();
                        salt.hash(&mut s);
                        "Goto".hash(&mut s);
                        agent_id.hash(&mut s);
                        movement.hash(&mut s);
                        let prayer_id = s.finish();
                        goto_request.send(GotoRequest {
                            agent_id,
                            prayer_id,
                            movement,
                        });

                        *made_world_query = true;
                        *state = AgentState::WaitForAction(prayer_id);
                    }
                },
                ethnolib::sandbox::ai::InpulseId::Plant => {
                    let Some(StackItem::EntityId(object_id)) = cpu.stack.pop() else {
                        cpu.stack.push(StackItem::failure());

                        *made_world_query = true;
                        *state = AgentState::Running;
                        return
                    };
                    let mut s = DefaultHasher::new();
                    salt.hash(&mut s);
                    "Plant".hash(&mut s);
                    agent_id.hash(&mut s);
                    object_id.hash(&mut s);
                    let prayer_id = s.finish();

                    drop_request.send(DropRequest{ agent_id, prayer_id, object_id });

                    *made_world_query = true;
                    *state = AgentState::WaitForAction(prayer_id);

                },
                ethnolib::sandbox::ai::InpulseId::Take => todo!(),
                ethnolib::sandbox::ai::InpulseId::Use => todo!(),
                ethnolib::sandbox::ai::InpulseId::UseOn => todo!(),
                ethnolib::sandbox::ai::InpulseId::EatClass(_food_class) => {
                },
            }
        },
        Status::None => todo!(),
    }
}
