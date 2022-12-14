use std::fmt;

use bevy::prelude::*;

use crate::board::BoardPosition;

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

struct PiecesRenderData {
    king: PiecePbr,
    queen: PiecePbr,
    rook: PiecePbr,
    bishop: PiecePbr,
    knight: PiecePbr,
    pawn: PiecePbr,
    white_mat: Handle<StandardMaterial>,
    black_mat: Handle<StandardMaterial>,
}

impl FromWorld for PiecesRenderData {
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PieceColor {
    #[default]
    White,
    Black,
}

impl PieceColor {
    pub fn next(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::White => "White",
                Self::Black => "Black",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn(bool),
}

#[derive(Clone, Component, Copy, Debug, Eq, PartialEq)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
}

struct PieceConstData {
    piece: Piece,
    pos: BoardPosition,
}

#[rustfmt::skip]
const STARTING_BOARD: [PieceConstData; 32] = [
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Rook,        }, pos: BoardPosition { row: 0, col: 0 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Knight,      }, pos: BoardPosition { row: 0, col: 1 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Bishop,      }, pos: BoardPosition { row: 0, col: 2 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Queen,       }, pos: BoardPosition { row: 0, col: 3 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::King,        }, pos: BoardPosition { row: 0, col: 4 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Bishop,      }, pos: BoardPosition { row: 0, col: 5 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Knight,      }, pos: BoardPosition { row: 0, col: 6 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Rook,        }, pos: BoardPosition { row: 0, col: 7 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 1, col: 0 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 1, col: 1 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 1, col: 2 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 1, col: 3 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 1, col: 4 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 1, col: 5 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 1, col: 6 } },
    PieceConstData { piece: Piece { color: PieceColor::White, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 1, col: 7 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 6, col: 0 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 6, col: 1 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 6, col: 2 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 6, col: 3 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 6, col: 4 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 6, col: 5 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 6, col: 6 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Pawn(false), }, pos: BoardPosition { row: 6, col: 7 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Rook,        }, pos: BoardPosition { row: 7, col: 0 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Knight,      }, pos: BoardPosition { row: 7, col: 1 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Bishop,      }, pos: BoardPosition { row: 7, col: 2 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Queen,       }, pos: BoardPosition { row: 7, col: 3 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::King,        }, pos: BoardPosition { row: 7, col: 4 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Bishop,      }, pos: BoardPosition { row: 7, col: 5 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Knight,      }, pos: BoardPosition { row: 7, col: 6 } },
    PieceConstData { piece: Piece { color: PieceColor::Black, kind: PieceKind::Rook,        }, pos: BoardPosition { row: 7, col: 7 } },
];

fn create_pieces(mut commands: Commands, piece_render_data: Res<PiecesRenderData>) {
    for piece_data in STARTING_BOARD {
        spawn_piece(
            &mut commands,
            piece_data.piece,
            piece_data.pos,
            &piece_render_data,
        );
    }
}

fn spawn_piece(
    commands: &mut Commands,
    piece: Piece,
    board_pos: BoardPosition,
    render_data: &Res<PiecesRenderData>,
) {
    let pbr = match piece.kind {
        PieceKind::King => &render_data.king,
        PieceKind::Queen => &render_data.queen,
        PieceKind::Rook => &render_data.rook,
        PieceKind::Bishop => &render_data.bishop,
        PieceKind::Knight => &render_data.knight,
        PieceKind::Pawn(_) => &render_data.pawn,
    };
    let mat = match piece.color {
        PieceColor::White => &render_data.white_mat,
        PieceColor::Black => &render_data.black_mat,
    };

    commands
        .spawn_bundle(PbrBundle::default())
        .insert(piece)
        .insert(board_pos)
        .with_children(|parent| {
            for mesh in &pbr.meshes {
                parent.spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: mat.clone(),
                    transform: pbr.transform,
                    ..default()
                });
            }
        });
}

fn animate_pieces(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &BoardPosition), With<Piece>>,
    mut anim_complete_events: EventWriter<PieceAnimCompleteEvent>,
) {
    for (entity, mut transform, board_pos) in &mut query {
        let direction = board_pos.to_translation() - transform.translation;
        if direction.length() != 0.0 {
            let speed = 5.0;
            let step = direction.normalize() * time.delta_seconds() * speed;
            // If it's only a small step then move the whole distance and no further
            let step_to_use = if direction.length() > step.length() {
                step
            } else {
                anim_complete_events.send(PieceAnimCompleteEvent { entity });
                direction
            };
            transform.translation += step_to_use;
        }
    }
}

#[derive(Debug)]
pub struct PieceAnimCompleteEvent {
    pub entity: Entity,
}

#[derive(Debug)]
pub struct PieceMoveEvent {
    pub entity: Entity,
    pub source: BoardPosition,
    pub target: BoardPosition,
}

impl PieceMoveEvent {
    pub fn new(entity: Entity, source: BoardPosition, target: BoardPosition) -> Self {
        Self {
            entity,
            source,
            target,
        }
    }
}

fn move_pieces(
    mut events: EventReader<PieceMoveEvent>,
    mut piece_pos_query: Query<(Entity, &mut BoardPosition), With<Piece>>,
) {
    for event in events.iter() {
        for (entity, mut pos) in &mut piece_pos_query {
            if event.entity == entity {
                *pos = event.target;
            }
        }
    }
}

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_pieces)
            .init_resource::<PiecesRenderData>()
            .add_system(animate_pieces)
            .add_system(move_pieces)
            .add_event::<PieceMoveEvent>()
            .add_event::<PieceAnimCompleteEvent>();
    }
}
