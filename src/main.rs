// struct TileWorld {
//     tiles: Vec<Vec<Tile>>,
//     width: usize,
//     height: usize,
// }

// #[derive(Clone, Debug)]
// struct Tile {
//     filled: bool,
// }

// impl Iterator for Tile {
//     type Item = Tile;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

// impl Tile {
//     fn new(filled: bool) -> Self {
//         Self { filled }
//     }
// }

// impl TileWorld {
//     fn create(&mut self) -> &mut Self {
//         let mut rng = rand::thread_rng();

//         self.tiles = vec![vec![]; self.height];
//         for x in 0..self.height {
//             for _y in 0..self.width {
//                 self.tiles[x].push(Tile::new(rng.gen_bool(random())));
//             }
//         }

//         for row in &self.tiles {
//             for tile in row {}
//         }
//         return self;
//     }

//     fn print(&mut self) {
//         let mut tiles_to_print = "".to_string();

//         for row in &self.tiles {
//             for tile in row {
//                 if tile.filled {
//                     tiles_to_print += "█";
//                 } else {
//                     tiles_to_print += " ";
//                 }
//             }
//             tiles_to_print += "\n";
//         }
//         println!("{}", tiles_to_print);
//     }
// }

// fn main() {
//     let mut world = TileWorld {
//         tiles: vec![vec![Tile { filled: false }]],
//         width: 100,
//         height: 100,
//     };

//     let mut world = world.create();
//     world.print();
// }

// use macroquad::prelude::*;

// #[derive(PartialEq, Clone)]
// struct TileWorld {
//     tiles: Vec<Vec<Tile>>,
//     width: usize,
//     height: usize,
// }

// #[derive(Clone, Debug, PartialEq)]
// struct Tile {
//     filled: bool,
// }

// impl Iterator for Tile {
//     type Item = Tile;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

// impl Tile {
//     fn new(filled: bool) -> Self {
//         Self { filled }
//     }
// }

// impl TileWorld {
//     fn create(&mut self) -> &mut Self {
//         self.tiles = Vec::with_capacity(self.width);
//         for _ in 0..self.width {
//             let mut row = Vec::with_capacity(self.height);
//             for _ in 0..self.height {
//                 row.push(Tile::new(true));
//             }
//             self.tiles.push(row);
//         }
//         self
//     }
// }

// #[macroquad::main("Pixsand")]
// async fn main() {
//     let pix_size: f32 = 5.0;

//     let mut world = TileWorld {
//         tiles: vec![vec![Tile { filled: false }]],
//         width: 1920,
//         height: 1080,
//     };
//     let mut world = world.create();

//     loop {
//         clear_background(BLACK);

//         for (x_pos, row) in world.tiles.iter().enumerate() {
//             for (y_pos, tile) in row.iter().enumerate() {
//                 if tile.filled {
//                     draw_rectangle(
//                         x_pos as f32,
//                         y_pos as f32,
//                         pix_size,
//                         pix_size,
//                         Color::new(0.99, 0.98, 0.0, 1.0),
//                     );
//                 }
//             }
//         }
//         // Optimised version of the above is:

//         let (mouse_x, mouse_y) = mouse_position();
//         draw_rectangle(
//             mouse_x,
//             mouse_y,
//             pix_size,
//             pix_size,
//             Color::new(1.0, 0.0, 0.0, 1.0),
//         );

//         next_frame().await
//     }
// }

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

#[macroquad::main("Pixsand")]
async fn main() {
    let pix_size: f32 = 5.0;
    let mut world = TileWorld {
        tiles: vec![vec![Tile { filled: false }]],
        width: 1920,
        height: 1080,
    };
    let mut world = world.create();

    let mut image = Image::gen_image_color(
        world.width as u16,
        world.height as u16,
        Color::new(0.0, 0.0, 0.0, 1.0),
    );
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);
        let (mouse_x, mouse_y) = mouse_position();

        for (x_pos, row) in world.tiles.iter().enumerate() {
            for (y_pos, tile) in row.iter().enumerate() {
                if tile.filled {
                    let x = x_pos as f32;
                    let y = y_pos as f32;
                    let color = Color::new(0.99, 0.98, 0.0, 1.0);
                    if image.get_pixel(x as u32, y as u32) != color {
                        // Creates a square of pixels
                        for x_square in 0..pix_size as u32 {
                            for y_square in 0..pix_size as u32 {
                                image.set_pixel(
                                    (x + x_square as f32 - pix_size / 2.0) as u32,
                                    (y + y_square as f32 - pix_size / 2.0) as u32,
                                    color,
                                );
                            }
                        }
                    }
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            world.set_tile(mouse_x as usize, mouse_y as usize, true);
        }
        if is_mouse_button_down(MouseButton::Right) {
            world.set_tile(mouse_x as usize, mouse_y as usize, false);
        }

        texture.update(&image);

        draw_texture(texture, 0.0, 0.0, WHITE);

        draw_rectangle(
            mouse_x - pix_size / 2.0,
            mouse_y - pix_size / 2.0,
            pix_size,
            pix_size,
            Color::new(1.0, 0.0, 0.0, 1.0),
        );

        next_frame().await
    }
}
