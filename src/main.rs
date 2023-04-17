use rand::prelude::*;

struct TileWorld {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug)]
struct Tile {
    filled: bool,
}

impl Iterator for Tile {
    type Item = Tile;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Tile {
    fn new(filled: bool) -> Self {
        Self { filled }
    }
}

impl TileWorld {
    fn create(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();

        self.tiles = vec![vec![]; self.height];
        for i in 0..self.height {
            for _ in 0..self.width {
                self.tiles[i].push(Tile::new(rng.gen_bool(random())));
            }
        }

        for row in &self.tiles {
            for tile in row {}
        }
        return self;
    }

    fn print(&mut self) {
        let mut tiles_to_print = "".to_string();

        for row in &self.tiles {
            for tile in row {
                if tile.filled {
                    tiles_to_print += "█";
                } else {
                    tiles_to_print += " ";
                }
            }
            tiles_to_print += "\n";
        }
        println!("{}", tiles_to_print);
    }
}

fn main() {
    let mut world = TileWorld {
        tiles: vec![vec![Tile { filled: false }]],
        width: 100,
        height: 100,
    };

    let mut world = world.create();
    world.print();
}
