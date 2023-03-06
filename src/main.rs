use bevy::{prelude::*, sprite::MaterialMesh2dBundle, winit::WinitSettings};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::env;

const SQUARE_SIZE: f32 = 50.0;
// const PIECE_SIZE: f32 = 1.0;
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

const LIGHT_COL: Color = Color::rgb(1.0, 1.0, 1.0);
const DARK_COL: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Component, Debug)]
struct Piece;

#[derive(Component, Debug)]
struct DraggedPiece;

#[derive(Component, Debug)]
struct Square;

#[derive(Resource, Debug, Deref, DerefMut)]
struct ChessPieces {
    // Create a 1d array of 64 u8 values
    pub pieces: [u8; 64],
}

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

impl ChessPieces {
    pub fn default() -> Self {
        Self { pieces: [0; 64] }
    }

    pub fn get_piece_index(&self, x: usize, y: usize) -> usize {
        let piece_location = (y * 8) + x;
        let piece = self.pieces[piece_location];
        piece as usize
    }

    pub fn get_piece_name(&self, piece: PieceType) -> &'static str {
        match piece {
            PieceType::Pawn => "pawn",
            PieceType::Knight => "knight",
            PieceType::Bishop => "bishop",
            PieceType::Rook => "rook",
            PieceType::Queen => "queen",
            PieceType::King => "king",
            PieceType::None => "none",
        }
    }

    pub fn get_piece_color(&self, color: PieceColor) -> &'static str {
        match color {
            PieceColor::White => "white",
            PieceColor::Black => "black",
        }
    }
    pub fn get_piece_type(&self, index: usize) -> (PieceType, PieceColor) {
        let piece = self.pieces[index];
        let piece_type = match piece & 7 {
            0 => PieceType::None,
            1 => PieceType::King,
            2 => PieceType::Pawn,
            3 => PieceType::Knight,
            4 => PieceType::Bishop,
            5 => PieceType::Rook,
            6 => PieceType::Queen,
            _ => PieceType::None,
        };
        let piece_color = match piece & 24 {
            8 => PieceColor::White,
            16 => PieceColor::Black,
            _ => PieceColor::White,
        };
        (piece_type, piece_color)
    }
    pub fn get_piece_image(&self, index: usize) -> &str {
        let (piece_type, piece_color) = self.get_piece_type(index);
        match (&piece_type, &piece_color) {
            (PieceType::Pawn, PieceColor::White) => "white-pawn.png",
            (PieceType::Knight, PieceColor::White) => "white-knight.png",
            (PieceType::Bishop, PieceColor::White) => "white-bishop.png",
            (PieceType::Rook, PieceColor::White) => "white-rook.png",
            (PieceType::Queen, PieceColor::White) => "white-queen.png",
            (PieceType::King, PieceColor::White) => "white-king.png",
            (PieceType::Pawn, PieceColor::Black) => "black-pawn.png",
            (PieceType::Knight, PieceColor::Black) => "black-knight.png",
            (PieceType::Bishop, PieceColor::Black) => "black-bishop.png",
            (PieceType::Rook, PieceColor::Black) => "black-rook.png",
            (PieceType::Queen, PieceColor::Black) => "black-queen.png",
            (PieceType::King, PieceColor::Black) => "black-king.png",
            (PieceType::None, _) => "crong.png",
        }
    }
}

fn load_position_from_fen(
    commands: &mut Commands,
    piece_locations: &mut [u8; 64],
    asset_server: Res<AssetServer>,
) {
    // Initialize the board with the FEN string
    let mut x: usize = 0;
    let mut y: usize = 0;
    log::info!("Loading position from FEN...");
    log::info!("FEN: {}", START_FEN);
    for char in START_FEN.chars() {
        if char == '/' {
            // Move to the next row
            x = 0;
            y += 1;
            continue;
        }

        if char.is_ascii_digit() {
            // Skip the number of empty squares
            x += char.to_digit(10).unwrap() as usize;
            continue;
        }

        let piece_type = match char.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => PieceType::None,
        };

        let piece_color = if char.is_lowercase() {
            PieceColor::White
        } else {
            PieceColor::Black
        };

        let piece = (piece_type as u8) | (piece_color as u8);
        piece_locations[y * 8 + x] = piece;
        log::info!("{:?} | {:?} = {}", piece_type, piece_color, piece);

        let square_position = Vec3::new(
            x as f32 * SQUARE_SIZE + SQUARE_SIZE / 2.0,
            y as f32 * SQUARE_SIZE + SQUARE_SIZE / 2.0,
            1.0,
        );
        draw_piece(
            commands,
            piece_type,
            piece_color,
            square_position,
            &asset_server,
        );

        x += 1;
    }
    log::info!("PIECE LOCATIONS:\n{:?}", piece_locations)
}

