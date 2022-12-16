use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const SQUARE_SIZE: f32 = 50.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Setup camera at coordinate (4*30, 4*30)
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            4.0 * SQUARE_SIZE,
            4.0 * SQUARE_SIZE,
            0.0,
        )),
        ..Default::default()
    });
    // Setup chess board

    // let mut board = Vec::new();
    for x in 0..8 {
        for y in 0..8 {
            // Checkerboard pattern
            if (x + y) % 2 == 0 {
                commands.spawn(MaterialMesh2dBundle {
                    material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                    // Size 30 quads
                    mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(SQUARE_SIZE, SQUARE_SIZE),
                        flip: false,
                    }))),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * SQUARE_SIZE,
                        y as f32 * SQUARE_SIZE,
                        0.0,
                    )),
                    ..Default::default()
                });
            } else {
                commands.spawn(MaterialMesh2dBundle {
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    // Size 30 quads
                    mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(SQUARE_SIZE, SQUARE_SIZE),
                        flip: false,
                    }))),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * SQUARE_SIZE,
                        y as f32 * SQUARE_SIZE,
                        0.0,
                    )),
                    ..Default::default()
                });
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}