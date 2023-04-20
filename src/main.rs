use macroquad::prelude::*;

#[derive(PartialEq, Clone)]
struct TileWorld {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    filled: bool,
}

impl Tile {
    fn new(filled: bool) -> Self {
        Self { filled }
    }
}

impl TileWorld {
    fn create(&mut self) -> &mut Self {
        self.tiles = Vec::with_capacity(self.width);
        for _ in 0..self.width {
            let mut row = Vec::with_capacity(self.height);
            for _ in 0..self.height {
                row.push(Tile::new(false));
            }
            self.tiles.push(row);
        }
        self
    }
    fn set_tile(&mut self, x: usize, y: usize, filled: bool) {
        self.tiles[x][y].filled = filled;
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
    const PIX_SIZE: f32 = 20.0;
    let screen_width: usize = screen_width() as usize;
    let screen_height: usize = screen_height() as usize;

    let mut world = TileWorld {
        tiles: vec![vec![Tile { filled: false }]],
        width: screen_width / PIX_SIZE as usize,
        height: screen_height / PIX_SIZE as usize,
    };

    let world = world.create();

    let mut image = Image::gen_image_color(
        screen_width as u16,
        screen_height as u16,
        Color::new(0.0, 0.0, 0.0, 1.0),
    );
    let texture = Texture2D::from_image(&image);

    let mut last_world: TileWorld = world.clone();
    last_world.set_tile(0, 0, true);

    loop {
        clear_background(BLACK);
        let (mouse_x, mouse_y) = mouse_position();
        let mut change = false;
        let (downscalled_mouse_x, downscalled_mouse_y) = (mouse_x / PIX_SIZE, mouse_y / PIX_SIZE);

        // Handle mouse events
        if is_mouse_button_down(MouseButton::Left) {
            world.set_tile(
                downscalled_mouse_x as usize,
                downscalled_mouse_y as usize,
                true,
            );
            change = true;
        }
        if is_mouse_button_down(MouseButton::Right) {
            world.set_tile(
                downscalled_mouse_x as usize,
                downscalled_mouse_y as usize,
                false,
            );
            change = true;
        }

        // Construct tile image
        if &mut last_world != world || change {
            for (x_pos, row) in world.tiles.iter().enumerate() {
                for (y_pos, tile) in row.iter().enumerate() {
                    let x = x_pos as f32 * PIX_SIZE;
                    let y = y_pos as f32 * PIX_SIZE;

                    let color: Color = if tile.filled {
                        Color::new(0.99, 0.98, 0.0, 1.0)
                    } else {
                        Color::new(0.0, 0.0, 0.0, 1.0)
                    };
                    for x_pix in 0..(PIX_SIZE) as u32 {
                        for y_pix in 0..(PIX_SIZE) as u32 {
                            if image.get_pixel(x as u32 + x_pix, y as u32 + y_pix) != color {
                                image.set_pixel(x as u32 + x_pix, y as u32 + y_pix, color);
                            }
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
            mouse_x - PIX_SIZE / 2.0,
            mouse_y - PIX_SIZE / 2.0,
            PIX_SIZE,
            PIX_SIZE,
            Color::new(1.0, 0.0, 0.0, 1.0),
        );

        // Draw fps text
        let fps: &str = &get_fps().to_string();
        draw_text(fps, 10.0, 15.0, 25.0, WHITE);

        if &mut last_world != world || change {
            last_world = world.clone();
        }

        next_frame().await
    }
}
