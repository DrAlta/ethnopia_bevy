//! AI sends an event to the world_polling system and sets the agents AI to counter of how many ticks it's waited to 0
//! world_polling system recaise the event from the AI syste mt pull the world and does the pulls then send the result back to the AI sysye m in a event
//! The AI first process the events and for pull_resulr events and continues proccess the AI [HA note:pull instustions finish processing on the same tick as the next instruction is run]
//!
//! the AI system process the agents and if a agent is is waitng for world_pull result then it's counter is incremented then a check ot see if it's been to long and the instruction should fail.
//!  if the agent isn't waiting it ticks the agent's AI
use bevy::{
    color::palettes::css::{BLUE, BROWN, GRAY, GREEN, ORANGE, RED, STEEL_BLUE, TAN, YELLOW},
    prelude::*,
};
use ethnolib::{
    Number,
    sandbox::{Item, movement::Collision},
};

use crate::systems::{
    actions::{
        ActionResult, GotoRequest, UseOnRequest, UseRequest, goto_system, use_object_system,
    },
    change_request::{
        ChangeConflict, ChangeDespawn, ChangeEnergy, ChangeHp, ChangeLocation, ChangeRequest,
        ChangeSpawnLocationType, change_request_system,
    },
    movement::{TravelCompleted, process_movement},
    posible_actions::{PosibleActionsRequest, PosibleActionsResponce},
};

mod dev_console;
use dev_console::collision_report_system;
mod game_state;
pub use game_state::GameState;
mod setup;
use setup::setup;
pub mod systems;
use systems::{
    SaltShaker,
    actions::{EatClassRequest, TakeRequest, eat_class_system, use_on_system},
    agent_system, cache_inventory_system,
    change_request::change_systems::{
        despawn_request_system, energy_request_system, hp_request_system, location_request_system,
        spawn_location_type_request_system,
    },
    query::{
        FindInInventoryRequest, FindInInventoryResult, FindNearestRequest, FindNearestResult,
        GetEnergyRequest, GetEnergyResult, GetEntitiesRequest, GetEntitiesResult, GetHpRequest,
        GetHpResult, GetIsInventoryGERequest, GetIsInventoryGEResult, GetLocationRequest,
        GetLocationResult, RemoveEntitiesOfClassRequest, RemoveEntitiesOfClassResult,
        RetainEntitiesOfClassRequest, RetainEntitiesOfClassResult, find_in_inventory_system,
        find_nearest_system, get_energy_system, get_entities_system, get_hp_system,
        get_is_inventory_ge_system, get_location_system,
    },
    receive_prayers_system, salt_system,
};
mod pawn_spawn;
use pawn_spawn::pawn_spawn;
//mod picking;
//use picking::{cursor_hovering_system, CursorOnPawn, Picky};
mod ui;
pub use ui::menu;
use ui::{hover_out_system, picking_system};

const CELL_SIZE: Number = Number::new(30, 1);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(systems::BVH(None))
        .insert_resource(SaltShaker(0))
        .insert_resource(ui::UIState {
            selected_entity: None,
            mode: ui::Mode::Pan,
            actions: Vec::new(),
        })
        .add_systems(Startup, setup)
        // movement
        .add_event::<TravelCompleted>()
        .add_event::<Collision>()
        // actions
        .add_event::<EatClassRequest>()
        .add_event::<GotoRequest>()
        .add_event::<TakeRequest>()
        .add_event::<UseRequest>()
        .add_event::<UseOnRequest>()
        // action results
        .add_event::<ActionResult>()
        // posible actions
        .add_event::<PosibleActionsRequest>()
        .add_event::<PosibleActionsResponce>()
        // changes
        .add_event::<ChangeRequest>()
        .add_event::<ChangeConflict>()
        .add_event::<ChangeDespawn>()
        .add_event::<ChangeEnergy>()
        .add_event::<ChangeHp>()
        .add_event::<ChangeLocation>()
        .add_event::<ChangeSpawnLocationType>()
        // prayers
        .add_event::<FindInInventoryRequest>()
        .add_event::<FindInInventoryResult>()
        .add_event::<FindNearestRequest>()
        .add_event::<FindNearestResult>()
        .add_event::<GetEnergyRequest>()
        .add_event::<GetEnergyResult>()
        .add_event::<GetEntitiesRequest>()
        .add_event::<GetEntitiesResult>()
        .add_event::<GetHpRequest>()
        .add_event::<GetHpResult>()
        .add_event::<GetIsInventoryGERequest>()
        .add_event::<GetIsInventoryGEResult>()
        .add_event::<GetLocationRequest>()
        .add_event::<GetLocationResult>()
        .add_event::<RemoveEntitiesOfClassRequest>()
        .add_event::<RemoveEntitiesOfClassResult>()
        .add_event::<RetainEntitiesOfClassRequest>()
        .add_event::<RetainEntitiesOfClassResult>()
        // ui
        .add_event::<ui::UISelect>()
        .add_systems(
            Update,
            (
                salt_system,
                // do the world simulation
                (
                    // movement systems
                    process_movement,
                    collision_report_system,
                    (
                        // action systems
                        eat_class_system,
                        use_object_system,
                        use_on_system,
                    ),
                    // resolve the changes that actions wanted to have on the world
                    change_request_system,
                    (
                        despawn_request_system,
                        energy_request_system,
                        hp_request_system,
                        location_request_system,
                        spawn_location_type_request_system,
                    ),
                )
                    .chain()
                    .run_if(in_state(GameState::RunningSimulation)),
                // world sim finished
                // build caches
                (cache_inventory_system, systems::bvh_system)
                    .run_if(in_state(GameState::RunningSimulation)),
                // 'answer divination prayers' and 'run AI' should loop until 'run AI' stops generating new divination prayers(or some time out)
                (
                    // answer divination prayers
                    find_in_inventory_system,
                    find_nearest_system,
                    get_energy_system,
                    get_entities_system,
                    get_hp_system,
                    get_is_inventory_ge_system,
                    get_location_system,
                ),
                // run A
                (receive_prayers_system, agent_system).chain(),
                // answer supplication prayers
                (goto_system).run_if(in_state(GameState::RunningSimulation)),
                // UI
                (pawn_spawn, picking_system, hover_out_system, ui::ui_system).chain(),
            )
                .chain(),
        )
        /*
        .add_systems(
            Update,
            (
                salt_system,
                receive_prayers_system
                agent_system,
            )
                .chain(),
        )*/
        .run();
}

pub fn type_to_color(tyep: &Item) -> Color {
    const DARKGRAY: Srgba = Srgba::rgb(0.31, 0.31, 0.31);
    const DARKBROWN: Srgba = Srgba::rgb(0.3, 0.25, 0.18);
    Color::Srgba(match tyep {
        Item::Agent => BLUE,
        Item::Axe => GRAY,
        Item::Food => YELLOW,
        Item::Stone => DARKGRAY,
        Item::Stick => BROWN,
        Item::Wood => DARKBROWN,
        Item::House => RED,
        Item::Tree => GREEN,
        Item::Veggie => ORANGE,
        Item::Knife => STEEL_BLUE,
        Item::Seed => TAN,
    })
}
