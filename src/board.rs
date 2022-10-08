use bevy::prelude::*;
use bevy_mod_picking::{HoverEvent, PickableBundle, PickingEvent};

struct SquareMaterials {
    hovered_color: Handle<StandardMaterial>,
    selected_color: Handle<StandardMaterial>,
    //valid_movement_color: Handle<StandardMaterial>,
    black_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
    background_color: Handle<StandardMaterial>,
}

impl FromWorld for SquareMaterials {
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

enum SquareColor {
    White,
    Black,
}

// (0, 0) is A1, (0, 7) is A8
#[derive(Clone, Component, Copy)]
pub struct Square {
    pub row: u8,
    pub col: u8,
}

impl Square {
    fn color(&self) -> SquareColor {
        if (self.row + self.col) % 2 == 0 {
            SquareColor::Black
        } else {
            SquareColor::White
        }
    }

    pub fn world_coords(&self) -> Vec3 {
        let x = self.col as f32 - 3.5;
        let y = 0.25;
        let z = -(self.row as f32 - 3.5);
        Vec3::new(x, y, z)
    }
}

impl From<(u8, u8)> for Square {
    fn from(other: (u8, u8)) -> Self {
        Self {
            row: other.0,
            col: other.1,
        }
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct HoveredSquare {
    entity: Option<Entity>,
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<SquareMaterials>,
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
            let sq = Square { row, col };
            let material = match sq.color() {
                SquareColor::White => materials.white_color.clone(),
                SquareColor::Black => materials.black_color.clone(),
            };
            commands
                .spawn_bundle(PbrBundle {
                    mesh: square_mesh.clone(),
                    material,
                    transform: Transform::from_translation(sq.world_coords()),
                    ..default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(sq);
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
    selected_square: Res<SelectedSquare>,
    materials: Res<SquareMaterials>,
    mut query: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>,
) {
    for (entity, square, mut material) in query.iter_mut() {
        if Some(entity) == selected_square.entity {
            *material = materials.selected_color.clone();
        } else if Some(entity) == hovered_square.entity {
            *material = materials.hovered_color.clone();
        } else {
            match square.color() {
                SquareColor::White => *material = materials.white_color.clone(),
                SquareColor::Black => *material = materials.black_color.clone(),
            }
        }
    }
}

fn select_square(
    mut events: EventReader<PickingEvent>,
    mut hovered_square: ResMut<HoveredSquare>,
    mut selected_square: ResMut<SelectedSquare>,
    squares_query: Query<&Square>,
) {
    for event in events.iter() {
        match event {
            /*PickingEvent::Selection(SelectionEvent::JustSelected(e)) => {
                if let Ok(_) = squares_query.get(*e) {
                    selected_square.entity = Some(*e);
                } else {
                    selected_square.entity = None;
                }
            },*/
            PickingEvent::Hover(e) => match e {
                HoverEvent::JustEntered(e) => {
                    if squares_query.get(*e).is_ok() {
                        hovered_square.entity = Some(*e);
                    } else {
                        hovered_square.entity = None;
                    }
                }
                HoverEvent::JustLeft(e) => {
                    if let Some(prev_ent) = hovered_square.entity {
                        if *e == prev_ent {
                            hovered_square.entity = None;
                        }
                    }
                }
            },
            PickingEvent::Clicked(e) => {
                if squares_query.get(*e).is_ok() {
                    selected_square.entity = Some(*e);
                } else {
                    selected_square.entity = None;
                }
            }
            _ => (),
        }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_board)
            .add_system(render_board)
            .add_system(select_square)
            .init_resource::<SquareMaterials>()
            .init_resource::<SelectedSquare>()
            .init_resource::<HoveredSquare>();
    }
}
