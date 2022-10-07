use crate::board::Square;
use bevy::prelude::*;

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

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> =
        asset_server.load("models/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> =
        asset_server.load("models/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> =
        asset_server.load("models/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> =
        asset_server.load("models/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> =
        asset_server.load("models/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> =
        asset_server.load("models/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> =
        asset_server.load("models/pieces.glb#Mesh7/Primitive0");

    // Add some materials
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    // TODO: make this even more data driven. Load init board table at startup?
    // White pieces
    spawn_piece(
        &mut commands,
        white_material.clone(),
        Piece {
            color: PieceColor::White,
            kind: PieceKind::Rook,
            square: (0, 0).into(),
        },
        [rook_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        white_material.clone(),
        Piece {
            color: PieceColor::White,
            kind: PieceKind::Knight,
            square: (0, 1).into(),
        },
        [knight_1_handle.clone(), knight_2_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        white_material.clone(),
        Piece {
            color: PieceColor::White,
            kind: PieceKind::Bishop,
            square: (0, 2).into(),
        },
        [bishop_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        white_material.clone(),
        Piece {
            color: PieceColor::White,
            kind: PieceKind::Queen,
            square: (0, 3).into(),
        },
        [queen_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        white_material.clone(),
        Piece {
            color: PieceColor::White,
            kind: PieceKind::King,
            square: (0, 4).into(),
        },
        [king_handle.clone(), king_cross_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        white_material.clone(),
        Piece {
            color: PieceColor::White,
            kind: PieceKind::Bishop,
            square: (0, 5).into(),
        },
        [bishop_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        white_material.clone(),
        Piece {
            color: PieceColor::White,
            kind: PieceKind::Knight,
            square: (0, 6).into(),
        },
        [knight_1_handle.clone(), knight_2_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        white_material.clone(),
        Piece {
            color: PieceColor::White,
            kind: PieceKind::Rook,
            square: (0, 7).into(),
        },
        [rook_handle.clone()],
    );
    for i in 0..8 {
        spawn_piece(
            &mut commands,
            white_material.clone(),
            Piece {
                color: PieceColor::White,
                kind: PieceKind::Pawn,
                square: (1, i).into(),
            },
            [pawn_handle.clone()],
        );
    }

    // Black pieces
    spawn_piece(
        &mut commands,
        black_material.clone(),
        Piece {
            color: PieceColor::Black,
            kind: PieceKind::Rook,
            square: (7, 0).into(),
        },
        [rook_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        black_material.clone(),
        Piece {
            color: PieceColor::Black,
            kind: PieceKind::Knight,
            square: (7, 1).into(),
        },
        [knight_1_handle.clone(), knight_2_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        black_material.clone(),
        Piece {
            color: PieceColor::Black,
            kind: PieceKind::Bishop,
            square: (7, 2).into(),
        },
        [bishop_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        black_material.clone(),
        Piece {
            color: PieceColor::Black,
            kind: PieceKind::Queen,
            square: (7, 3).into(),
        },
        [queen_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        black_material.clone(),
        Piece {
            color: PieceColor::Black,
            kind: PieceKind::King,
            square: (7, 4).into(),
        },
        [king_handle.clone(), king_cross_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        black_material.clone(),
        Piece {
            color: PieceColor::Black,
            kind: PieceKind::Bishop,
            square: (7, 5).into(),
        },
        [bishop_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        black_material.clone(),
        Piece {
            color: PieceColor::Black,
            kind: PieceKind::Knight,
            square: (7, 6).into(),
        },
        [knight_1_handle.clone(), knight_2_handle.clone()],
    );
    spawn_piece(
        &mut commands,
        black_material.clone(),
        Piece {
            color: PieceColor::Black,
            kind: PieceKind::Rook,
            square: (7, 7).into(),
        },
        [rook_handle.clone()],
    );
    for i in 0..8 {
        spawn_piece(
            &mut commands,
            black_material.clone(),
            Piece {
                color: PieceColor::Black,
                kind: PieceKind::Pawn,
                square: (6, i).into(),
            },
            [pawn_handle.clone()],
        );
    }
}

fn spawn_piece<const N: usize>(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    meshes: [Handle<Mesh>; N],
) {
    commands
    .spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(
            piece.square.row as f32,
            0.0,
            piece.square.col as f32,
        )),
        ..Default::default()
    })
    .insert(piece)
    .with_children(|parent| {
        for mesh in meshes {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        }
    });
}

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_pieces);
        /*.add_system(render_board)
        .add_system(select_square)
        .init_resource::<SquareMaterials>()
        .init_resource::<SelectedSquare>()
        .init_resource::<HoveredSquare>();*/
    }
}
