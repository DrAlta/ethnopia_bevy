use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::view::{RenderVisibleEntities, VisibleEntities};
use ethnolib::sandbox::world::Size;
use qol::logy;

use crate::pawn_spawn::Pawn;

#[derive(Debug, Event)]
pub struct CursorOnPawn{
    pub entity_id: Entity,
    pub pawn_id: Entity,
}

#[derive(Debug, Component)]
pub struct Picky(pub Entity);

fn get_cursor_xy(
    window: &Window,
    camera:&Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let cursor_pos = window.cursor_position()?;

    camera.viewport_to_world_2d(camera_transform, cursor_pos).ok()
}

pub fn cursor_hovering_system(
    window_query: Query<(&Window, &Picky)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    sprite_query: Query<(Entity, &Transform)>,
    size_query: Query<&Size>,
    pawn_query: Query<(Entity, &Pawn)>,
    mut text_query: Query<&mut Text2d>,
    mut cursor_on_event_writer: EventWriter<CursorOnPawn>,
){
    let Ok(mut text) = text_query.get_single_mut() else {
        logy!("debug", "no text found");
        return;
    };
        
    let mut hoving_pawn: Option<Entity> = None;
    let mut hoving_pawn_z = -f32::INFINITY;
    let who_for: HashMap<Entity, Entity> = pawn_query.iter().map(|(entity, pawn)|(pawn.0.clone(), entity)).collect();

    for (window, picky) in window_query.iter() {

        let Ok(camera) = camera_query.get(picky.0) else {
            continue;
        };
        if let Some(cursor_xy) = get_cursor_xy(window, &camera.0, &camera.1){

            for (pawn_id, transform) in sprite_query.iter(){
                let Some(entity_id) = who_for.get(&pawn_id).cloned() else {
                    //logy!("error", "no main entity for this pawn");
                    continue
                };
                let Ok(border) = size_query.get(entity_id) else {
                    logy!("error", "entity has no size");
                    continue;
                };
                //logy!("debug", "doing entity");
                let global_translation = transform.translation;
                if global_translation.z < hoving_pawn_z {continue}
                let half_width = border.width as f32 * 0.5;
                let half_height = border.height as f32 * 0.5;
                if
                    cursor_xy.x >= global_translation.x - half_width &&
                    cursor_xy.x <= global_translation.x + half_width &&
                    cursor_xy.y >= global_translation.y - half_height &&
                    cursor_xy.y <= global_translation.y + half_height
                {
                    hoving_pawn = Some(pawn_id);
                    hoving_pawn_z = global_translation.z;
                }
            }
        }
    }
    logy!("error", "{hoving_pawn:?}");

    if let Some(pawn_id) = hoving_pawn { 
        let Some(&entity_id) = who_for.get(&pawn_id) else {
            text.0 = format!("hoving over something?");

            return
        };
        text.0 = format!("{entity_id}");
        cursor_on_event_writer.send(CursorOnPawn{entity_id, pawn_id});
    } else {
        text.0 = format!("not hoving over anything");

    }
}