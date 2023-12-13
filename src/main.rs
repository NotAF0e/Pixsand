mod sim;
use std::env;

use bevy::{
    diagnostic::*,
    input::{keyboard, mouse::MouseWheel},
    prelude::*,
    render::color::Color,
    window::{PresentMode, Window},
};
use bevy_pixel_buffer::prelude::*;
use sim::{Tile, TileType};

// fn handle_input(
//     world: &mut sim::TileWorld,
//     mut brush_type: Option<sim::TileType>,
//     mut brush_size: f32,
//     buttons: Res<Input<MouseButton>>,
// ) -> (Option<sim::TileType>, f32) {
//     if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
//         let tile_type = if is_mouse_button_down(MouseButton::Left) {
//             brush_type.clone().unwrap()
//         } else {
//             sim::TileType::Air
//         };
//         let brush_half_size = (brush_size / 2.0) as usize;
//         for x_displace in 0..brush_size as usize {
//             for y_displace in 0..brush_size as usize {
//                 let x = downscalled_mouse_x as usize + x_displace - brush_half_size;
//                 let y = downscalled_mouse_y as usize + y_displace - brush_half_size;
//                 if let Some(row) = world.tiles.get_mut(x) {
//                     if let Some(tile) = row.get_mut(y) {
//                         tile.filled = tile_type;
//                     }
//                 }
//             }
//         }
//     }
//     if mouse_wheel() > (0.0, 0.0) {
//         brush_size += 1.0;
//     } else if mouse_wheel() < (0.0, 0.0) && brush_size != 1.0 {
//         brush_size -= 1.0;
//     }
//     if is_key_pressed(KeyCode::E) {
//         brush_type = match brush_type.unwrap() {
//             sim::TileType::Air => None,
//             sim::TileType::Sand => Some(sim::TileType::Water),
//             sim::TileType::Water => Some(sim::TileType::Stone),
//             sim::TileType::Stone => Some(sim::TileType::Sand),
//         };
//     }
//     return (brush_type, brush_size);
// }

#[derive(Resource)]
struct BrushSize {
    value: f32,
}

fn main() {
    let size = PixelBufferSize {
        size: UVec2::new(1920 / 4, 1080 / 4),
        pixel_size: UVec2::new(4, 4),
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pixsand".into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                present_mode: PresentMode::Immediate,
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(PixelBufferPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, pixel_buffer_setup(size))
        .add_systems(Startup, startup)
        .add_systems(Update, handle_input)
        .add_systems(Update, update_frame)
        .add_systems(Update, update_sim)
        .run()
}

fn startup(mut commands: Commands) {
    // World setup
    let mut tile_world = sim::TileWorld {
        tiles: vec![vec![sim::Tile {
            filled: sim::TileType::Air,
            color: Color::rgba(1.0, 1.0, 0.0, 1.0),
        }]],
        width: (1920 / 4) as usize,
        height: (1080 / 4) as usize,
    };
    let tile_world = tile_world.create();
    commands.insert_resource(tile_world.clone());

    // Brush size and type setup
    let brush_size = BrushSize { value: 8.0 };
    let brush_type = TileType::Sand;
    commands.insert_resource(brush_size);
    commands.insert_resource(brush_type);
}

fn handle_input(
    mut tile_world: ResMut<sim::TileWorld>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    keys: Res<Input<KeyCode>>,
    window: Query<&Window>,
    mut brush_type: ResMut<TileType>,
    mut brush_size: ResMut<BrushSize>,
) {
    let mouse_pos = window.single().cursor_position();

    if let Some(mouse_position) = mouse_pos {
        if mouse_buttons.pressed(MouseButton::Left) {
            let tile_type = *brush_type;
            modify_tiles(&mut tile_world, mouse_position, tile_type, &brush_size);
        } else if mouse_buttons.pressed(MouseButton::Right) {
            modify_tiles(&mut tile_world, mouse_position, TileType::Air, &brush_size);
        }
    }

    if keys.just_pressed(KeyCode::E) {
        *brush_type = match *brush_type {
            TileType::Air => TileType::Sand,
            TileType::Sand => TileType::Water,
            TileType::Water => TileType::Stone,
            TileType::Stone => TileType::Sand, // Change this cycle as needed
        };
    }

    let scroll = mouse_wheel.read();
    for ev in scroll {
        if ev.y > 0.0 && brush_size.value < 20.0 {
            brush_size.value += 1.0;
        } else if ev.y < 0.0 && brush_size.value > 1.0 {
            brush_size.value -= 1.0;
        }
    }
}

fn modify_tiles(
    tile_world: &mut ResMut<sim::TileWorld>,
    mouse_pos: Vec2,
    tile_type: sim::TileType,
    brush_size: &ResMut<BrushSize>,
) {
    let scalled_mouse_pos = mouse_pos / 4.0;
    let brush_half_size = (brush_size.value / 2.0) as usize;

    for x_displace in 0..brush_size.value as usize {
        for y_displace in 0..brush_size.value as usize {
            let x = (scalled_mouse_pos.x + x_displace as f32 - brush_half_size as f32)
                .clamp(0.0, 1920.0 / 4.0) as usize;
            let y = (scalled_mouse_pos.y + y_displace as f32 - brush_half_size as f32)
                .clamp(0.0, 1080.0 / 4.0) as usize;

            if let Some(row) = tile_world.tiles.get_mut(x) {
                if let Some(tile) = row.get_mut(y) {
                    tile.filled = tile_type;
                }
            }
        }
    }
}

fn update_frame(mut pb: QueryPixelBuffer, tile_world: ResMut<sim::TileWorld>) {
    pb.frame().per_pixel(|xy, _| {
        Pixel::as_color(
            tile_world.tiles[xy.x as usize][xy.y as usize]
                .filled
                .color()
                .into(),
        )
    });
}

fn update_sim(mut tile_world: ResMut<sim::TileWorld>) {
    tile_world.update_sim();
}
