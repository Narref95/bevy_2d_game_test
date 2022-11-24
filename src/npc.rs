pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Ready)
            .with_system(npc_spawn_system)
        );
    }
}

fn npc_spawn_system() {
    commands.spawn(Sprite3d {
        image: images.player.clone(),
    transform: Transform::from_xyz(0.0, 1.0, 0.0),
        pixels_per_metre: 100.,
        partial_alpha: true,
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params));
}