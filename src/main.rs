use macroquad::prelude::*;

#[derive(Clone, PartialEq)]
struct TileWorld {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    filled: TileName,
    color: Color,
}

impl Tile {
    fn new(filled: TileName) -> Self {
        // Match the color to the correct color
        let color = match filled {
            TileName::Air => Color::new(0.0, 0.0, 0.0, 1.0),
            TileName::Sand => Color::new(0.99, 0.98, 0.0, 1.0),
            TileName::Stone => Color::new(0.502, 0.502, 0.502, 1.0),
        };
        Self { filled, color }
    }
    fn match_color(&self) -> Color {
        match self.filled {
            TileName::Air => Color::new(0.0, 0.0, 0.0, 1.0),
            TileName::Sand => Color::new(0.99, 0.98, 0.0, 1.0),
            TileName::Stone => Color::new(0.502, 0.502, 0.502, 1.0),
        }
    }
    fn fall(&mut self, tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
        if let Some(row) = tiles.get_mut(x) {
            if let Some(tile_below) = row.get_mut(y + 1) {
                if self.filled != TileName::Air && tile_below.filled == TileName::Air {
                    tile_below.filled = TileName::Sand;
                    tiles[x][y].filled = TileName::Air;
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TileName {
    Air,
    Sand,
    Stone,
}

impl TileWorld {
    fn create(&mut self) -> &mut Self {
        self.tiles = Vec::with_capacity(self.width);
        for _ in 0..self.width {
            let mut row = Vec::with_capacity(self.height);
            for _ in 0..self.height {
                row.push(Tile::new(TileName::Air));
            }
            self.tiles.push(row);
        }
        self
    }
    fn update_sim(&mut self) {
        for (x_pos, row) in self.tiles.clone().iter_mut().enumerate() {
            for (y_pos, tile) in row.iter_mut().enumerate() {
                tile.fall(&mut self.tiles, x_pos, y_pos);
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
    const TILE_SIZE: f32 = 20.0;
    let screen_width: usize = screen_width() as usize;
    let screen_height: usize = screen_height() as usize;

    let mut world = TileWorld {
        tiles: vec![vec![Tile {
            filled: TileName::Air,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
        }]],
        width: screen_width / TILE_SIZE as usize,
        height: screen_height / TILE_SIZE as usize,
    };

    let world = world.create();

    let mut image = Image::gen_image_color(
        screen_width as u16,
        screen_height as u16,
        Color::new(0.0, 0.0, 0.0, 1.0),
    );
    let texture = Texture2D::from_image(&image);


    loop {
        clear_background(BLACK);
        let (mouse_x, mouse_y) = mouse_position();
        let (downscalled_mouse_x, downscalled_mouse_y) = (mouse_x / TILE_SIZE, mouse_y / TILE_SIZE);

        world.update_sim();

        // Handle mouse events
        if is_mouse_button_down(MouseButton::Left) {
            world.tiles[downscalled_mouse_x as usize][downscalled_mouse_y as usize].filled =
                TileName::Sand;
        }
        if is_mouse_button_down(MouseButton::Right) {
            world.tiles[downscalled_mouse_x as usize][downscalled_mouse_y as usize].filled =
                TileName::Air;
        }

        // Construct tile image
        for (x_pos, row) in world.tiles.iter().enumerate() {
            for (y_pos, tile) in row.iter().enumerate() {
                let x = x_pos as f32 * TILE_SIZE;
                let y = y_pos as f32 * TILE_SIZE;

                for x_pix in 0..(TILE_SIZE) as u32 {
                    for y_pix in 0..(TILE_SIZE) as u32 {
                        if image.get_pixel(x as u32 + x_pix, y as u32 + y_pix) != tile.match_color()
                        {
                            image.set_pixel(x as u32 + x_pix, y as u32 + y_pix, tile.match_color());
                        }
                    }
                }
            }
        }

        // Draw tile image
        texture.update(&image);
        draw_texture(texture, 0.0, 0.0, WHITE);

        // Draw texture
        draw_rectangle(
            mouse_x - TILE_SIZE / 2.0,
            mouse_y - TILE_SIZE / 2.0,
            TILE_SIZE,
            TILE_SIZE,
            Color::new(1.0, 0.0, 0.0, 1.0),
        );

        // Draw fps text
        let fps: &str = &get_fps().to_string();
        draw_text(fps, 10.0, 15.0, 25.0, WHITE);

        next_frame().await
    }
}
