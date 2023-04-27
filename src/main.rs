use macroquad::prelude::*;

use std::fmt;
use std::time::*;

#[derive(Clone, PartialEq)]
struct TileWorld {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    filled: TileType,
    color: Color,

    is_falling: bool,
}

impl Tile {
    fn new(filled: TileType) -> Self {
        // Match the color to the correct color
        let color = match filled {
            TileType::Air => Color::new(0.0, 0.0, 0.0, 1.0),
            TileType::Sand => Color::new(0.99, 0.98, 0.0, 1.0),
            TileType::Water => Color::new(0.0, 0.0, 0.8, 1.0),
            TileType::Stone => Color::new(0.502, 0.502, 0.502, 1.0),
        };
        Self {
            filled,
            color,
            is_falling: false,
        }
    }
    fn match_color(&self) -> Color {
        match self.filled {
            TileType::Air => Color::new(0.0, 0.0, 0.0, 1.0),
            TileType::Sand => Color::new(0.99, 0.98, 0.0, 1.0),
            TileType::Water => Color::new(0.0, 0.0, 0.8, 1.0),
            TileType::Stone => Color::new(0.502, 0.502, 0.502, 1.0),
        }
    }

    fn update_tile_pos(
        &mut self,
        tiles: &mut Vec<Vec<Tile>>,
        last_world_tiles: &mut Vec<Vec<Tile>>,
        tile_type: TileType,
        x: usize,
        y: usize,
    ) {
        if tiles[x][y].filled == last_world_tiles[x][y].filled {
            tiles[x][y].is_falling = false;
        }
        match tile_type {
            TileType::Air => {}
            TileType::Sand => {
                let check = self.move_tile(tiles, tile_type, x, y, x, y + 1, false);

                if !check {
                    if rand::RandomRange::gen_range(0, 10) > 5 {
                        if self.move_tile(tiles, tile_type, x, y, x - 1, y + 1, true) {
                            self.move_tile(tiles, tile_type, x, y, x - 1, y + 1, false);
                        }
                    } else {
                        if self.move_tile(tiles, tile_type, x, y, x + 1, y + 1, true) {
                            self.move_tile(tiles, tile_type, x, y, x + 1, y + 1, false);
                        }
                    }
                }
            }
            TileType::Water => {
                let check = self.move_tile(tiles, tile_type, x, y, x, y + 1, false);

                if !check {
                    if rand::RandomRange::gen_range(0, 10) > 5 {
                        if self.move_tile(tiles, tile_type, x, y, x - 1, y, true) {
                            self.move_tile(tiles, tile_type, x, y, x - 1, y, false);
                        }
                    } else {
                        if self.move_tile(tiles, tile_type, x, y, x + 1, y, true) {
                            self.move_tile(tiles, tile_type, x, y, x + 1, y, false);
                        }
                    }
                }
            }
            TileType::Stone => {}
        }
    }

    fn move_tile(
        &mut self,
        tiles: &mut Vec<Vec<Tile>>,
        tile_type: TileType,
        x: usize,
        y: usize,
        target_x: usize,
        target_y: usize,
        check: bool,
    ) -> bool {
        if target_x >= tiles.len() as usize {
            return false;
        }

        if target_y >= tiles[0].len() as usize {
            return false;
        }

        let target_tile = &mut tiles[target_x][target_y];

        if target_tile.filled == TileType::Air {
            if !check {
                target_tile.filled = tile_type;
                tiles[x][y].filled = TileType::Air;

                tiles[x][y].is_falling = true;
            }
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileType {
    Air,
    Sand,
    Water,
    Stone,
}
impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            TileType::Air => "Air",
            TileType::Sand => "Sand",
            TileType::Water => "Water",
            TileType::Stone => "Stone",
        };
        write!(f, "{}", display)
    }
}

