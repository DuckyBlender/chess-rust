use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
// Hashmap for storing piece locations
use std::collections::HashMap;

const SQUARE_SIZE: f32 = 60.0;
const PIECE_SIZE: f32 = 0.8;
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

const LIGHT_COL: Color = Color::rgb(1.0, 1.0, 1.0);
const DARK_COL: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Component)]
struct Piece;


#[derive(Debug, Clone, Copy)]
enum PieceType {
    None = 0,
    King = 1,
    Pawn = 2,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 6,
}

#[derive(Debug, Clone, Copy)]
enum PieceColor {
    White = 8,
    Black = 16,
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
    for row in 0..8 {
        for column in 0..8 {
            let is_light_square: bool = (row + column) % 2 != 0;

            let square_color = if is_light_square { LIGHT_COL } else { DARK_COL };
            let square_position =
                Vec3::new(column as f32 * SQUARE_SIZE, row as f32 * SQUARE_SIZE, 0.0);
            draw_square(
                &mut commands,
                square_color,
                square_position,
                &mut meshes,
                &mut materials,
            );
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

    // Setup the piece locations in a hashmap (key: square, value: piece)
    // let mut piece_locations: HashMap<String, String> = HashMap::new();

    // Setup the piece locations in a 1d array
    let mut square: [u8; 64] = [0; 64];
    // Insert a piece into the square array, the piece is represented by a number (see Piece enum)
    square[1] = PieceType::Pawn as u8 + PieceColor::White as u8;

    // For now, manually update the board
    update_board(&mut commands, &mut meshes, &mut materials, &square);

    // TODO: Setup the pieces from the FEN string
    // load_position_from_fen();
}

fn update_board(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    square: &[u8; 64],
) {
    
}

fn load_position_from_fen() {
    // TODO: Load the position from the FEN string
}

fn draw_square(
    commands: &mut Commands,
    square_color: Color,
    square_position: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        material: materials.add(square_color.into()),
        mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(SQUARE_SIZE, SQUARE_SIZE),
            flip: false,
        }))),
        transform: Transform::from_translation(square_position),
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
