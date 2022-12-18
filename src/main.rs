use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    sprite::MaterialMesh2dBundle,
    winit::WinitSettings,
};
use bevy_inspector_egui::WorldInspectorPlugin;

const SQUARE_SIZE: f32 = 60.0;
const PIECE_SIZE: f32 = 1.0;
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

impl Default for ChessPieces {
    fn default() -> Self {
        Self { pieces: [0; 64] }
    }
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
trait GetPieceType {
    fn get_piece_type(&self, index: usize) -> (PieceType, PieceColor);
}

impl GetPieceType for ChessPieces {
    fn get_piece_type(&self, index: usize) -> (PieceType, PieceColor) {
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
}

trait GetPieceImage {
    fn get_piece_image(&self, index: usize) -> &str;
}

impl GetPieceImage for ChessPieces {
    fn get_piece_image(&self, index: usize) -> &str {
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

trait GetPieceColor {
    fn get_piece_color(&self, color: PieceColor) -> &'static str;
}

impl GetPieceColor for ChessPieces {
    fn get_piece_color(&self, color: PieceColor) -> &'static str {
        match color {
            PieceColor::White => "white",
            PieceColor::Black => "black",
        }
    }
}

trait GetPieceName {
    fn get_piece_name(&self, piece: PieceType) -> &'static str;
}

impl GetPieceName for ChessPieces {
    fn get_piece_name(&self, piece: PieceType) -> &'static str {
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
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut piece_locations: ResMut<ChessPieces>,
) {
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
                .with_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
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
            text: Text::from_section(y.to_string(), text_style.clone()).with_alignment(
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_translation(Vec3::new(
                -SQUARE_SIZE / 2.0,
                y as f32 * SQUARE_SIZE - SQUARE_SIZE / 2.0,
                0.0,
            )),
            ..Default::default()
        });
    }

    // Setup the piece locations in a 1d array
    //let mut piece_locations: [u8; 64] = [0; 64];
    // Initialize the board with the FEN string

    // TODO: Setup the pieces from the FEN string
    load_position_from_fen(&mut commands, &mut piece_locations, asset_server);
}

fn load_position_from_fen(
    commands: &mut Commands,
    piece_locations: &mut [u8; 64],
    asset_server: Res<AssetServer>,
) {
    // Initialize the board with the FEN string
    let mut x: usize = 0;
    let mut y: usize = 0;
    println!("Loading position from FEN...");
    println!("FEN: {}", START_FEN);
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
        println!("{:?} | {:?} = {}", piece_type, piece_color, piece);

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
    println!("PIECE LOCATIONS:\n{:?}", piece_locations)
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
                .with_scale(Vec3::splat(PIECE_SIZE)),
            ..Default::default()
        })
        .insert(Piece);
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

#[allow(clippy::too_many_arguments)]
fn my_cursor_system(
    // need to get window dimensions and cursor position
    wnds: Res<Windows>,
    // need to get mouse button input
    mouse_button_input: Res<Input<MouseButton>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
    // query to get all sprites with the Piece component
    mut q_pieces: Query<(&mut Transform, With<Piece>)>,
    // query to get all sprites with the DraggedPiece component
    mut q_dragged_pieces: Query<(Entity, &mut Transform, With<DraggedPiece>, Without<Piece>)>,
    // query to get the ChessPiece resource
    chess_pieces: Res<ChessPieces>,
    // asset server to get the piece images
    asset_server: Res<AssetServer>,
    // commands to spawn new sprites
    mut commands: Commands,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width(), wnd.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate().round();

        // eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);

        // Now that the translated cursor position is known, start handling piece dragging
        // If the left mouse button is pressed, check if a piece is under the cursor
        // TODO: Finish this function
        match mouse_button_input.pressed(MouseButton::Left) {
            // If the left mouse button is pressed, check if there is a piece under the cursor
            true => {
                // If there is no piece being dragged, check if there is a piece under the cursor. If there is a piece under the cursor, spawn a new piece at the cursor position and add the DraggedPiece component to it. If there is no piece under the cursor, do nothing.
                match q_dragged_pieces.iter_mut().next().is_none() {
                    true => println!("No piece being dragged"),
                    false => println!("Piece being dragged"),
                }
            }
            // If the left mouse button is not pressed, check if there is a piece being dragged. If there is a piece being dragged, check if there is a square under the cursor. If there is a square under the cursor, move the piece to the square. If there is no square under the cursor, move the piece back to its original square. Then, despawn the piece with the DraggedPiece component.
            false => match q_dragged_pieces.iter_mut().next().is_none() {
                true => println!("No piece being was being dragged"),
                false => println!("Piece was being dragged"),
            },
        }
    }
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
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(WorldInspectorPlugin::new())
        // .add_system(mouse_click)
        .add_system(my_cursor_system)
        // Antialiasing
        .insert_resource(Msaa { samples: 4 })
        .add_system(bevy::window::close_on_esc)
        .insert_resource(ChessPieces::default())
        .add_event::<MoveEvent>()
        .add_startup_system(setup)
        .run();
}
