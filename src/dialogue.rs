use bevy::prelude::*;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_ui);
    }
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    //commands.spawn()
    commands.spawn(
        TextBundle::from_section(
            "Hola que tal como estas?",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            display: Display::Flex,
            position: UiRect {bottom: Val::Px(0.), left: Val::Px(440.), ..default()},
            justify_content: JustifyContent::Center,
            ..default()
        }),
    );
}