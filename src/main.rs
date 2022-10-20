mod board;
mod game;
mod pieces;
mod ui;

use crate::{board::BoardPlugin, game::GamePlugin, pieces::PiecesPlugin, ui::UiPlugin};
use bevy::prelude::*;
use bevy_mod_picking::{InteractablePickingPlugin, PickingCameraBundle, PickingPlugin};

fn main() {
    App::new()
        //.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Schach!".to_string(),
            width: 1200.0,
            height: 800.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(UiPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 12.0, 8.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert_bundle(PickingCameraBundle::default());

    // Light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(2.0, 10.0, 2.0)),
        ..default()
    });
}
