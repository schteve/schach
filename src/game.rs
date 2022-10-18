use bevy::prelude::*;

use crate::{
    board::{BoardPosition, ClickSquareEvent, Square},
    pieces::{Piece, PieceAnimCompleteEvent, PieceColor, PieceMoveEvent},
};

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
    CheckCapture,
    EndTurn,
}

#[derive(Clone, Component, Copy, Default)]
pub struct TurnData {
    state: TurnState,
    pub move_piece: Option<Entity>,
    pub move_target: Option<BoardPosition>,
}

#[derive(Component)]
pub struct ValidMove;

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
    valid_moves_query: Query<(Entity, &BoardPosition), With<ValidMove>>,
    mut piece_move_events: EventWriter<PieceMoveEvent>,
    mut anim_complete_events: EventReader<PieceAnimCompleteEvent>,
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
                        } else if valid_moves_query.iter().any(|(_, pos)| *pos == target_pos) {
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
                    for (entity, _) in &valid_moves_query {
                        commands.entity(entity).remove::<ValidMove>();
                    }
                }
            }
        }
        TurnState::AnimateMove => {
            for event in anim_complete_events.iter() {
                if event.entity == turn_data.move_piece.unwrap() {
                    turn_data.state = TurnState::CheckCapture;
                }
            }
        }
        TurnState::CheckCapture => {
            for (entity, piece, pos) in &piece_query {
                if turn.0 != piece.color && *pos == turn_data.move_target.unwrap() {
                    commands.entity(entity).despawn_recursive();
                }
            }
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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(turn_manager)
            .init_resource::<Turn>()
            .init_resource::<TurnData>();
    }
}
