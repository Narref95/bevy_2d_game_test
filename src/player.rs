use bevy::prelude::*;
use bevy_sprite3d::*;
use crate::*;

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
    transform: Transform::from_xyz(0.0, 1.0, 0.0),
        pixels_per_metre: 100.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params))
    .insert(Player)
    .insert(LookAtCamera)
    .insert(Velocity {x: 0., z: 0.});
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = 0.;
        velocity.z = 0.;
        if kb.pressed(KeyCode::Left) {
            velocity.x += 1.;
            velocity.z += -1.;
        } 
        if kb.pressed(KeyCode::Right) {
            velocity.x += -1.;
            velocity.z += 1.;
        } 
        if kb.pressed(KeyCode::Up) {
            velocity.x += 1.;
            velocity.z += 1.;
        } 
        if kb.pressed(KeyCode::Down) {
            velocity.z += -1.;
            velocity.x += -1.;
        }
    }
}

fn player_movement_system(
    mut query: Query<(&Velocity, &mut Transform), With<Player>>
) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.z += velocity.z * TIME_STEP * BASE_SPEED;
        //transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.,45.,0.);
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