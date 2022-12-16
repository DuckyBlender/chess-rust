use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
// Hashmap for storing pieces
use std::collections::HashMap;

const SQUARE_SIZE: f32 = 60.0;
const PIECES: [&str; 6] = ["pawn", "rook", "knight", "bishop", "queen", "king"];
const COLORS: [&str; 2] = ["white", "black"];
const PIECE_SIZE: f32 = 0.8;
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

enum PieceColor {
    White,
    Black,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Setup camera at coordinate (4*30, 4*30)
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            4.0 * SQUARE_SIZE - SQUARE_SIZE / 2.0,
            4.0 * SQUARE_SIZE - SQUARE_SIZE / 2.0,
            500.0,
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

    // LETTERS BELOW BOARD
    let font = asset_server.load("fonts/Lexend-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::ORANGE,
    };
    for x in 0..8 {
        commands.spawn(Text2dBundle {
            // Convert numbers to letters
            text: Text::from_section(&((x + 65) as u8 as char).to_string(), text_style.clone())
                .with_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            transform: Transform::from_translation(Vec3::new(
                x as f32 * SQUARE_SIZE,
                -SQUARE_SIZE,
                0.0,
            )),
            ..Default::default()
        });
    }

    // NUMBERS TO THE LEFT OF BOARD
    for y in 0..8 {
        commands.spawn(Text2dBundle {
            text: Text::from_section(&(y + 1).to_string(), text_style.clone()).with_alignment(
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_translation(Vec3::new(
                -SQUARE_SIZE,
                y as f32 * SQUARE_SIZE,
                0.0,
            )),
            ..Default::default()
        });
    }

    // Setup pieces


    // Setup for example king piece, will be replaced by FEN
    commands.spawn(SpriteBundle {
        texture: asset_server.load("pieces/white-king.png"),
        transform: Transform::from_translation(Vec3::new(
            4.0 * SQUARE_SIZE,
            4.0 * SQUARE_SIZE,
            1.0,
        )),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rust Chess".to_string(),
                width: SQUARE_SIZE * 10.0,
                height: SQUARE_SIZE * 10.0,
                resizable: true,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}
