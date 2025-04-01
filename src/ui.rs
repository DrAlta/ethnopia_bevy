use bevy::{color::palettes::css::PINK, picking::focus::HoverMap, prelude::*};
use ethnolib::sandbox::actions::PosibleActionsResponce;
use qol::logy;

use crate::pawn_spawn::Pawn;
#[allow(dead_code)]
#[derive(Debug)]
pub enum Mode {
    Pan,
    Move,
    Use,
}

#[derive(Debug, Resource)]
pub struct UIState {
    pub selected_entity: Option<Entity>,
    pub mode: Mode,
    pub actions: Vec<String>,
    //pub action_hash: u64,
}

#[derive(Debug, Event)]

pub struct UISelect(pub Entity);

pub fn ui_system(
    mut sprit_query: Query<&mut Sprite>,
    item_query: Query<&Pawn>,
    mut select_event: EventReader<UISelect>,
    mut actions_event: EventReader<PosibleActionsResponce>,
    mut ui_state: ResMut<UIState>,
    hover_map: Res<HoverMap>,
    mut text_query: Query<&mut Text2d>,
) {
    let mut select = None;
    for x in select_event.read() {
        select = Some(x.0)
    }

    let Some(thing) = select else {
        let mut text = text_query.single_mut();
        let x = hover_map.iter().next().unwrap().1.keys();
        println!("updating text");
        text.0 = format!("{:?}{}", x, match ui_state.mode {
            Mode::Pan => "P",
            Mode::Move => "M",
            Mode::Use => "U",
        },);
        return;
    };

    let Ok(Pawn(pawn_id)) = item_query.get(thing) else {
        logy!("debug", "coudn fins pawn id");
        return;
    };
    let Ok(mut pawn) = sprit_query.get_mut(*pawn_id) else {
        logy!("debug", "couldn't fins pawn");
        return;
    };
    ui_state.selected_entity = Some(thing);
    pawn.color = PINK.into();
    let mut collect_actions = Vec::new();
    for PosibleActionsResponce {
        agent_id: _,
        target_id: _,
        action_id,
    } in actions_event.read()
    {
        logy!("debug", "adding action{action_id:?}");
        collect_actions.push(format!("{action_id:?}"))
    }
    if !collect_actions.is_empty() {
        ui_state.actions = collect_actions
    }
    let mut text = text_query.single_mut();
    let x = hover_map.iter().next().unwrap().1.keys();
    println!("updating text");
    text.0 = format!(
        "{:?}{}{}\n{:#?}",
        x,
        match ui_state.mode {
            Mode::Pan => "P",
            Mode::Move => "M",
            Mode::Use => "U",
        },
        thing,
        ui_state.actions
    );
}
