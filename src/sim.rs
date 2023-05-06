use macroquad::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct TileWorld {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub filled: TileType,
    pub color: Color,

    pub is_falling: bool,
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = (self.filled.to_string(), self.match_color(), self.is_falling);
        write!(f, "{:?}", display)
    }
}

impl Tile {
    pub fn new(filled: TileType) -> Self {
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
    pub fn match_color(&self) -> Color {
        match self.filled {
            TileType::Air => Color::new(0.0, 0.0, 0.0, 1.0),
            TileType::Sand => Color::new(0.99, 0.98, 0.0, 1.0),
            TileType::Water => Color::new(0.0, 0.0, 0.8, 1.0),
            TileType::Stone => Color::new(0.502, 0.502, 0.502, 1.0),
        }
    }

    pub fn update_tile_pos(
        &mut self,
        tiles: &mut Vec<Vec<Tile>>,
        last_world_tiles: &mut Vec<Vec<Tile>>,
        tile_type: TileType,
        x: usize,
        y: usize,
    ) {
        match tile_type {
            TileType::Air => {}
            TileType::Sand => {
                let check = self.move_tile(tiles, tile_type, x, y, x, y + 1, false).0;

                if !check {
                    if rand::RandomRange::gen_range(0, 10) > 5 {
                        if self.move_tile(tiles, tile_type, x, y, x - 1, y + 1, true).0 {
                            self.move_tile(tiles, tile_type, x, y, x - 1, y + 1, false);
                        }
                    } else {
                        if self.move_tile(tiles, tile_type, x, y, x + 1, y + 1, true).0 {
                            self.move_tile(tiles, tile_type, x, y, x + 1, y + 1, false);
                        }
                    }
                }
            }
            TileType::Water => {
                let check = self.move_tile(tiles, tile_type, x, y, x, y + 1, false).0;
                let dir = rand::RandomRange::gen_range(0, 10) > 5;
                let mut dis = 1;

                if !check {
                    if self.move_tile(tiles, tile_type, x, y, x - 1, y, true).0
                        || self.move_tile(tiles, tile_type, x, y, x + 1, y, true).0
                    {
                        dis = rand::RandomRange::gen_range(2, 6);
                    }
                    if dir {
                        if self.move_tile(tiles, tile_type, x, y, x - dis, y, true).0 {
                            self.move_tile(tiles, tile_type, x, y, x - dis, y, false);
                        }
                    } else {
                        if self.move_tile(tiles, tile_type, x, y, x + dis, y, true).0 {
                            self.move_tile(tiles, tile_type, x, y, x + dis, y, false);
                        }
                    }
                }
            }

            TileType::Stone => {}
        }
    }

    pub fn move_tile(
        &mut self,
        tiles: &mut Vec<Vec<Tile>>,
        tile_type: TileType,
        x: usize,
        y: usize,
        target_x: usize,
        target_y: usize,
        check: bool,
    ) -> (bool, TileType) {
        if target_x >= tiles.len() as usize {
            return (false, tiles[x][y].filled);
        }

        if target_y >= tiles[0].len() as usize {
            return (false, tiles[x][y].filled);
        }

        let target_tile: &mut Tile = &mut tiles[target_x][target_y];

        if target_tile.filled == TileType::Air {
            if !check {
                target_tile.filled = tile_type;
                tiles[x][y].filled = TileType::Air;
            }
            return (true, tiles[x][y].filled);
        } else {
            return (false, tiles[x][y].filled);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
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
    pub fn create(&mut self) -> &mut Self {
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
    pub fn update_sim(&mut self, mut last_world_tiles: Vec<Vec<Tile>>) {
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
