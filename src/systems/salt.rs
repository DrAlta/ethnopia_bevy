use bevy::prelude::*;

pub type Salt = u64;

#[derive(Debug, Resource)]
pub struct SaltShaker(pub Salt);

pub fn salt_system(mut salt: ResMut<SaltShaker>) {
    salt.0 += 1;
    // assert_ne!(salt.0, 15)
    //    println!("----salt {}", salt.0);
}
