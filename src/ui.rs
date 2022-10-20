use bevy::prelude::*;

use crate::game::{GameOver, GameState};

#[derive(Component)]
struct GameStateText;

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    // This is absolute garbage but I can't figure out why
                    left: Val::Percent(25.0),
                    right: Val::Percent(25.0),
                    top: Val::Percent(0.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(GameStateText);
}

fn update_ui(game_state: Res<GameState>, mut query: Query<&mut Text, With<GameStateText>>) {
    if !game_state.is_changed() {
        return;
    }

    let mut text = query.get_single_mut().unwrap();
    let value = match game_state.game_over {
        Some(GameOver::Checkmate(winner)) => format!("CHECKMATE!\n{} wins!", winner),
        Some(GameOver::Stalemate) => String::from("STALEMATE"),
        None => format!("{} to move", game_state.curr_player),
    };
    text.sections[0].value = value;
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(update_ui);
    }
}
