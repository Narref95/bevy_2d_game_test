use bevy::prelude::*;
use bevy_sprite3d::*;
use crate::*;
use bevy_spatial::{RTreeAccess3D, RTreePlugin3D, SpatialAccess};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(RTreePlugin3D::<NPC> { ..default() })
        .add_system_set(SystemSet::on_enter(GameState::Ready)
            .with_system(player_spawn_system)
        )
        .add_system(player_movement_system)
        .add_system(player_keyboard_event_system)
        .add_system(camera_follow_player)
        .add_system(check_nearest_npc);
    }
}

type NNTree = RTreeAccess3D<NPC>;

fn check_nearest_npc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    kb: Res<Input<KeyCode>>,
    mut query_p: Query<(&mut Transform, &mut Player), With<Player>>,
    treeaccess: Res<NNTree>
) {
    if kb.pressed(KeyCode::Z) {
        if let Ok(mut transform) = query_p.get_single_mut() {
            if !transform.1.active {
                return;
            }
            for (_, entity) in treeaccess.within_distance(transform.0.translation, 4.0) {
                //Aqui solo entra cuando hay un NPC a menos de 10 de distancia, con esto se puede detectar para mostrar el dialogo del NPC
                transform.1.active = false;
                commands.spawn(NodeBundle {
                    style: Style {
                        border: UiRect::all(Val::Px(10.)),
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::FlexEnd,
                        align_content: AlignContent::Center,
                        display: Display::Flex,
                        flex_basis: Val::Px(1.0),
                        position: UiRect {bottom: Val::Px(10.), left: Val::Px(440.), ..default()},
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GRAY),
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Hola que tal como estas?",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 35.0,
                                color: Color::WHITE,
                            })
                        );
                }).insert(Dialogue);
            }
        }
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
        pixels_per_metre: 100.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params))
    .insert(Player {
        active: true
    })
    .insert(LookAtCamera)
    .insert(Velocity {x: 0., z: 0.});

    commands.spawn(Sprite3d {
        image: images.enemy.clone(),
        transform: Transform::from_xyz(10.0, 2.0, 0.0).with_rotation(Quat::from_rotation_y(0.75)),
        pixels_per_metre: 50.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params))
    .insert(LookAtCamera)
    .insert(NPC {
        text: "Hola que tal soy el chico de las poesias".to_string(),
        image: images.enemy.clone()
    });

    commands.spawn(Sprite3d {
        image: images.grass.clone(),
        transform: Transform::from_xyz(0.0, 0.5, -5.0).with_rotation(Quat::from_rotation_y(0.75)),
        pixels_per_metre: 200.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params));

    commands.spawn(Sprite3d {
        image: images.grass.clone(),
        transform: Transform::from_xyz(7.0, 0.5, 1.0).with_rotation(Quat::from_rotation_y(0.75)),
        pixels_per_metre: 200.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params));

    commands.spawn(Sprite3d {
        image: images.grass.clone(),
        transform: Transform::from_xyz(10.0, 0.5, 10.0).with_rotation(Quat::from_rotation_y(0.75)),
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