# Schach

Some kind of interative chess application.

# Features

## Must haves

Kinda need these to call it chess.

- [x] Render a chess board
- [x] Take turns
- [x] Move pieces according to game rules
    - [x] Usual piece movement
    - [x] Capture pieces
    - [x] Pawn 2-move
    - [ ] Castling
    - [ ] En passant
    - [ ] Promotion
- [x] Check
- [ ] Game over
    - [x] Checkmate
    - [x] Stalemate - no moves
    - [ ] Stalemate - insufficient material
    - [ ] Stalemate - repetition
    - [ ] Resignation / draw by agreement

## Nice to have

We'll see if I get there.

- [ ] Display taken pieces in a side board (also show material point score there)
- [ ] Pan & rotate board
- [ ] Undo move
- [ ] Time control
- [ ] Various screens (splash, pause, game over, etc)
- [ ] Duck chess
- [ ] Wasm target
- [ ] Hosted multiplayer?
- [ ] Output game results in some type of notation
- [ ] Load game state from some type of notation
- [ ] Use an engine to run AI opponent
- [ ] 2D vs 3D

## Out of scope

- No captures / pawn moves in X turns

# References

- Initially adapted from https://caballerocoll.com/blog/bevy-chess-tutorial/
