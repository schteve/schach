use bevy::prelude::*;
use bevy_mod_picking::{HoverEvent, PickableBundle, PickingEvent};

use crate::pieces::{Piece, PieceColor, PieceMoveEvent};

struct SquaresRenderData {
    hovered_color: Handle<StandardMaterial>,
    selected_color: Handle<StandardMaterial>,
    valid_move_color: Handle<StandardMaterial>,
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
struct Square;

// (0, 0) is A1, (0, 7) is A8
#[derive(Clone, Component, Copy, Debug, Eq, PartialEq)]
pub struct BoardPosition {
    pub row: u8,
    pub col: u8,
}

impl BoardPosition {
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
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
struct ClickSquareEvent {
    kind: MouseButton,
    board_pos: Option<BoardPosition>,
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

#[derive(Clone, Component, Copy, Debug, Default)]
struct Turn(PieceColor);

impl Turn {
    fn advance(&mut self) {
        self.0 = match self.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Clone, Copy, Default)]
enum TurnState {
    #[default]
    SelectPiece,
    ShowHighlights,
    SelectTarget,
    AnimateMove,
    EndTurn,
}

#[derive(Clone, Component, Copy, Default)]
struct TurnData {
    state: TurnState,
    move_piece: Option<Entity>,
    move_target: Option<BoardPosition>,
}

#[derive(Component)]
struct ValidMove;

/*
                          ┌──────────────────────────────────────────┐
                          │                                          │
                 ┌────────▼─────────┐                                │
                 │                  │                                │
                 │ Select piece     ◄──────────────────────┐         │
                 │                  │                      │         │
                 └────────┬─────────┘                      │         │
                          │                                │         │
                          │ Valid (own piece)              │         │
                          │                                │         │
                 ┌────────▼─────────┐                      │         │
                 │ Highlight piece  │                      │         │
┌────────────────► Generate moves   │                      │         │
│                │ Highlight moves  │                      │         │
│                └────────┬─────────┘                      │         │
│                         │                                │         │
│                         │                                │         │
│                         │                                │         │
│                ┌────────▼─────────┐                      │         │
│        Invalid │                  │ Invalid              │         │
└────────────────┤ Select target    ├──────────────────────┘         │
     (own piece) │                  │ (enemy, empty, off board)      │
                 └────────┬─────────┘                                │
                          │                                          │
                          │ Valid target selected                    │
                          │                                          │
                 ┌────────▼─────────┐                                │
                 │ Clear highlights │                                │
                 │ Enact move       │                                │
                 │ Animate move     │                                │
                 └────────┬─────────┘                                │
                          │                                          │
                          │ Anim done                                │
                          │                                          │
                 ┌────────▼─────────┐                                │
                 │ Clear selections │                                │
                 │ Change player    ├────────────────────────────────┘
                 │ End turn         │
                 └──────────────────┘
 */
#[allow(clippy::too_many_arguments)]
fn turn_manager(
    mut commands: Commands,
    mut turn: ResMut<Turn>,
    mut turn_data: ResMut<TurnData>,
    mut click_square_events: EventReader<ClickSquareEvent>,
    piece_query: Query<(Entity, &Piece, &BoardPosition)>,
    square_query: Query<(Entity, &BoardPosition), With<Square>>,
    board_query: Query<(&Piece, &BoardPosition)>,
    valid_moves_query: Query<Entity, With<ValidMove>>,
    mut piece_move_events: EventWriter<PieceMoveEvent>,
) {
    match turn_data.state {
        TurnState::SelectPiece => {
            for ev in click_square_events.iter() {
                if ev.kind == MouseButton::Left {
                    if let Some(pos) = ev.board_pos {
                        for (entity, piece, piece_pos) in &piece_query {
                            if turn.0 == piece.color && pos == *piece_pos {
                                turn_data.move_piece = Some(entity); // This piece is highlighted in render_board()
                                turn_data.state = TurnState::ShowHighlights;
                                break;
                            }
                        }
                    } else {
                        turn_data.move_piece = None;
                    }
                }
            }
        }
        TurnState::ShowHighlights => {
            let (_, piece, pos) = piece_query.get(turn_data.move_piece.unwrap()).unwrap();
            let valid_moves = piece.valid_moves(*pos, &board_query);
            for (entity, board_pos) in &square_query {
                if valid_moves.contains(board_pos) {
                    commands.entity(entity).insert(ValidMove);
                }
            }
            turn_data.state = TurnState::SelectTarget;
        }
        TurnState::SelectTarget => {
            for ev in click_square_events.iter() {
                if ev.kind == MouseButton::Left {
                    if let Some(target_pos) = ev.board_pos {
                        // Get details of the source piece
                        let (_, piece, source_pos) =
                            piece_query.get(turn_data.move_piece.unwrap()).unwrap();

                        // Check if the target selection is a friendly piece
                        let friendly_target =
                            piece_query.iter().find_map(|(entity, piece, piece_pos)| {
                                if target_pos == *piece_pos && turn.0 == piece.color {
                                    Some(entity)
                                } else {
                                    None
                                }
                            });

                        if let Some(entity) = friendly_target {
                            // Invalid selection, but it's our own piece so just go back and use this as the piece to move
                            turn_data.move_piece = Some(entity); // This piece is highlighted in render_board()
                            turn_data.state = TurnState::ShowHighlights;
                        } else if is_target_valid(*piece, *source_pos, target_pos) {
                            // Valid selection, move this piece
                            turn_data.move_target = Some(target_pos);
                            turn_data.state = TurnState::AnimateMove;
                            piece_move_events.send(PieceMoveEvent::new(
                                turn_data.move_piece.unwrap(),
                                turn_data.move_target.unwrap(),
                            ));
                        } else {
                            // Invalid selection (whether enemy piece or empty). Deselect and go back to the beginning.
                            turn_data.move_piece = None;
                            turn_data.state = TurnState::SelectPiece;
                        }
                    } else {
                        // Invalid selection (off board). Deselect and go back to the beginning.
                        turn_data.move_piece = None;
                        turn_data.state = TurnState::SelectPiece;
                    }

                    // Clear highlighted valid moves
                    for entity in &valid_moves_query {
                        commands.entity(entity).remove::<ValidMove>();
                    }
                }
            }
        }
        TurnState::AnimateMove => {
            // TODO: wait for animation to complete
            turn_data.state = TurnState::EndTurn;
        }
        TurnState::EndTurn => {
            // Clear selections & end turn
            turn_data.move_piece = None;
            turn_data.move_target = None;
            turn_data.state = TurnState::SelectPiece;
            // Change player
            turn.advance();
        }
    }
}

fn is_target_valid(
    _move_piece: Piece,
    _move_source: BoardPosition,
    _move_target: BoardPosition,
) -> bool {
    true
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
            .add_system(turn_manager)
            .init_resource::<Turn>()
            .init_resource::<TurnData>();
    }
}
