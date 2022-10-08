use crate::board::Square;
use bevy::prelude::*;

#[rustfmt::skip]
const PIECE_TRANSFORMS: [Transform; 6] = [
    Transform { translation: Vec3::new(-0.2, 0.0, -1.9),  rotation: Quat::IDENTITY, scale: Vec3::new(0.2, 0.2, 0.2) }, // King
    Transform { translation: Vec3::new(-0.2, 0.0, -0.95), rotation: Quat::IDENTITY, scale: Vec3::new(0.2, 0.2, 0.2) }, // Queen
    Transform { translation: Vec3::new(-0.1, 0.0, 1.8),   rotation: Quat::IDENTITY, scale: Vec3::new(0.2, 0.2, 0.2) }, // Rook
    Transform { translation: Vec3::new(-0.1, 0.0, 0.0),   rotation: Quat::IDENTITY, scale: Vec3::new(0.2, 0.2, 0.2) }, // Bishop
    Transform { translation: Vec3::new(-0.2, 0.0, 0.9),   rotation: Quat::IDENTITY, scale: Vec3::new(0.2, 0.2, 0.2) }, // Knight
    Transform { translation: Vec3::new(-0.05, 0.0, 2.6),  rotation: Quat::IDENTITY, scale: Vec3::new(0.2, 0.2, 0.2) }, // Pawn
];

struct PiecePbr {
    meshes: Vec<Handle<Mesh>>,
    transform: Transform,
}

struct PieceData {
    king: PiecePbr,
    queen: PiecePbr,
    rook: PiecePbr,
    bishop: PiecePbr,
    knight: PiecePbr,
    pawn: PiecePbr,
    white_mat: Handle<StandardMaterial>,
    black_mat: Handle<StandardMaterial>,
}

impl FromWorld for PieceData {
    fn from_world(world: &mut World) -> Self {
        // Load all the meshes
        // TODO: make the mesh path part of the const data table?
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let king: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh0/Primitive0");
        let king_cross: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh1/Primitive0");
        let pawn: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh2/Primitive0");
        let knight_1: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh3/Primitive0");
        let knight_2: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh4/Primitive0");
        let rook: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh5/Primitive0");
        let bishop: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh6/Primitive0");
        let queen: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh7/Primitive0");

        // Create materials
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let white_mat = materials.add(Color::rgb(1., 0.8, 0.8).into());
        let black_mat = materials.add(Color::rgb(0., 0.2, 0.2).into());

        Self {
            king: PiecePbr {
                meshes: vec![king, king_cross],
                transform: PIECE_TRANSFORMS[0],
            },
            queen: PiecePbr {
                meshes: vec![queen],
                transform: PIECE_TRANSFORMS[1],
            },
            rook: PiecePbr {
                meshes: vec![rook],
                transform: PIECE_TRANSFORMS[2],
            },
            bishop: PiecePbr {
                meshes: vec![bishop],
                transform: PIECE_TRANSFORMS[3],
            },
            knight: PiecePbr {
                meshes: vec![knight_1, knight_2],
                transform: PIECE_TRANSFORMS[4],
            },
            pawn: PiecePbr {
                meshes: vec![pawn],
                transform: PIECE_TRANSFORMS[5],
            },
            white_mat,
            black_mat,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
enum PieceKind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Component, Copy)]
struct Piece {
    color: PieceColor,
    kind: PieceKind,
    square: Square,
}

#[rustfmt::skip]
const STARTING_BOARD: [Piece; 32] = [
    Piece { color: PieceColor::White, kind: PieceKind::Rook,    square: Square { row: 0, col: 0 } },
    Piece { color: PieceColor::White, kind: PieceKind::Knight,  square: Square { row: 0, col: 1 } },
    Piece { color: PieceColor::White, kind: PieceKind::Bishop,  square: Square { row: 0, col: 2 } },
    Piece { color: PieceColor::White, kind: PieceKind::Queen,   square: Square { row: 0, col: 3 } },
    Piece { color: PieceColor::White, kind: PieceKind::King,    square: Square { row: 0, col: 4 } },
    Piece { color: PieceColor::White, kind: PieceKind::Bishop,  square: Square { row: 0, col: 5 } },
    Piece { color: PieceColor::White, kind: PieceKind::Knight,  square: Square { row: 0, col: 6 } },
    Piece { color: PieceColor::White, kind: PieceKind::Rook,    square: Square { row: 0, col: 7 } },
    Piece { color: PieceColor::White, kind: PieceKind::Pawn,    square: Square { row: 1, col: 0 } },
    Piece { color: PieceColor::White, kind: PieceKind::Pawn,    square: Square { row: 1, col: 1 } },
    Piece { color: PieceColor::White, kind: PieceKind::Pawn,    square: Square { row: 1, col: 2 } },
    Piece { color: PieceColor::White, kind: PieceKind::Pawn,    square: Square { row: 1, col: 3 } },
    Piece { color: PieceColor::White, kind: PieceKind::Pawn,    square: Square { row: 1, col: 4 } },
    Piece { color: PieceColor::White, kind: PieceKind::Pawn,    square: Square { row: 1, col: 5 } },
    Piece { color: PieceColor::White, kind: PieceKind::Pawn,    square: Square { row: 1, col: 6 } },
    Piece { color: PieceColor::White, kind: PieceKind::Pawn,    square: Square { row: 1, col: 7 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Rook,    square: Square { row: 7, col: 0 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Knight,  square: Square { row: 7, col: 1 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Bishop,  square: Square { row: 7, col: 2 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Queen,   square: Square { row: 7, col: 3 } },
    Piece { color: PieceColor::Black, kind: PieceKind::King,    square: Square { row: 7, col: 4 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Bishop,  square: Square { row: 7, col: 5 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Knight,  square: Square { row: 7, col: 6 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Rook,    square: Square { row: 7, col: 7 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Pawn,    square: Square { row: 6, col: 0 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Pawn,    square: Square { row: 6, col: 1 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Pawn,    square: Square { row: 6, col: 2 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Pawn,    square: Square { row: 6, col: 3 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Pawn,    square: Square { row: 6, col: 4 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Pawn,    square: Square { row: 6, col: 5 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Pawn,    square: Square { row: 6, col: 6 } },
    Piece { color: PieceColor::Black, kind: PieceKind::Pawn,    square: Square { row: 6, col: 7 } },
];

fn create_pieces(mut commands: Commands, piece_data: Res<PieceData>) {
    for piece in STARTING_BOARD {
        spawn_piece(&mut commands, &piece_data, piece);
    }
}

fn spawn_piece(commands: &mut Commands, piece_data: &Res<PieceData>, piece: Piece) {
    let data = match piece.kind {
        PieceKind::King => &piece_data.king,
        PieceKind::Queen => &piece_data.queen,
        PieceKind::Bishop => &piece_data.bishop,
        PieceKind::Knight => &piece_data.knight,
        PieceKind::Rook => &piece_data.rook,
        PieceKind::Pawn => &piece_data.pawn,
    };
    let mat = match piece.color {
        PieceColor::White => &piece_data.white_mat,
        PieceColor::Black => &piece_data.black_mat,
    };

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(piece.square.world_coords()),
            ..default()
        })
        .insert(piece)
        .with_children(|parent| {
            for mesh in &data.meshes {
                parent.spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: mat.clone(),
                    transform: data.transform,
                    ..default()
                });
            }
        });
}

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_pieces)
            .init_resource::<PieceData>();
    }
}
