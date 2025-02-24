use std::collections::HashMap;

use bevy::prelude::*;

use ethnolib::sandbox::world::Type;
use qol::logy;

use crate::pawn_spawn::Pawn;

pub fn picking_system(
    mut query: Query<&mut Sprite>,
    mut clicks: EventReader<Pointer<Click>>,
    mut hovers: EventReader<Pointer<Over>>,
){
    for event in clicks.read(){
        logy!("debug", "{}", event.target);
    }
    for event in hovers.read(){
        let Ok(mut sprite) = query.get_mut(event.target) else {
            continue
        };
        sprite.color = sprite.color.lighter(5.0);
    }
}

pub fn hover_out_system(
    mut query: Query<&mut Sprite>,
    item_query: Query<(Entity, &Pawn, &Type)>,
    mut hovers: EventReader<Pointer<Out>>,
){
    let who_for: HashMap<Entity, Entity> = item_query.iter().map(|(entity, pawn, _)|(pawn.0.clone(), entity)).collect();

    for event in hovers.read(){
        let Ok(mut sprite) = query.get_mut(event.target) else {
            continue
        };
        let Some(&main_id) = who_for.get(&event.target) else {
            continue;
        };
        let Ok((_, _, Type(tyep))) = item_query.get(main_id) else {
            continue;
        };
        sprite.color = crate::type_to_color(tyep);
    }
}
