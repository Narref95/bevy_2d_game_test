use bevy::prelude::{Component, Handle, Image};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub z: f32
}
#[derive(Component)]
pub struct Player {
    pub active: bool
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct LookAtCamera;

#[derive(Component)]
pub struct NPC {
    pub text: String,
    pub image: Handle<Image>
}

#[derive(Component)]
pub struct Dialogue;