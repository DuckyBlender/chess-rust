use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
// Hashmap for storing piece locations
use std::collections::HashMap;

const SQUARE_SIZE: f32 = 60.0;
const PIECE_SIZE: f32 = 0.9;
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

const LIGHT_COL: Color = Color::rgb(1.0, 1.0, 1.0);
const DARK_COL: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Component)]
struct Piece;

struct MoveEvent;

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
            text: Text::from_section(((x + 65) as u8 as char).to_string(), text_style.clone())
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
            text: Text::from_section((y + 1).to_string(), text_style.clone()).with_alignment(
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
    // Initialize the board with the FEN string

    println!("{:?}", square[1]);

    // For now, manually update the board with the piece
    // Create a hashmap with the piece prefixes
    let mut piece_type_from_symbol = HashMap::new();
    piece_type_from_symbol.insert('p', PieceType::Pawn);
    piece_type_from_symbol.insert('n', PieceType::Knight);
    piece_type_from_symbol.insert('b', PieceType::Bishop);
    piece_type_from_symbol.insert('r', PieceType::Rook);
    piece_type_from_symbol.insert('q', PieceType::Queen);
    piece_type_from_symbol.insert('k', PieceType::King);

    // TODO: Setup the pieces from the FEN string
    load_position_from_fen(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut square,
        asset_server,
    );
}

fn load_position_from_fen(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    square: &mut [u8; 64],
    asset_server: Res<AssetServer>,
) {
    // Setup the piece locations in a 1d array
    // let mut square: [u8; 64] = [0; 64];
    // Initialize the board with the FEN string
    let mut x: usize = 0;
    let mut y: usize = 0;
    for char in START_FEN.chars() {
        if char == '/' {
            x = 0;
            y += 1;
            continue;
        }

        if char.is_ascii_digit() {
            x += char.to_digit(10).unwrap() as usize;
            continue;
        }

        let piece_type = match char {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => PieceType::None,
        };

        let piece_color_enum = if char.is_lowercase() {
            PieceColor::White
        } else {
            PieceColor::Black
        };

        let piece = (piece_type as u8) | (piece_color_enum as u8);
        square[y * 8 + x] = piece;

        let piece_color = if char.is_lowercase() {
            Color::ORANGE
        } else {
            Color::RED
        };

        let square_position = Vec3::new(x as f32 * SQUARE_SIZE, y as f32 * SQUARE_SIZE, 1.0);
        draw_piece(
            commands,
            piece_type,
            piece_color_enum,
            square_position,
            meshes,
            materials,
            &asset_server,
        );
        x += 1;
    }
}

fn draw_piece(
    commands: &mut Commands,
    piece_type: PieceType,
    piece_color: PieceColor,
    square_position: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    // Map the piece type to the correct image
    let piece_img = match (&piece_type, &piece_color) {
        (PieceType::Pawn, PieceColor::White) => "white-pawn.png",
        (PieceType::Knight, PieceColor::White) => "white-knight.png",
        (PieceType::Bishop, PieceColor::White) => "white-bishop.png",
        (PieceType::Rook, PieceColor::White) => "white-rook.png",
        (PieceType::Queen, PieceColor::White) => "white-queen.png",
        (PieceType::King, PieceColor::White) => "white-king.png",
        (PieceType::Pawn, PieceColor::Black) => "white-pawn.png",
        (PieceType::Knight, PieceColor::Black) => "black-knight.png",
        (PieceType::Bishop, PieceColor::Black) => "black-bishop.png",
        (PieceType::Rook, PieceColor::Black) => "black-rook.png",
        (PieceType::Queen, PieceColor::Black) => "black-queen.png",
        (PieceType::King, PieceColor::Black) => "black-king.png",
        (PieceType::None, _) => "crong.png",
    };
    // Add the path to the image
    let piece_img = format!("pieces/{}", &piece_img);

    // Spawn the piece
    commands.spawn(SpriteBundle {
        texture: asset_server.load(piece_img),
        transform: Transform::from_translation(square_position),
        ..Default::default()
    });
}

// TODO: Finish this function
// fn translate_square_to_position(square: String) -> Vec3 {
// Convert the square to a position
// There are 8 squares per row
// Start from the top left corner (X: 0, Y: BLOCK_SIZE * 8)
// The square is represented by a number (0-63)

// println!("pos: {}, x: {}, y: {}", square, x, y);
// Vec3::new(x as f32, y as f32, 0.0)
// }

// TODO: Update the board when a move event has been triggered
// fn update_board() {
// }

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
