use bevy::prelude::Component;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub z: f32
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct LookAtCamera;
