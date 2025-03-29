use std::collections::HashMap;

use bevy::prelude::*;

use ethnolib::sandbox::{actions::{PosibleActionsRequest, UseRequest}, world::Type};
use qol::logy;

use crate::{pawn_spawn::Pawn, ui::{Mode, UISelect, UIState}};

pub fn picking_system(
    mut query: Query<&mut Sprite>,
    item_query: Query<(Entity, &Pawn)>,
    mut clicks: EventReader<Pointer<Click>>,
    mut hovers: EventReader<Pointer<Over>>,
    mut use_event: EventWriter<UseRequest>,
    mut actions_request_event: EventWriter<PosibleActionsRequest>,
    mut select_event: EventWriter<UISelect>,
    ui_state: Res<UIState>,
){
    //println!("pinking_system");
    let who_for: HashMap<Entity, Entity> = item_query.iter().map(|(entity, pawn)|(pawn.0.clone(), entity)).collect();

    let mut select = None;
    for event in clicks.read(){
        logy!("debug", "{}", event.target);
        if let Some(thing) = who_for.get(&event.target) {
            let x = ui_state.as_ref();
            match x {
                UIState { selected_entity: Some(agent_id), mode: Mode::Use, .. } => {
                    use_event.send(UseRequest { agent_id: *agent_id, target_id: *thing });
                },
                UIState { selected_entity: _, mode: Mode::Pan, ..} => {
                    select = Some(*thing);
                },
                _  => {}
            };
        }
    }
    if let Some(thing) = select {
        select_event.send(UISelect(thing));
    };
    let mut hov = None;
    for event in hovers.read(){
        if let Some(thing) = who_for.get(&event.target) {
            hov = Some(*thing);
        }
        let Ok(mut sprite) = query.get_mut(event.target) else {
            continue
        };
        sprite.color = sprite.color.lighter(0.25);
    }
    if let (Some(agent_id), Some(target_id)) = (ui_state.selected_entity, hov) {
        logy!("debug", "sending request for action on {target_id}");
        actions_request_event.send(PosibleActionsRequest{ agent_id, target_id });
    }
}

pub fn hover_out_system(
    mut query: Query<&mut Sprite>,
    item_query: Query<(Entity, &Pawn, &Type)>,
    mut hovers: EventReader<Pointer<Out>>,
    ui_state: Res<UIState>,
){
    //println!("hover_out_system");
    let who_for: HashMap<Entity, Entity> = item_query.iter().map(|(entity, pawn, _)|(pawn.0.clone(), entity)).collect();


    for event in hovers.read(){
        let Ok(mut sprite) = query.get_mut(event.target) else {
            continue
        };
        let Some(&main_id) = who_for.get(&event.target) else {
            continue;
        };
        if Some(main_id) == ui_state.selected_entity {
            continue;
        }
        let Ok((_, _, Type(tyep))) = item_query.get(main_id) else {
            continue;
        };
        sprite.color = crate::type_to_color(tyep);
    }
}
