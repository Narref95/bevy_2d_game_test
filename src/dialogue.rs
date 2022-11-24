use bevy::{prelude::*};

use crate::components::{Dialogue, Player};
pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_dialogue);
    }
}

fn despawn_dialogue(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    dialogue_query: Query<Entity, With<Dialogue>>
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if kb.any_pressed([KeyCode::Space]) {
            for ent in dialogue_query.iter() {
                commands.entity(ent).despawn_recursive();
            }
            player.active = true;
        }
    }
}