use bevy::{
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, camera::ScalingMode},
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin}
    };
use bevy_sprite3d::Sprite3dPlugin;
use components::*;
use npc::NPCPlugin;
use player::PlayerPlugin;
use dialogue::DialoguePlugin;
use bevy_editor_pls::prelude::*;
use bevy_asset_loader::prelude::*;

pub const HEIGHT: f32 = 720.0;
pub const RATIO: f32 = 16. / 9.;

const TIME_STEP: f32 = 1. / 120.;
const BASE_SPEED: f32 = 10.;

#[macro_use]
mod dialogue;
mod components;
mod player;
mod npc;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GameState { Loading, Ready }

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(path = "player.png")]
    player: Handle<Image>,
    #[asset(path = "enemy.png")]
    enemy: Handle<Image>,
    #[asset(path = "grass.png")]
    grass: Handle<Image>,
    #[asset(path = "luffy.png")]
    luffy: Handle<Image>,
}

fn main() {
    let wpu_settings: WgpuSettings = WgpuSettings {
        features: WgpuFeatures::POLYGON_MODE_LINE,
        ..default()
    };
    let window_plugin: WindowPlugin = WindowPlugin {
        window: WindowDescriptor {
            width: HEIGHT * RATIO,
            height: HEIGHT,
            title: "Turn Based Game".to_string(),
            resizable: true,
            ..default()
        },
        ..default()
    };
    let color_settings: bevy::prelude::ClearColor = ClearColor(Color::rgb(0.2, 0.2, 0.2));
    App::new()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Ready)
                .with_collection::<ImageAssets>()
        )
        .add_state(GameState::Loading)
        .insert_resource(wpu_settings)
        .insert_resource(color_settings)
        .insert_resource(AmbientLight {
            color: Color::rgb(0.5, 0.5, 0.5),
            brightness: 5.
        })
        .add_plugins(DefaultPlugins.set(window_plugin))
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(Sprite3dPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(NPCPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Ready).with_system(setup))
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(DialoguePlugin::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // floor
    for x in -20..20 {
        for z in -20..20 {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(x as f32 * 0.05, 0.5, 0.3).into()),
                transform: Transform::from_xyz(x as f32, -0.5, z as f32),
                ..default()
            }).insert(Ground);
        }
    }

    // wall with door
    for y in 0..7 {
        for z in -20..20 {
            if !(z >= -3 && z <= 3 && y >= 1 && y <= 5) {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.0, 1.0, 0.3).into()),
                    transform: Transform::from_xyz(20.0, y as f32 - 0.5, z as f32),
                    ..default()
                });
            }
        }
    }
    
    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 10.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            far: 150.,
            near: -150.,
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(-10.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(MainCamera);
}