use crate::systems::{
    actions::{DropRequest, GotoRequest, TakeRequest, UseOnRequest, UseRequest},
    agent::AgentState,
    query::{
        FindInInventoryRequest, FindNearestRequest, GetEnergyRequest, GetEntitiesRequest,
        GetHpRequest, GetIsInventoryGERequest, GetLocationRequest, RemoveEntitiesOfClassRequest,
        RetainEntitiesOfClassRequest,
    },
};
use bevy::prelude::*;
use ethnolib::{
    Number,
    sandbox::{
        ai::{CPU, StackItem, Status, ThreadName},
        world::Movement,
    },
    vec2,
};
use qol::{logy, placeholder};
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn handle_prayer(
    made_world_query: &mut bool,

    agent_id: Entity,
    main: &ThreadName,
    ok: Status,
    cpu: &mut CPU,
    /*
    drop_request: &mut EventWriter<DropRequest>,
    goto_request: &mut EventWriter<GotoRequest>,
    take_request: &mut EventWriter<TakeRequest>,
    use_request: &mut EventWriter<UseRequest>,
    use_on_request: &mut EventWriter<UseOnRequest>,
    */
    find_in_inventory_request: &mut EventWriter<FindInInventoryRequest>,
    find_nearest_request: &mut EventWriter<FindNearestRequest>,
    get_energy_request: &mut EventWriter<GetEnergyRequest>,
    get_entities_request: &mut EventWriter<GetEntitiesRequest>,
    get_hp_request: &mut EventWriter<GetHpRequest>,
    get_is_inventory_ge_request: &mut EventWriter<GetIsInventoryGERequest>,
    get_location_request: &mut EventWriter<GetLocationRequest>,
    remove_entities_of_class_request: &mut EventWriter<RemoveEntitiesOfClassRequest>,
    retain_entities_of_class_request: &mut EventWriter<RetainEntitiesOfClassRequest>,

    state: &mut AgentState,
    commands: &mut Commands,
) -> () {
    let salt = 0;
    match ok {
        Status::Success => {
            cpu.stack = vec![StackItem::success()];
            cpu.return_stack = Vec::new();
            cpu.pc = Some((main.clone(), 0));

            *state = AgentState::Running;
        }
        Status::Failure => {
            cpu.stack = vec![StackItem::failure()];
            cpu.return_stack = Vec::new();
            cpu.pc = Some((main.clone(), 0));

            *state = AgentState::Running;
        }
        Status::FindInInventory { item_class } => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "FindInInventory".hash(&mut s);
            agent_id.hash(&mut s);
            item_class.hash(&mut s);
            let prayer_id = s.finish();
            let request = FindInInventoryRequest {
                prayer_id,
                agent_id,
                item_class,
            };

            find_in_inventory_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        }
        Status::UseOn(tool_id, target_id) => {
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
        Status::FindNearest { x, y, item_class } => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "FindNearest".hash(&mut s);
            agent_id.hash(&mut s);
            x.hash(&mut s);
            y.hash(&mut s);
            item_class.hash(&mut s);
            let prayer_id = s.finish();
            let request = FindNearestRequest {
                agent_id,
                prayer_id,
                min_radius: placeholder!(16 * 50),
                x,
                y,
            };

            find_nearest_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        }
        Status::GetEnergy(subject_id) => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "GetEnergy".hash(&mut s);
            agent_id.hash(&mut s);
            subject_id.hash(&mut s);
            let prayer_id = s.finish();
            let request = GetEnergyRequest {
                agent_id,
                prayer_id,
                subject_id,
            };

            get_energy_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        }
        Status::GetLocation(subject_id) => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "GetLocation".hash(&mut s);
            agent_id.hash(&mut s);
            subject_id.hash(&mut s);
            let prayer_id = s.finish();
            let request = GetLocationRequest {
                agent_id,
                prayer_id,
                subject_id,
            };

            get_location_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        }
        Status::GetHp(subject_id) => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "GetHp".hash(&mut s);
            agent_id.hash(&mut s);
            subject_id.hash(&mut s);
            let prayer_id = s.finish();
            let request = GetHpRequest {
                agent_id,
                prayer_id,
                subject_id,
            };

            get_hp_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        }
        Status::GetIsInventoryGE {
            agent,
            item_class,
            amount,
        } => {
            let subject_id = agent;
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "GetIsInventoryGE".hash(&mut s);
            agent_id.hash(&mut s);
            subject_id.hash(&mut s);
            item_class.hash(&mut s);
            amount.hash(&mut s);
            let prayer_id = s.finish();
            let request = GetIsInventoryGERequest {
                agent_id,
                prayer_id,
                subject_id,
                item_class,
                amount,
            };

            get_is_inventory_ge_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        }
        Status::GetEntities {
            min_x,
            min_y,
            max_x,
            max_y,
        } => {
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "GetEntities".hash(&mut s);
            agent_id.hash(&mut s);
            min_x.hash(&mut s);
            min_y.hash(&mut s);
            max_x.hash(&mut s);
            max_y.hash(&mut s);
            let prayer_id = s.finish();
            let request = GetEntitiesRequest {
                agent_id,
                prayer_id,
                min_x,
                min_y,
                max_x,
                max_y,
            };

            get_entities_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        }
        Status::RemoveEntitiesOfType(item) => {
            let Some(StackItem::Table(_)) = cpu.stack.last() else {
                todo!()
            };
            let Some(StackItem::Table(table)) = cpu.stack.pop() else {
                unreachable!()
            };
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "RemoveEntitiesOfClass".hash(&mut s);
            agent_id.hash(&mut s);
            item.hash(&mut s);
            table.hash(&mut s);
            let prayer_id = s.finish();

            let request = RemoveEntitiesOfClassRequest {
                agent_id,
                prayer_id,
                item_class: item,
                table,
            };
            remove_entities_of_class_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        }
        Status::RetainEntitiesOfType(item) => {
            let Some(StackItem::Table(_)) = cpu.stack.last() else {
                todo!()
            };
            let Some(StackItem::Table(table)) = cpu.stack.pop() else {
                unreachable!()
            };
            let mut s = DefaultHasher::new();
            salt.hash(&mut s);
            "RemoveEntitiesOfClass".hash(&mut s);
            agent_id.hash(&mut s);
            item.hash(&mut s);
            table.hash(&mut s);
            let prayer_id = s.finish();

            let request = RetainEntitiesOfClassRequest {
                agent_id,
                prayer_id,
                item_class: item,
                table,
            };
            retain_entities_of_class_request.send(request);

            *made_world_query = true;
            *state = AgentState::WaitForAction(prayer_id);
        }
        Status::Running(inpulse_id) => match inpulse_id {
            ethnolib::sandbox::ai::InpulseId::Act1 |
            ethnolib::sandbox::ai::InpulseId::Act2 |
            ethnolib::sandbox::ai::InpulseId::Act3 => {
                logy!("warning", "got an prayer for {inpulse_id:?}");
            },
            ethnolib::sandbox::ai::InpulseId::GoTo => {
                if let Some(StackItem::Coord { x, y }) = cpu.stack.pop() {
                    let movement = Movement {
                        target: vec2(Into::<Number>::into(x), Into::<Number>::into(y)),
                        speed: Number::FIVE,
                    };
                    let mut s = DefaultHasher::new();
                    salt.hash(&mut s);
                    "Goto".hash(&mut s);
                    agent_id.hash(&mut s);
                    movement.hash(&mut s);
                    let prayer_id = s.finish();
                    let request = GotoRequest {
                        agent_id,
                        prayer_id,
                        movement,
                    };

                    commands.send_event(request);


                    *made_world_query = true;
                    *state = AgentState::WaitForAction(prayer_id);
                }
            }
            ethnolib::sandbox::ai::InpulseId::Plant => {
                let Some(StackItem::EntityId(object_id)) = cpu.stack.pop() else {
                    cpu.stack.push(StackItem::failure());

                    *made_world_query = true;
                    *state = AgentState::Running;
                    return;
                };
                let mut s = DefaultHasher::new();
                salt.hash(&mut s);
                "Plant".hash(&mut s);
                agent_id.hash(&mut s);
                object_id.hash(&mut s);
                let prayer_id = s.finish();

                let request = DropRequest {
                    agent_id,
                    prayer_id,
                    object_id,
                };

                commands.send_event(request);


                *made_world_query = true;
                *state = AgentState::WaitForAction(prayer_id);
            }
            ethnolib::sandbox::ai::InpulseId::Take => {
                let Some(StackItem::EntityId(object_id)) = cpu.stack.pop() else {
                    cpu.stack.push(StackItem::failure());

                    *made_world_query = true;
                    *state = AgentState::Running;
                    return;
                };
                let mut s = DefaultHasher::new();
                salt.hash(&mut s);
                "Take".hash(&mut s);
                agent_id.hash(&mut s);
                object_id.hash(&mut s);
                let prayer_id = s.finish();

                let request = TakeRequest {
                    agent_id,
                    prayer_id,
                    object_id,
                };

                commands.send_event(request);


                *made_world_query = true;
                *state = AgentState::WaitForAction(prayer_id);
            },
            ethnolib::sandbox::ai::InpulseId::Use => {
                if let Some(StackItem::EntityId(target_id)) = cpu.stack.pop() {
                    let mut s = DefaultHasher::new();
                    salt.hash(&mut s);
                    "Use".hash(&mut s);
                    agent_id.hash(&mut s);
                    target_id.hash(&mut s);
                    let prayer_id = s.finish();
                    let request = UseRequest {
                        agent_id,
                        prayer_id,
                        target_id,
                    };

                    commands.send_event(request);

                    *made_world_query = true;
                    *state = AgentState::WaitForAction(prayer_id);
                }
            }
            ethnolib::sandbox::ai::InpulseId::UseOn => todo!(),
            ethnolib::sandbox::ai::InpulseId::EatClass(_food_class) => {}
        },
        Status::None => (),
    }
}
