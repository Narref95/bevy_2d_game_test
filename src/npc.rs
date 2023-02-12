use bevy::prelude::*;
use bevy_spatial::{RTreeAccess3D, SpatialAccess, RTreePlugin3D};
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use queues::*;

use crate::{GameState, ImageAssets, components::Player, dialogue::{Dialogue, DialogueQueue}};

pub struct NPCPlugin;

#[derive(Component, Clone)]
pub struct NPC {
    pub name: String,
    pub image: Handle<Image>
}

impl NPC {
    pub fn default() -> Self {
        Self { name: String::default(), image: Handle::default() }
    }
}

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(RTreePlugin3D::<NPC> { ..default() })
        .add_system_set(SystemSet::on_enter(GameState::Ready)
            .with_system(npc_spawn_system)
        )
        .add_system_set(SystemSet::on_update(GameState::Ready)
            .with_system(check_nearest_npc)
        );
    }
}

fn npc_spawn_system(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams
) {
    commands.spawn(Sprite3d {
        image: images.enemy.clone(),
        transform: Transform::from_xyz(10.0, 2.0, 0.0).with_rotation(Quat::from_rotation_y(0.75)),
        pixels_per_metre: 50.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params))
    .insert(NPC {
        name: "Luffy".to_string(),
        image: images.luffy.clone()
    });

    commands.spawn(Sprite3d {
        image: images.enemy.clone(),
        transform: Transform::from_xyz(-10.0, 2.0, 0.0).with_rotation(Quat::from_rotation_y(0.75)),
        pixels_per_metre: 50.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params))
    .insert(NPC {
        name: "Luffy 2".to_string(),
        image: images.enemy.clone()
    });
}

type NNTree = RTreeAccess3D<NPC>;

fn check_nearest_npc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    kb: Res<Input<KeyCode>>,
    mut query_p: Query<(&mut Transform, &mut Player), With<Player>>,
    query_npcs: Query<&mut NPC, With<NPC>>,
    treeaccess: Res<NNTree>
) {
    if kb.pressed(KeyCode::Z) {
        if let Ok(mut transform) = query_p.get_single_mut() {
            if !transform.1.active {
                return;
            }
            for (_, entity) in treeaccess.within_distance(transform.0.translation, 4.0) {
                //Aqui solo entra cuando hay un NPC a menos de 10 de distancia, con esto se puede detectar para mostrar el dialogo del NPC
                if let Ok(npc) = query_npcs.get(entity) {
                    transform.1.active = false;
                    let dialogue = Dialogue {
                        text: "Prueba de dialogo 1".to_string(), 
                        title: npc.name.clone(), 
                        image: npc.image.clone() 
                    };
                    let mut vec = Vec::new();
                    vec.push(Dialogue {
                        text: "Prueba con un dialogo bastante mas largo que el anterior".to_string(),
                        title: npc.name.clone(),
                        image: npc.image.clone()
                    });
                    vec.push(Dialogue {
                        text: "%$#@!*()=+".to_string(),
                        title: npc.name.clone(),
                        image: npc.image.clone()
                    });
                    spawn_dialogue_with_queue!(commands, dialogue.clone(), DialogueQueue { dialogues: vec }, asset_server);
                }
            }
        }
    }
}
