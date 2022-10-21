use std::mem;

use bevy::prelude::*;

use crate::{
    board::{BoardPosition, ClickSquareEvent, Square},
    pieces::{Piece, PieceAnimCompleteEvent, PieceColor, PieceKind, PieceMoveEvent},
};

enum MoveCapture {
    Move,
    Capture,
}

#[derive(Clone, Copy, Component, Debug)]
pub enum GameOver {
    Checkmate(PieceColor), // Winner
    Stalemate,
}

#[derive(Clone, Component, Debug, Default)]
pub struct GameState {
    pub board: [[Option<Piece>; 8]; 8], // Set of rows (first row is A1-A8, etc)
    pub curr_player: PieceColor,
    pub game_over: Option<GameOver>,
}

impl GameState {
    fn get_pos(&self, pos: BoardPosition) -> Option<Piece> {
        // TODO: should this return Result<Option<Piece>, ()> to indicate if something is out of bounds vs just empty?
        if pos.is_in_bounds() {
            self.board[pos.row as usize][pos.col as usize]
        } else {
            None
        }
    }

    fn set_pos(&mut self, pos: BoardPosition, piece: Option<Piece>) -> Option<Piece> {
        if pos.is_in_bounds() {
            mem::replace(&mut self.board[pos.row as usize][pos.col as usize], piece)
        } else {
            None
        }
    }

    fn iter_pieces(&self) -> PieceIter {
        PieceIter {
            game_state: self,
            curr_pos: Some(BoardPosition::new()),
        }
    }

    fn apply_movement(&mut self, from_pos: BoardPosition, to_pos: BoardPosition) -> Option<Piece> {
        assert!(
            from_pos.is_in_bounds(),
            "Moved from out of bounds position: {:?}",
            from_pos
        );
        assert!(
            to_pos.is_in_bounds(),
            "Moved to out of bounds position: {:?}",
            to_pos
        );

        let mut moving_piece = self.get_pos(from_pos);
        assert!(moving_piece.is_some(), "Moving a non-existent piece");

        // Update moving piece to indicate that it has moved
        let p = moving_piece.as_mut().unwrap();
        p.kind = match p.kind {
            PieceKind::Pawn(false) => PieceKind::Pawn(true),
            x => x,
        };

        // Update board
        let taken_piece = self.get_pos(to_pos);
        self.set_pos(from_pos, None);
        self.set_pos(to_pos, moving_piece);
        taken_piece
    }

    fn moves_and_captures(
        &self,
        piece: Piece,
        piece_pos: BoardPosition,
    ) -> (Vec<BoardPosition>, Vec<BoardPosition>) {
        let (mut moves, mut captures) = self.pseudo_moves_and_captures(piece, piece_pos);

        moves.retain(|pos| {
            let mut new_state = self.clone();
            new_state.apply_movement(piece_pos, *pos);
            new_state.advance_turn();
            !new_state.is_in_check(piece.color)
        });

        captures.retain(|pos| {
            let mut new_state = self.clone();
            new_state.apply_movement(piece_pos, *pos);
            new_state.advance_turn();
            !new_state.is_in_check(piece.color)
        });

        (moves, captures)
    }