fn draw_piece(
    commands: &mut Commands,
    piece_type: PieceType,
    piece_color: PieceColor,
    square_position: Vec3,
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
        (PieceType::Pawn, PieceColor::Black) => "black-pawn.png",
        (PieceType::Knight, PieceColor::Black) => "black-knight.png",
        (PieceType::Bishop, PieceColor::Black) => "black-bishop.png",
        (PieceType::Rook, PieceColor::Black) => "black-rook.png",
        (PieceType::Queen, PieceColor::Black) => "black-queen.png",
        (PieceType::King, PieceColor::Black) => "black-king.png",
        (PieceType::None, _) => "crong.png",
    };
    // Add the path to the image
    let piece_img = format!("pieces/{}", &piece_img);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load(piece_img),
            transform: Transform::from_translation(square_position)
                // Scale the piece to the correct size. The png is 60x60 so we divide by 60
                .with_scale(Vec3::splat(SQUARE_SIZE / 60.0)),
            ..Default::default()
        })
        .insert(Piece)
        // to change the default name of "sprite"
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    format!("{:?}", piece_type),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                ),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..Default::default()
            });
        });
}

// TODO: Finish this function
// fn translate_square_to_position(square: String) -> Vec3 {
// Convert the square to a position

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
    // Draw the square with the "Square" component
    commands
        .spawn(MaterialMesh2dBundle {
            material: materials.add(square_color.into()),
            mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(SQUARE_SIZE, SQUARE_SIZE),
                flip: false,
            }))),
            transform: Transform::from_translation(square_position),
            ..Default::default()
        })
        .insert(Square);
}

fn mouse_button_input(buttons: Res<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        log::info!("Left button was pressed");
        // Left button was pressed
    }
    if buttons.just_released(MouseButton::Left) {
        log::info!("Left button was released");
        // Left Button was released
    }
    if buttons.pressed(MouseButton::Right) {
        log::info!("Right button is being held down");
        // Right Button is being held down
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        log::info!("Either the left or the right button was just pressed");
        // Either the left or the right button was just pressed
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut piece_locations: ResMut<ChessPieces>,
) {
    // Create window
    commands.spawn(Window {
        title: "Rust Chess".to_string(),
        resolution: Vec2::new(SQUARE_SIZE * 8.0, SQUARE_SIZE * 8.0).into(),
        resizable: true,
        ..default()
    });

    // Setup camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            4.0 * SQUARE_SIZE,
            4.0 * SQUARE_SIZE,
            // The camera is going to be very high up so that nothing is above it
            500.0,
        )),
        ..Default::default()
    });
    // Setup chess board

    // let mut board = Vec::new();
    for row in 0..8 {
        for column in 0..8 {
            // Check if the square is supposed to be light or dark
            let is_light_square: bool = (row + column) % 2 != 0;

            let square_color = if is_light_square { LIGHT_COL } else { DARK_COL };
            let square_position = Vec3::new(
                column as f32 * SQUARE_SIZE + SQUARE_SIZE / 2.0,
                row as f32 * SQUARE_SIZE + SQUARE_SIZE / 2.0,
                0.0,
            );
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
    for x in 1..=8 {
        commands.spawn(Text2dBundle {
            // Convert numbers to letters
            text: Text::from_section(((x + 64) as u8 as char).to_string(), text_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(Vec3::new(
                x as f32 * SQUARE_SIZE - SQUARE_SIZE / 2.0,
                -SQUARE_SIZE / 2.0,
                0.0,
            )),
            ..Default::default()
        });
    }

    // NUMBERS TO THE LEFT OF BOARD
    for y in 1..=8 {
        commands.spawn(Text2dBundle {
            text: Text::from_section(y.to_string(), text_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(Vec3::new(
                -SQUARE_SIZE / 2.0,
                y as f32 * SQUARE_SIZE - SQUARE_SIZE / 2.0,
                0.0,
            )),
            ..Default::default()
        });
    }

    load_position_from_fen(&mut commands, &mut piece_locations, asset_server);
}

fn my_startup_system(_commands: Commands) {
    println!("My startup system");
}

fn main() {
    env::set_var("RUST_LOG", "info");
    // env::set_var("RUST_BACKTRACE", "1");

    App::new()
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ChessPieces::default())
        .add_system(setup.on_startup())
        .add_event::<MoveEvent>()
        .add_system(my_startup_system.on_startup())
        .add_systems((bevy::window::close_on_esc, mouse_button_input))
        .run();
}
