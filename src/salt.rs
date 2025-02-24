use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct Salt(pub u64);

pub fn salt_system(mut salt: ResMut<Salt>) {
    salt.0 += 1;
   // assert_ne!(salt.0, 15)
//    println!("----salt {}", salt.0);
}