    fn pseudo_moves_and_captures(
        &self,
        piece: Piece,
        piece_pos: BoardPosition,
    ) -> (Vec<BoardPosition>, Vec<BoardPosition>) {
        // TODO: handle check, en passant, castling, pawn 2-moves
        let mut moves = Vec::new();
        let mut captures = Vec::new();

        match piece.kind {
            PieceKind::King => {
                #[rustfmt::skip]
                let offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
                for offset in offsets {
                    let new_pos = piece_pos + offset;
                    self.save_moves_captures(piece, new_pos, &mut moves, &mut captures);
                }
            }
            PieceKind::Queen => {
                #[rustfmt::skip]
                let directions = [(-1, 0), (1, 0), (0, 1), (0, -1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
                for dir in directions {
                    let (m, c) = self.check_line(piece, piece_pos, dir);
                    moves.extend(m);
                    captures.extend(c);
                }
            }
            PieceKind::Rook => {
                let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];
                for dir in directions {
                    let (m, c) = self.check_line(piece, piece_pos, dir);
                    moves.extend(m);
                    captures.extend(c);
                }
            }
            PieceKind::Bishop => {
                let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
                for dir in directions {
                    let (m, c) = self.check_line(piece, piece_pos, dir);
                    moves.extend(m);
                    captures.extend(c);
                }
            }
            PieceKind::Knight => {
                let diags = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
                for diag in diags {
                    let diag_pos = piece_pos + diag;

                    let new_pos = diag_pos + (diag.0, 0);
                    self.save_moves_captures(piece, new_pos, &mut moves, &mut captures);

                    let new_pos = diag_pos + (0, diag.1);
                    self.save_moves_captures(piece, new_pos, &mut moves, &mut captures);
                }
            }
            PieceKind::Pawn(has_moved) => {
                let next_row = match piece.color {
                    PieceColor::White => 1,
                    PieceColor::Black => -1,
                };

                // 1-move
                let new_pos = piece_pos + (next_row, 0);
                if new_pos.is_in_bounds() && self.get_pos(new_pos).is_none() {
                    moves.push(new_pos);
                }

                // 2-move
                if !has_moved {
                    let new_pos = piece_pos + (next_row * 2, 0);
                    if new_pos.is_in_bounds() && self.get_pos(new_pos).is_none() {
                        moves.push(new_pos);
                    }
                }

                // Captures
                for col in [-1, 1] {
                    let new_pos = piece_pos + (next_row, col);
                    if new_pos.is_in_bounds() {
                        if let Some(Piece { color, .. }) = self.get_pos(new_pos) {
                            if color != piece.color {
                                captures.push(new_pos);
                            }
                        }
                    }
                }
            }
        }

        (moves, captures)
    }

    fn is_move_or_capture(&self, piece: Piece, pos: BoardPosition) -> Option<MoveCapture> {
        if !pos.is_in_bounds() {
            None
        } else if let Some(Piece { color, .. }) = self.get_pos(pos) {
            if color == piece.color {
                None // Blocking
            } else {
                Some(MoveCapture::Capture)
            }
        } else {
            Some(MoveCapture::Move)
        }
    }

    fn save_moves_captures(
        &self,
        piece: Piece,
        pos: BoardPosition,
        moves: &mut Vec<BoardPosition>,
        captures: &mut Vec<BoardPosition>,
    ) {
        match self.is_move_or_capture(piece, pos) {
            Some(MoveCapture::Move) => moves.push(pos),
            Some(MoveCapture::Capture) => captures.push(pos),
            _ => (),
        }
    }

    fn check_line(
        &self,
        piece: Piece,
        from_pos: BoardPosition,
        direction: (i8, i8),
    ) -> (Vec<BoardPosition>, Vec<BoardPosition>) {
        let mut moves = Vec::new();
        let mut captures = Vec::new();

        let mut new_pos = from_pos;
        loop {
            new_pos += direction;
            match self.is_move_or_capture(piece, new_pos) {
                Some(MoveCapture::Move) => moves.push(new_pos),
                Some(MoveCapture::Capture) => {
                    captures.push(new_pos);
                    break;
                }
                None => break,
            }
        }
        (moves, captures)
    }

    fn get_king_pos(&self, player: PieceColor) -> BoardPosition {
        let king = Piece {
            kind: PieceKind::King,
            color: player,
        };
        self.iter_pieces()
            .find_map(|(piece, pos)| if piece == king { Some(pos) } else { None })
            .expect("Couldn't find king for {player:?} player")
    }

    fn is_in_check(&self, player: PieceColor) -> bool {
        let king_pos = self.get_king_pos(player);
        self.iter_pieces()
            .filter(|(piece, _)| piece.color != player)
            .any(|(piece, pos)| {
                let (_, captures) = self.pseudo_moves_and_captures(piece, pos);
                captures.contains(&king_pos)
            })
    }

    fn no_legal_moves(&self) -> bool {
        self.iter_pieces()
            .filter(|(piece, _)| piece.color == self.curr_player)
            .all(|(piece, piece_pos)| {
                let (m, c) = self.moves_and_captures(piece, piece_pos);
                m.is_empty() && c.is_empty()
            })
    }

    fn advance_turn(&mut self) {
        self.curr_player = match self.curr_player {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

struct PieceIter<'a> {
    game_state: &'a GameState,
    curr_pos: Option<BoardPosition>,
}

impl<'a> Iterator for PieceIter<'a> {
    type Item = (Piece, BoardPosition);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let curr_pos = self.curr_pos?;
            self.curr_pos = curr_pos.next();
            if let Some(piece) = self.game_state.get_pos(curr_pos) {
                return Some((piece, curr_pos));
            }
        }
    }
}

#[rustfmt::skip]
const STARTING_BOARD: [[Option<Piece>; 8]; 8] = [
    [
        Some(Piece { color: PieceColor::White, kind: PieceKind::Rook,   }),
        Some(Piece { color: PieceColor::White, kind: PieceKind::Knight, }),
        Some(Piece { color: PieceColor::White, kind: PieceKind::Bishop, }),
        Some(Piece { color: PieceColor::White, kind: PieceKind::Queen,  }),
        Some(Piece { color: PieceColor::White, kind: PieceKind::King,   }),
        Some(Piece { color: PieceColor::White, kind: PieceKind::Bishop, }),
        Some(Piece { color: PieceColor::White, kind: PieceKind::Knight, }),
        Some(Piece { color: PieceColor::White, kind: PieceKind::Rook,   }),
    ],
    [Some(Piece { color: PieceColor::White, kind: PieceKind::Pawn(false) }); 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [Some(Piece { color: PieceColor::Black, kind: PieceKind::Pawn(false) }); 8],
    [
        Some(Piece { color: PieceColor::Black, kind: PieceKind::Rook,   }),
        Some(Piece { color: PieceColor::Black, kind: PieceKind::Knight, }),
        Some(Piece { color: PieceColor::Black, kind: PieceKind::Bishop, }),
        Some(Piece { color: PieceColor::Black, kind: PieceKind::Queen,  }),
        Some(Piece { color: PieceColor::Black, kind: PieceKind::King,   }),
        Some(Piece { color: PieceColor::Black, kind: PieceKind::Bishop, }),
        Some(Piece { color: PieceColor::Black, kind: PieceKind::Knight, }),
        Some(Piece { color: PieceColor::Black, kind: PieceKind::Rook,   }),
    ]
];

fn setup(mut game_state: ResMut<GameState>) {
    game_state.board = STARTING_BOARD;
}

#[derive(Clone, Copy, Default)]
enum TurnState {
    #[default]
    CheckForGameOver,
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

impl TurnData {
    fn reset(&mut self) {
        self.state = TurnState::CheckForGameOver;
        self.move_piece = None;
        self.move_target = None;
    }
}

#[derive(Component)]
pub struct ValidMove;

#[derive(Component)]
struct Captured;

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
    mut game_state: ResMut<GameState>,
    mut turn_data: ResMut<TurnData>,
    mut click_square_events: EventReader<ClickSquareEvent>,
    piece_query: Query<(Entity, &BoardPosition), With<Piece>>,
    captured_query: Query<Entity, With<Captured>>,
    square_query: Query<(Entity, &BoardPosition), With<Square>>,
    valid_moves_query: Query<(Entity, &BoardPosition), With<ValidMove>>,
    mut piece_move_events: EventWriter<PieceMoveEvent>,
    mut anim_complete_events: EventReader<PieceAnimCompleteEvent>,
) {
    if game_state.game_over.is_some() {
        return;
    }

    match turn_data.state {
        TurnState::CheckForGameOver => {
            if game_state.no_legal_moves() {
                if game_state.is_in_check(game_state.curr_player) {
                    game_state.game_over = Some(GameOver::Checkmate(game_state.curr_player.next()))
                } else {
                    game_state.game_over = Some(GameOver::Stalemate)
                }
            } else {
                turn_data.state = TurnState::SelectPiece;
            }
        }
        TurnState::SelectPiece => {
            for ev in click_square_events.iter() {
                if ev.kind == MouseButton::Left {
                    if let Some(pos) = ev.board_pos {
                        for (entity, piece_pos) in &piece_query {
                            let piece = game_state
                                .get_pos(*piece_pos)
                                .expect("Entity for piece exists but it's not on the board");
                            if game_state.curr_player == piece.color && pos == *piece_pos {
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
            let piece_pos = piece_query
                .get_component::<BoardPosition>(turn_data.move_piece.unwrap())
                .unwrap();
            let piece = game_state
                .get_pos(*piece_pos)
                .expect("Entity for piece exists but it's not on the board");
            let (moves, captures) = game_state.moves_and_captures(piece, *piece_pos);
            for (entity, board_pos) in &square_query {
                if moves.contains(board_pos) || captures.contains(board_pos) {
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
                        let friendly_target = piece_query.iter().find_map(|(entity, piece_pos)| {
                            let piece = game_state
                                .get_pos(*piece_pos)
                                .expect("Entity for piece exists but it's not on the board");
                            if target_pos == *piece_pos && game_state.curr_player == piece.color {
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

                            // Unwrap some values - these *should* all be guaranteed to be Some at this point
                            let piece_ent = turn_data.move_piece.unwrap();
                            let source = piece_query
                                .get_component::<BoardPosition>(piece_ent)
                                .unwrap();
                            let target = turn_data.move_target.unwrap();

                            // Move the piece in the game state
                            let captured_piece = game_state.apply_movement(*source, target);
                            if captured_piece.is_some() {
                                // If there's a piece already in the target square, capture it
                                for (entity, piece_pos) in &piece_query {
                                    if *piece_pos == target {
                                        commands.entity(entity).insert(Captured);
                                    }
                                }
                            }

                            // Signal to the ECS that the piece has moved, so it can be updated & animated there
                            piece_move_events.send(PieceMoveEvent::new(
                                turn_data.move_piece.unwrap(),
                                *source,
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
            for entity in &captured_query {
                commands.entity(entity).despawn_recursive();
            }
            turn_data.state = TurnState::EndTurn;
        }
        TurnState::EndTurn => {
            turn_data.reset(); // Clear selections & end turn
            game_state.advance_turn(); // Change player
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(turn_manager)
            .init_resource::<GameState>()
            .init_resource::<TurnData>();
    }
}
