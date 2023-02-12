use std::time::Duration;

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
        .add_system_set( SystemSet::on_update(GameState::Ready).with_system(animate_sprite))
        .add_system(player_movement_system)
        .add_system(player_keyboard_event_system)
        .add_system(camera_follow_player);
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn player_spawn_system(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams 
) {
    commands.spawn(AtlasSprite3d {
        atlas: images.luffy_sheet.clone(),
        transform: Transform::from_xyz(0.0, 1.0, 0.0).with_rotation(Quat::from_rotation_y(0.75)),
        pixels_per_metre: 20.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params))
    .insert(Player {
        active: true
    })
    .insert(Velocity {x: 0., z: 0.})
    .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));

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
            base_velocity *= 1.3;
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

const NO_ANIMATION: [usize; 2] = [0,0];
const DOWN_RANGE: [usize; 2] = [0, 4];
const RIGHT_RANGE: [usize; 2] = [5, 9];
const LEFT_RANGE: [usize; 2] = [10, 14];
const UP_RANGE: [usize; 2] = [15, 19];

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&Player, &mut AnimationTimer, &mut AtlasSprite3dComponent)>,
    kb: Res<Input<KeyCode>>
) {
    for (player, mut timer, mut sprite) in query.iter_mut() {
        if !player.active {
            sprite.index = 0;
            return;
        }
        let mut range: [usize; 2] = NO_ANIMATION;
        if kb.pressed(KeyCode::Left) {
            range = LEFT_RANGE;
        } else if kb.pressed(KeyCode::Right) {
            range = RIGHT_RANGE;
        } else if kb.pressed(KeyCode::Down) {
            range = DOWN_RANGE;
        } else if kb.pressed(KeyCode::Up) {
            range = UP_RANGE;
        }
        if kb.pressed(KeyCode::LShift) {
            timer.0.set_duration(Duration::from_secs_f32(0.1));
        } else {
            timer.0.set_duration(Duration::from_secs_f32(0.125));
        }
        timer.tick(time.delta());
        if kb.any_pressed([KeyCode::Left, KeyCode::Right, KeyCode::Down, KeyCode::Up]) {
            if timer.just_finished() {
                sprite.index += 1;
                if sprite.index < range[0] || sprite.index > range[1] || sprite.index > sprite.atlas.len() - 1 {
                    sprite.index = range[0];
                }
            }
        } else {
            sprite.index = 0;
        }
    }
}