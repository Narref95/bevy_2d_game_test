use bevy::prelude::*;
use bevy_sprite3d::*;
use crate::{*};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Ready)
            .with_system(player_spawn_system)
        )
        .add_system(player_movement_system)
        .add_system(player_keyboard_event_system)
        .add_system(camera_follow_player);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams 
) {
    commands.spawn(Sprite3d {
        image: images.player.clone(),
        transform: Transform::from_xyz(0.0, 1.0, 0.0).with_rotation(Quat::from_rotation_y(0.75)),
        pixels_per_metre: 125.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params))
    .insert(Player {
        active: true
    })
    .insert(Velocity {x: 0., z: 0.});

    let grass_positions = [
        Transform::from_xyz(0.0, 0.5, -5.0),
        Transform::from_xyz(7.0, 0.5, 1.0),
        Transform::from_xyz(10.0, 0.5, 10.0),
        Transform::from_xyz(-7.0, 0.5, -1.0)
    ];

    for position in grass_positions {
        commands.spawn(Sprite3d {
            image: images.grass.clone(),
            transform: position.with_rotation(Quat::from_rotation_y(0.75)),
            pixels_per_metre: 200.,
            partial_alpha: true,
            unlit: true,
            ..default()
        }.bundle(&mut sprite_params));
    }

    commands.spawn(Sprite3d {
        image: images.grass.clone(),
        transform: Transform::from_xyz(0.0, 0.5, -5.0).with_rotation(Quat::from_rotation_y(0.75)),
        pixels_per_metre: 200.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params));
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = 0.;
        velocity.z = 0.;

        let mut base_velocity: f32 = 1.;

        if kb.pressed(KeyCode::LShift) {
            base_velocity *= 2.0;
        }

        if kb.pressed(KeyCode::Left) {
            velocity.x += base_velocity;
            velocity.z += -base_velocity;
        } 
        if kb.pressed(KeyCode::Right) {
            velocity.x += -base_velocity;
            velocity.z += base_velocity;
        } 
        if kb.pressed(KeyCode::Up) {
            velocity.x += base_velocity;
            velocity.z += base_velocity;
        } 
        if kb.pressed(KeyCode::Down) {
            velocity.z += -base_velocity;
            velocity.x += -base_velocity;
        }
    }
}

fn player_movement_system(
    mut query: Query<(&Velocity, &mut Transform, &mut Player), With<Player>>
) {
    for (velocity, mut transform, player) in query.iter_mut() {
        if !player.active {
            return;
        }
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.z += velocity.z * TIME_STEP * BASE_SPEED;
    }
}

fn camera_follow_player(
    mut query_p: Query<&mut Transform, With<Player>>,
    mut query_c: Query<&mut Transform, (With<MainCamera>, Without<Player>)>
) {
    if let Ok(player_transform) = query_p.get_single_mut() {
        if let Ok(mut camera_transform) = query_c.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x - 10.;
            camera_transform.translation.z = player_transform.translation.z - 10.;
        }
    }
}