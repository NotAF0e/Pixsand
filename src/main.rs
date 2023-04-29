mod sim;

use macroquad::input::KeyCode;
use macroquad::prelude::*;
use macroquad::ui::*;
use std::{thread, time::*};

fn construct_frame(world: &mut sim::TileWorld, texture: Texture2D, image: &mut Image) {
    // Construct tile image
    for (x, row) in world.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            image.set_pixel(x as u32, y as u32, tile.match_color());
        }
    }

    // Draw tile image
    texture.update(&image);
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width() as f32, screen_height() as f32)),
            ..Default::default()
        },
    );
}

fn handle_input(
    world: &mut sim::TileWorld,
    mut brush_type: Option<sim::TileType>,
    mut brush_size: f32,
    downscalled_mouse_x: f32,
    downscalled_mouse_y: f32,
) -> (Option<sim::TileType>, f32) {
    if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
        let tile_type = if is_mouse_button_down(MouseButton::Left) {
            brush_type.clone().unwrap()
        } else {
            sim::TileType::Air
        };

        let brush_half_size = (brush_size / 2.0) as usize;

        for x_displace in 0..brush_size as usize {
            for y_displace in 0..brush_size as usize {
                let x = downscalled_mouse_x as usize + x_displace - brush_half_size;
                let y = downscalled_mouse_y as usize + y_displace - brush_half_size;

                if let Some(row) = world.tiles.get_mut(x) {
                    if let Some(tile) = row.get_mut(y) {
                        tile.filled = tile_type;
                    }
                }
            }
        }
    }
    if mouse_wheel() > (0.0, 0.0) {
        brush_size += 1.0;
    } else if mouse_wheel() < (0.0, 0.0) && brush_size != 1.0 {
        brush_size -= 1.0;
    }
    if is_key_pressed(KeyCode::E) {
        brush_type = match brush_type.unwrap() {
            sim::TileType::Air => None,
            sim::TileType::Sand => Some(sim::TileType::Water),
            sim::TileType::Water => Some(sim::TileType::Stone),
            sim::TileType::Stone => Some(sim::TileType::Sand),
        };
    }

    return (brush_type, brush_size);
}

#[derive(PartialEq)]
enum GameState {
    Menu,
    LoadSave,
    Playing,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pixsand".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    // Constant unchangable values
    const TILE_SIZE: f32 = 3.0;

    // Values changeable by player
    let mut game_state: GameState = GameState::Menu;
    let mut brush_type = Some(sim::TileType::Sand);
    let mut brush_size = 7.0;

    // World setup
    let mut world = sim::TileWorld {
        tiles: vec![vec![sim::Tile {
            filled: sim::TileType::Air,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            is_falling: false,
        }]],
        width: (screen_width() / TILE_SIZE) as usize,
        height: (screen_height() / TILE_SIZE) as usize,
    };

    let world = world.create();
    let mut last_world = world.clone();

    // Image setup for frame construction
    let mut image = Image::gen_image_color(
        screen_width() as u16 / TILE_SIZE as u16,
        screen_height() as u16 / TILE_SIZE as u16,
        Color::new(0.0, 0.0, 0.0, 1.0),
    );
    let texture = Texture2D::from_image(&image);
    Texture2D::set_filter(&texture, FilterMode::Nearest);

    loop {
        if game_state == GameState::Menu {
            root_ui().label(
                vec2(
                    (screen_width() / 2.0) as f32,
                    (screen_height() / 2.0) as f32 + 100.0,
                ),
                "Menu",
            );

            if root_ui().button(
                vec2(
                    (screen_width() / 2.0) as f32,
                    (screen_height() / 2.0) as f32,
                ),
                "Play",
            ) {
                thread::sleep(Duration::from_millis(100));
                game_state = GameState::Playing;
            }
        } else if game_state == GameState::LoadSave {
            todo!();
        } else if game_state == GameState::Playing {
            let (mouse_x, mouse_y) = mouse_position();
            let (downscalled_mouse_x, downscalled_mouse_y) =
                (mouse_x / TILE_SIZE, mouse_y / TILE_SIZE);

            (brush_type, brush_size) = handle_input(
                world,
                brush_type,
                brush_size,
                downscalled_mouse_x,
                downscalled_mouse_y,
            );

            // Updating the simulation
            let update_sim_time = Instant::now();
            world.update_sim(last_world.tiles);
            last_world = world.clone();
            let update_sim_time = &update_sim_time.elapsed().as_secs_f32().to_string();

            // Contructing the frame for rendering
            let construct_frame_time = Instant::now();
            construct_frame(world, texture, &mut image);
            let construct_frame_time = &construct_frame_time.elapsed().as_secs_f32().to_string();

            // Draw fps and rendering time text
            let frame_time: &str = &get_frame_time().to_string();
            let debug_text: Vec<String> = vec![
                frame_time.to_string(),
                "Update sim: ".to_owned() + update_sim_time,
                "Construct frame: ".to_owned() + construct_frame_time,
                "Brush selected: ".to_owned() + &brush_type.unwrap().to_string(),
                "Brush size: ".to_owned() + &brush_size.to_string(),
            ];
            let mut text_displacement = 15.0;
            for text in debug_text {
                draw_text(&text, 0.0, text_displacement, 25.0, WHITE);
                text_displacement += 20.0;
            }
        }

        next_frame().await
    }
}
