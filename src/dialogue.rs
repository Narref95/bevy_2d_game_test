use bevy::{prelude::*};
use queues::*;

use crate::{components::Player};

pub struct DialoguePlugin;

#[derive(Component, Clone, Default)]
pub struct Dialogue {
    pub text: String,
    pub title: String,
    pub image: Handle<Image>
}

impl Dialogue {
    pub fn default() -> Self {
        Self {
            text: String::default(),
            title: String::default(),
            image: Handle::default()
        }
    }
}

#[derive(Component, Default)]
pub struct DialogueQueue {
    pub dialogues: Vec<Dialogue>
}

#[derive(Component)]
pub struct DialogueTest {
    pub dialogues: Queue<Dialogue>
}

macro_rules! spawn_dialogue_with_queue {
    ($commands: expr, $dialogue: expr, $dialogue_queue: expr, $asset_server: expr) => {
        {
            $commands.spawn(NodeBundle {
                background_color: BackgroundColor(Color::GRAY),
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    align_content: AlignContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(10.)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Percent(1.0),
                        ..default()
                    },
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage($dialogue.image.clone()),
                    style: Style {
                        size: Size {height:Val::Px(150.0), width:Val::Px(150.0)},
                        ..default()
                    },
                    ..default()
                });
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Baseline,
                        border: UiRect::all(Val::Px(10.)),
                        size: Size {
                            height: Val::Px(150.0),
                            width: Val::Px(500.0)
                        },
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            $dialogue.title + ":",
                        TextStyle {
                            font: $asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE
                        })
                    );
                    parent.spawn(
                        TextBundle::from_section(
                        $dialogue.text,
                        TextStyle {
                            font: $asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 25.0,
                            color: Color::WHITE,
                        })
                    );
                });
            })
            .insert($dialogue)
            .insert($dialogue_queue);
        }
    }
}

// use macro_rules! <name of macro>{<Body>}
macro_rules! spawn_dialogue {
    // macth like arm for macro
       ($commands: expr, $dialogue: expr, $asset_server: expr) => {
    // macro expand to this code
           {
                spawn_dialogue_with_queue!($commands, $dialogue, DialogueQueue { dialogues: Queue::default() }, $asset_server);
           }
       }
   }

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_dialogue);
    }
}

impl DialoguePlugin {
    pub fn default() -> Self {
        Self
    }
}

fn despawn_dialogue(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    dialogue_query: Query<(Entity, &DialogueQueue), With<Dialogue>>,
    asset_server: Res<AssetServer>
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if kb.just_released(KeyCode::Space) {
            let mut activate_player = true;
            for (dialogue, queue) in dialogue_query.iter() {
                dbg!(dialogue);
                commands.entity(dialogue).despawn_recursive();
                if !queue.dialogues.is_empty() {
                    let mut remaining_dialogues = queue.dialogues.clone();
                    let dialogue = remaining_dialogues.pop().unwrap();
                    spawn_dialogue_with_queue!(commands, dialogue.clone(), DialogueQueue { dialogues: remaining_dialogues }, asset_server);
                    activate_player = false;
                }
            }
            player.active = activate_player;
        }
    }
}