use bevy::{
    prelude::{ResMut, Resource},
    render::color::Color,
};
use rand::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq, Resource)]
pub struct TileWorld {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}
#[derive(Clone, Debug, PartialEq, Resource)]
pub struct Tile {
    pub filled: TileType,
    pub color: Color,
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = (self.filled.to_string(), self.filled.color());
        write!(f, "{:?}", display)
    }
}
impl Tile {
    pub fn new(filled: TileType) -> Self {
        // Match the color to the correct color
        let color = match filled {
            TileType::Air => Color::rgba(0.0, 0.0, 0.0, 1.0),
            TileType::Sand => Color::rgba(0.99, 0.98, 0.0, 1.0),
            TileType::Water => Color::rgba(0.0, 0.0, 0.8, 1.0),
            TileType::Stone => Color::rgba(0.502, 0.502, 0.502, 1.0),
        };
        Self { filled, color }
    }
    pub fn update_tile_pos(&mut self, tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
        if self.filled == TileType::Air {
            return;
        }
        let rng = rand::thread_rng();

        match self.filled {
            TileType::Sand => self.update_sand(rng, tiles, x, y),
            TileType::Water => self.update_water(rng, tiles, x, y),
            _ => {} // Other TileTypes can be handled similarly
        }
    }

    fn update_sand(&mut self, mut rng: ThreadRng, tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
        let check = if self.tile_type_safe_index(tiles, x as isize, y as isize + 1) == TileType::Air
        {
            true
        } else {
            false
        };
        let dir: isize = if !check {
            if rng.gen_range(0..10) > 5 {
                if self.tile_type_safe_index(tiles, x as isize + 1, y as isize + 1) == TileType::Air
                {
                    1
                } else {
                    0
                }
            } else {
                if self.tile_type_safe_index(tiles, x as isize - 1, y as isize + 1) == TileType::Air
                {
                    -1
                } else {
                    0
                }
            }
        } else {
            0
        };
        let (target_x, target_y) = ((x as isize + dir) as usize, y + 1);

        if self.is_within_bounds(tiles, target_x, target_y) {
            self.move_tile(tiles, x, y, target_x, target_y);
        }
    }

    fn update_water(&mut self, mut rng: ThreadRng, tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
        let mut dis: isize = 1;
        let (y_dis, check) =
            if self.tile_type_safe_index(tiles, x as isize, y as isize + 1) == TileType::Air {
                (1, true)
            } else {
                (0, false)
            };
        let dir = rng.gen_range(0..10) > 5;

        if !check {
            if self.tile_type_safe_index(tiles, x as isize + 1, y as isize) == TileType::Air
                || self.tile_type_safe_index(tiles, x as isize - 1, y as isize) == TileType::Air
            {
                dis = rng.gen_range(2..6);
            }
            if dir {
                if self.tile_type_safe_index(tiles, x as isize + dis, y as isize) == TileType::Air {
                    dis = dis
                } else {
                    dis = 0
                }
            } else {
                if self.tile_type_safe_index(tiles, x as isize - dis, y as isize) == TileType::Air {
                    dis = -dis
                } else {
                    dis = 0
                }
            }
        } else {
            dis = 0
        };
        let (target_x, target_y) = ((x as isize + dis) as usize, y + y_dis);

        if self.is_within_bounds(tiles, target_x, target_y) {
            self.move_tile(tiles, x, y, target_x, target_y);
        }
    }

    fn is_within_bounds(&self, tiles: &Vec<Vec<Tile>>, x: usize, y: usize) -> bool {
        x < tiles.len() && y < tiles[0].len()
    }
    fn tile_type_safe_index(&self, tiles: &Vec<Vec<Tile>>, x: isize, y: isize) -> TileType {
        let x_index = x.clamp(0, tiles.len() as isize - 1) as usize;
        let y_index = y.clamp(0, tiles[0].len() as isize - 1) as usize;

        tiles[x_index][y_index].filled
    }

    fn move_tile(
        &mut self,
        tiles: &mut Vec<Vec<Tile>>,
        x: usize,
        y: usize,
        target_x: usize,
        target_y: usize,
    ) {
        if tiles[target_x][target_y].filled == TileType::Air {
            tiles[target_x][target_y] = self.clone();
            tiles[x][y].filled = TileType::Air;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Resource)]
pub enum TileType {
    Air,
    Sand,
    Water,
    Stone,
}
impl TileType {
    pub fn color(&self) -> Color {
        match self {
            TileType::Air => Color::rgba(0.0, 0.0, 0.0, 1.0),
            TileType::Sand => Color::rgba(0.99, 0.98, 0.0, 1.0),
            TileType::Water => Color::rgba(0.0, 0.0, 0.8, 1.0),
            TileType::Stone => Color::rgba(0.502, 0.502, 0.502, 1.0),
        }
    }
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
    pub fn create(&mut self) -> &mut Self {
        self.tiles = (0..self.width)
            .map(|_| (0..self.height).map(|_| Tile::new(TileType::Air)).collect())
            .collect();
        self
    }

