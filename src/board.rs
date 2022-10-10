use bevy::prelude::*;
use bevy_mod_picking::{HoverEvent, PickableBundle, PickingEvent};

use crate::pieces::{Piece, PieceMoveEvent};

struct SquaresRenderData {
    hovered_color: Handle<StandardMaterial>,
    selected_color: Handle<StandardMaterial>,
    //valid_movement_color: Handle<StandardMaterial>,
    black_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
    background_color: Handle<StandardMaterial>,
}

impl FromWorld for SquaresRenderData {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        Self {
            hovered_color: materials.add(Color::rgb(0.6, 0.3, 0.3).into()),
            selected_color: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
            //valid_movement_color: materials.add(Color::rgb(0.3, 0.8, 0.3).into()),
            black_color: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            white_color: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            background_color: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        }
    }
}

#[derive(Clone, Copy)]
enum SquareColor {
    White,
    Black,
}

#[derive(Clone, Component, Copy)]
struct Square(SquareColor);

// (0, 0) is A1, (0, 7) is A8
#[derive(Clone, Component, Copy, Eq, PartialEq)]
pub struct BoardPosition {
    pub row: u8,
    pub col: u8,
}

impl BoardPosition {
    fn square_color(&self) -> SquareColor {
        if (self.row + self.col) % 2 == 0 {
            SquareColor::Black
        } else {
            SquareColor::White
        }
    }

    pub fn to_translation(self) -> Vec3 {
        let x = self.col as f32 - 3.5;
        let y = 0.25;
        let z = -(self.row as f32 - 3.5);
        Vec3::new(x, y, z)
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<SquaresRenderData>,
) {
    // Every square on the board is the same shape - a square with some depth
    let square_mesh = meshes.add(Mesh::from(shape::Box {
        min_x: -0.5,
        max_x: 0.5,
        min_y: 0.0,
        max_y: 0.25,
        min_z: -0.5,
        max_z: 0.5,
    }));

    for row in 0..8 {
        for col in 0..8 {
            let pos = BoardPosition { row, col };
            let sq = Square(pos.square_color());
            let material = match sq.0 {
                SquareColor::White => materials.white_color.clone(), // TODO: do we need to clone here? Creating too many handles?
                SquareColor::Black => materials.black_color.clone(),
            };
            commands
                .spawn_bundle(PbrBundle {
                    mesh: square_mesh.clone(),
                    material,
                    transform: Transform::from_translation(pos.to_translation()),
                    ..default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(sq)
                .insert(pos);
        }
    }

    // Create a back plane entity. This is needed to allow clicking on something that's not the board.
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.background_color.clone(),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
}

fn render_board(
    hovered_square: Res<HoveredSquare>,
    selected_piece: Res<SelectedPiece>,
    materials: Res<SquaresRenderData>,
    mut square_query: Query<(
        Entity,
        &Square,
        &BoardPosition,
        &mut Handle<StandardMaterial>,
    )>,
    board_pos_query: Query<&BoardPosition>,
) {
    let piece_pos = if let Some(piece_ent) = selected_piece.entity {
        let board_pos = board_pos_query.get(piece_ent).unwrap();
        Some(board_pos)
    } else {
        None
    };

    for (entity, square, pos, mut material) in &mut square_query {
        if Some(pos) == piece_pos {
            *material = materials.selected_color.clone();
        } else if Some(entity) == hovered_square.entity {
            *material = materials.hovered_color.clone();
        } else {
            match square.0 {
                SquareColor::White => *material = materials.white_color.clone(), // TODO: don't clone materials?
                SquareColor::Black => *material = materials.black_color.clone(),
            }
        }
    }
}

#[derive(Default)]
struct HoveredSquare {
    entity: Option<Entity>,
}

struct ClickSquareEvent {
    kind: MouseButton,
    entity: Option<Entity>,
}

fn click_square(
    mut pick_events: EventReader<PickingEvent>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    squares_query: Query<&Square>,
    mut hovered_square: ResMut<HoveredSquare>,
    mut click_square_events: EventWriter<ClickSquareEvent>,
) {
    for event in pick_events.iter() {
        match event {
            PickingEvent::Hover(HoverEvent::JustEntered(e)) => {
                if squares_query.get(*e).is_ok() {
                    hovered_square.entity = Some(*e);
                } else {
                    hovered_square.entity = None;
                }
            }
            PickingEvent::Hover(HoverEvent::JustLeft(e)) => {
                if let Some(prev_ent) = hovered_square.entity {
                    if *e == prev_ent {
                        hovered_square.entity = None;
                    }
                }
            }
            _ => (),
        }
    }

    if mouse_button_inputs.just_pressed(MouseButton::Left) {
        click_square_events.send(ClickSquareEvent {
            kind: MouseButton::Left,
            entity: hovered_square.entity,
        });
    }
    if mouse_button_inputs.just_pressed(MouseButton::Right) {
        click_square_events.send(ClickSquareEvent {
            kind: MouseButton::Right,
            entity: hovered_square.entity,
        });
    }
    if mouse_button_inputs.just_pressed(MouseButton::Middle) {
        click_square_events.send(ClickSquareEvent {
            kind: MouseButton::Middle,
            entity: hovered_square.entity,
        });
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

fn select_square(
    mut events: EventReader<ClickSquareEvent>,
    mut selected_square: ResMut<SelectedSquare>,
) {
    for event in events.iter() {
        if event.kind == MouseButton::Left {
            selected_square.entity = event.entity;
        }
    }
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

fn select_piece(
    selected_square: Res<SelectedSquare>,
    board_pos_query: Query<&BoardPosition>,
    piece_query: Query<(Entity, &Piece, &BoardPosition)>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut events: EventWriter<PieceMoveEvent>,
) {
    // Only do this stuff if a new square is selected
    if !selected_square.is_changed() {
        return;
    }

    if let Some(sq_ent) = selected_square.entity {
        // First find which position was selected
        let select_pos = *board_pos_query.get(sq_ent).unwrap();
        if selected_piece.entity.is_none() {
            // Use the selected piece (if any) only if we didn't already have a piece selected
            let piece_at_selection = piece_query.iter().find_map(|(entity, _piece, piece_pos)| {
                if *piece_pos == select_pos {
                    Some(entity)
                } else {
                    None
                }
            });
            selected_piece.entity = piece_at_selection;
        } else {
            // If we already had a piece selected then move it to the newly selected position
            events.send(PieceMoveEvent::new(
                selected_piece.entity.unwrap(),
                select_pos,
            ));
            selected_piece.entity = None;
        }
    } else {
        // Selected something that's not a square, remove the piece selection
        selected_piece.entity = None;
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_board)
            .add_system(render_board)
            .init_resource::<SquaresRenderData>()
            .add_system(click_square)
            .init_resource::<HoveredSquare>()
            .add_event::<ClickSquareEvent>()
            .add_system(select_square)
            .init_resource::<SelectedSquare>()
            .add_system(select_piece)
            .init_resource::<SelectedPiece>();
    }
}