impl TileWorld {
    fn create(&mut self) -> &mut Self {
        self.tiles = Vec::with_capacity(self.width);
        for _ in 0..self.width {
            let mut row = Vec::with_capacity(self.height);
            for _ in 0..self.height {
                row.push(Tile::new(TileType::Air));
            }
            self.tiles.push(row);
        }
        self
    }
    fn update_sim(&mut self, mut last_world_tiles: Vec<Vec<Tile>>) {
        for (x_pos, row) in self.tiles.clone().iter_mut().enumerate() {
            for (y_pos, tile) in row.iter_mut().enumerate() {
                if vec![TileType::Sand, TileType::Water].contains(&tile.filled) {
                    tile.update_tile_pos(
                        &mut self.tiles,
                        &mut last_world_tiles,
                        tile.filled.clone(),
                        x_pos,
                        y_pos,
                    );
                }
            }
        }
    }
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
    const TILE_SIZE: f32 = 5.0;
    let screen_width: usize = screen_width() as usize;
    let screen_height: usize = screen_height() as usize;

    let mut brush_type: Option<TileType> = Some(TileType::Sand);
    let mut brush_size = 5.0; // Must be odd

    let mut world = TileWorld {
        tiles: vec![vec![Tile {
            filled: TileType::Air,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            is_falling: false,
        }]],
        width: screen_width / TILE_SIZE as usize,
        height: screen_height / TILE_SIZE as usize,
    };

    let world = world.create();
    let mut last_world = world.clone();

    let mut image = Image::gen_image_color(
        screen_width as u16 / TILE_SIZE as u16,
        screen_height as u16 / TILE_SIZE as u16,
        Color::new(0.0, 0.0, 0.0, 1.0),
    );
    let texture = Texture2D::from_image(&image);
    Texture2D::set_filter(&texture, FilterMode::Nearest);

    loop {
        clear_background(BLACK);
        let (mouse_x, mouse_y) = mouse_position();
        let (downscalled_mouse_x, downscalled_mouse_y) = (mouse_x / TILE_SIZE, mouse_y / TILE_SIZE);
        let fps: &str = &get_fps().to_string();

        let update_sim_time = Instant::now();

        world.update_sim(last_world.tiles);
        last_world = world.clone();

        let update_sim_time = &update_sim_time.elapsed().as_secs_f32().to_string();

        // Handle input events
        if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
            let tile_type = if is_mouse_button_down(MouseButton::Left) {
                brush_type.clone().unwrap()
            } else {
                TileType::Air
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
        } else if mouse_wheel() < (0.0, 0.0) && brush_size != 0.0 {
            brush_size -= 1.0;
        }
        if is_key_pressed(KeyCode::E) {
            brush_type = match brush_type.unwrap() {
                TileType::Air => None,
                TileType::Sand => Some(TileType::Water),
                TileType::Water => Some(TileType::Stone),
                TileType::Stone => Some(TileType::Sand),
            };
        }

        let tile_image_time = Instant::now();
        // Construct tile image
        for (x, row) in world.tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                image.set_pixel(x as u32, y as u32, tile.match_color());
            }
        }
        let tile_image_time = &tile_image_time.elapsed().as_secs_f32().to_string();

        let draw_texture_time = Instant::now();
        // Draw tile image
        texture.update(&image);
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width as f32, screen_height as f32)),
                ..Default::default()
            },
        );
        let draw_texture_time = &draw_texture_time.elapsed().as_secs_f32().to_string();

        // Draw fps and rendering time text
        draw_text(fps, 10.0, 15.0, 25.0, WHITE);
        draw_text(
            &("Update sim: ".to_owned() + update_sim_time),
            10.0,
            35.0,
            25.0,
            WHITE,
        );
        draw_text(
            &("Tile image creation: ".to_owned() + tile_image_time),
            10.0,
            55.0,
            25.0,
            WHITE,
        );
        draw_text(
            &("Draw texture: ".to_owned() + draw_texture_time),
            10.0,
            75.0,
            25.0,
            WHITE,
        );
        draw_text(
            &("Brush selected: ".to_owned() + &brush_type.unwrap().to_string()),
            10.0,
            95.0,
            25.0,
            WHITE,
        );
        draw_text(
            &("Brush size: ".to_owned() + &brush_size.to_string()),
            10.0,
            115.0,
            25.0,
            WHITE,
        );

        next_frame().await
    }
}