    pub fn update_sim(&mut self) {
        for (x_pos, row) in self.tiles.clone().iter_mut().enumerate() {
            for (y_pos, tile) in row.iter_mut().enumerate() {
                tile.update_tile_pos(&mut self.tiles, x_pos, y_pos);
            }
        }
    }
}

// impl Tile {
//     pub fn new(filled: TileType) -> Self {
//         // Match the color to the correct color
//         let color = match filled {
//             TileType::Air => Color::rgba(0.0, 0.0, 0.0, 1.0),
//             TileType::Sand => Color::rgba(0.99, 0.98, 0.0, 1.0),
//             TileType::Water => Color::rgba(0.0, 0.0, 0.8, 1.0),
//             TileType::Stone => Color::rgba(0.502, 0.502, 0.502, 1.0),
//         };
//         Self {
//             filled,
//             color,
//             is_falling: false,
//         }
//     }

//     pub fn update_tile_pos(
//         &mut self,
//         tiles: &mut Vec<Vec<Tile>>,
//         tile_type: TileType,
//         x: usize,
//         y: usize,
//     ) {
//         let mut rng = rand::thread_rng();
//             TileType::Water => {
//                 let check = self.move_tile(tiles, tile_type, x, y, x, y + 1, false).0;
//                 let dir = rng.gen_range(0..10) > 5;
//                 let mut dis = 1;

//                 if !check {
//                     if self.move_tile(tiles, tile_type, x, y, x - 1, y, true).0
//                         || self.move_tile(tiles, tile_type, x, y, x + 1, y, true).0
//                     {
//                         dis = rng.gen_range(2..6);
//                     }
//                     if dir {
//                         if self.move_tile(tiles, tile_type, x, y, x - dis, y, true).0 {
//                             self.move_tile(tiles, tile_type, x, y, x - dis, y, false);
//                         }
//                     } else {
//                         if self.move_tile(tiles, tile_type, x, y, x + dis, y, true).0 {
//                             self.move_tile(tiles, tile_type, x, y, x + dis, y, false);
//                         }
//                     }
//                 }
//             }

//             TileType::Stone => {}
//         }
//     }

//     pub fn move_tile(
//         &mut self,
//         tiles: &mut Vec<Vec<Tile>>,
//         tile_type: TileType,
//         x: usize,
//         y: usize,
//         target_x: usize,
//         target_y: usize,
//         check: bool,
//     ) -> (bool, TileType) {
//         if target_x >= tiles.len() as usize {
//             return (false, tiles[x][y].filled);
//         }
//         if target_y >= tiles[0].len() as usize {
//             return (false, tiles[x][y].filled);
//         }

//         let target_tile: &mut Tile = &mut tiles[target_x][target_y];

//         if target_tile.filled == TileType::Air {
//             if !check {
//                 target_tile.filled = tile_type;
//                 tiles[x][y].filled = TileType::Air;
//             }
//             return (true, tiles[x][y].filled);
//         } else {
//             return (false, tiles[x][y].filled);
//         }
//     }
// }

// #[derive(Clone, Copy, Debug, PartialEq, Resource)]
// pub enum TileType {
//     Air,
//     Sand,
//     Water,
//     Stone,
// }

// impl TileType {
//     pub fn color(&self) -> Color {
//         match self {
//             TileType::Air => Color::rgba(0.0, 0.0, 0.0, 1.0),
//             TileType::Sand => Color::rgba(0.99, 0.98, 0.0, 1.0),
//             TileType::Water => Color::rgba(0.0, 0.0, 0.8, 1.0),
//             TileType::Stone => Color::rgba(0.502, 0.502, 0.502, 1.0),
//         }
//     }
// }

// impl fmt::Display for TileType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let display = match self {
//             TileType::Air => "Air",
//             TileType::Sand => "Sand",
//             TileType::Water => "Water",
//             TileType::Stone => "Stone",
//         };
//         write!(f, "{}", display)
//     }
// }

// impl TileWorld {
//     pub fn create(&mut self) -> &mut Self {
//         self.tiles = Vec::with_capacity(self.width);
//         for _ in 0..self.width {
//             let mut row = Vec::with_capacity(self.height);
//             for _ in 0..self.height {
//                 row.push(Tile::new(TileType::Air));
//             }
//             self.tiles.push(row);
//         }
//         self
//     }
//     pub fn update_sim(&mut self) {
//         for (x_pos, row) in self.tiles.clone().iter_mut().enumerate() {
//             for (y_pos, tile) in row.iter_mut().enumerate() {
//                 if vec![TileType::Sand, TileType::Water].contains(&tile.filled) {
//                     tile.update_tile_pos(&mut self.tiles, tile.filled.clone(), x_pos, y_pos);
//                 }
//             }
//         }
//     }
// }
