use std::ops::{Add, AddAssign};

use bevy::prelude::*;
use bevy_mod_picking::{HoverEvent, PickableBundle, PickingEvent};

use crate::{
    game::{TurnData, ValidMove},
    pieces::PieceMoveEvent,
};

struct SquaresRenderData {
    hovered_color: Handle<StandardMaterial>,
    selected_color: Handle<StandardMaterial>,
    valid_move_color: Handle<StandardMaterial>,
    shadow_color: Handle<StandardMaterial>,
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
            valid_move_color: materials.add(Color::rgb(0.3, 0.8, 0.3).into()),
            shadow_color: materials.add(Color::rgb(0.6, 0.6, 0.2).into()),
            black_color: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            white_color: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            background_color: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum SquareColor {
    White,
    Black,
}

#[derive(Clone, Component, Copy, Debug)]
pub struct Square;

// (0, 0) is A1, (0, 7) is A8
#[derive(Clone, Component, Copy, Debug, Eq, PartialEq)]
pub struct BoardPosition {
    pub row: i8,
    pub col: i8,
}

impl BoardPosition {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }

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

    pub fn is_in_bounds(self) -> bool {
        (0..8).contains(&self.row) && (0..8).contains(&self.col)
    }

    pub fn next(self) -> Option<Self> {
        let mut next = self;
        if next.is_in_bounds() {
            next.col += 1;
            if next.is_in_bounds() {
                Some(next)
            } else {
                next.row += 1;
                next.col = 0;
                if next.is_in_bounds() {
                    Some(next)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

impl Add<(i8, i8)> for BoardPosition {
    type Output = Self;

    fn add(self, rhs: (i8, i8)) -> Self::Output {
        Self {
            row: self.row + rhs.0,
            col: self.col + rhs.1,
        }
    }
}

impl AddAssign<(i8, i8)> for BoardPosition {
    fn add_assign(&mut self, other: (i8, i8)) {
        *self = *self + other;
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
            let material = match pos.square_color() {
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
                .insert(Square)
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

#[allow(clippy::type_complexity)]
fn render_board(
    hovered_square: Res<HoveredSquare>,
    turn_data: Res<TurnData>,
    materials: Res<SquaresRenderData>,
    mut square_query: Query<
        (
            Entity,
            &BoardPosition,
            Option<&ValidMove>,
            &mut Handle<StandardMaterial>,
        ),
        With<Square>,
    >,
    board_pos_query: Query<&BoardPosition>,
    shadow_squares: Res<ShadowSquares>,
) {
    let piece_pos = turn_data
        .move_piece
        .and_then(|piece_ent| board_pos_query.get(piece_ent).ok());

    for (entity, pos, valid_move, mut material) in &mut square_query {
        if Some(pos) == piece_pos {
            *material = materials.selected_color.clone();
        } else if Some(entity) == hovered_square.entity {
            *material = materials.hovered_color.clone();
        } else if valid_move.is_some() {
            *material = materials.valid_move_color.clone();
        } else if shadow_squares.0.contains(pos) {
            *material = materials.shadow_color.clone();
        } else {
            match pos.square_color() {
                SquareColor::White => *material = materials.white_color.clone(), // TODO: don't clone materials?
                SquareColor::Black => *material = materials.black_color.clone(),
            }
        }
    }
}

#[derive(Debug, Default)]
struct HoveredSquare {
    entity: Option<Entity>,
}

#[derive(Debug)]
pub struct ClickSquareEvent {
    pub kind: MouseButton,
    pub board_pos: Option<BoardPosition>,
}

fn click_square(
    mut pick_events: EventReader<PickingEvent>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    squares_query: Query<&Square>,
    board_pos_query: Query<&BoardPosition>,
    mut hovered_square: ResMut<HoveredSquare>,
    mut click_square_events: EventWriter<ClickSquareEvent>,
) {
    for event in pick_events.iter() {
        match event {
            PickingEvent::Hover(HoverEvent::JustEntered(e)) => {
                if squares_query.contains(*e) {
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

    let board_pos = hovered_square
        .entity
        .and_then(|sq_ent| board_pos_query.get(sq_ent).ok().copied());

    let button_kinds = [MouseButton::Left, MouseButton::Right, MouseButton::Middle];
    for kind in button_kinds {
        if mouse_button_inputs.just_pressed(kind) {
            click_square_events.send(ClickSquareEvent { kind, board_pos });
        }
    }
}

#[derive(Component, Default)]
struct ShadowSquares(Vec<BoardPosition>);

fn leave_shadow(
    mut events: EventReader<PieceMoveEvent>,
    mut shadow_squares: ResMut<ShadowSquares>,
) {
    for event in events.iter() {
        shadow_squares.0.clear();
        shadow_squares.0.push(event.source);
        shadow_squares.0.push(event.target);
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
            .add_system(leave_shadow)
            .init_resource::<ShadowSquares>();
    }
}
