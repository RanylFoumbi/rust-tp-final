use noise::{NoiseFn, Perlin};
use rand::Rng;

use super::tile::{MapTile, Resource, ResourceType, TileType};

pub const TERRAIN_SCALE: f64 = 6.0;
pub const RESOURCE_SCALE: f64 = 2.0;
const RESOURCE_PROBABILITY: f64 = 0.1;
const THRESHOLD: f64 = 0.3;

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<MapTile>,
    pub seed: u32,
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let mut map = Map {
            width,
            height,
            grid: vec![MapTile::new(0, 0, ' ', TileType::Default); width * height],
            seed,
        };

        map.generate_terrain();
        map.place_resources();
        map.place_science_base();
        map
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> MapTile {
        self.grid[self.get_index(x, y)]
    }

    fn set(&mut self, tile: MapTile) {
        let idx = self.get_index(tile.x, tile.y);
        self.grid[idx] = tile;
    }

    fn generate_terrain(&mut self) {
        let perlin: Perlin = Perlin::new(self.seed);

        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                let noise_value = perlin.get([x as f64 / TERRAIN_SCALE, y as f64 / TERRAIN_SCALE]);
                if noise_value > THRESHOLD {
                    self.set(MapTile::new(x, y,'⛰', TileType::Default));
                }
            }
        }
    }

    fn place_resources(&mut self) {
        let perlin = Perlin::new(self.seed);
        let mut rng = rand::rng();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y).char != ' ' {
                    continue;
                }

                let noise_value =
                    perlin.get([x as f64 / RESOURCE_SCALE, y as f64 / RESOURCE_SCALE]);
                if noise_value > THRESHOLD && rng.random_bool(RESOURCE_PROBABILITY) {
                    self.set(MapTile::new(x, y, '⚡', TileType::Resource(Resource::new(10.0, ResourceType::Energy))));
                } else if noise_value > THRESHOLD && rng.random_bool(RESOURCE_PROBABILITY) {
                    self.set(MapTile::new(x, y, '💎', TileType::Resource(Resource::new(10.0, ResourceType::Mineral))));
                }
            }
        }
    }

    fn place_science_base(&mut self) {
        let mut rng = rand::rng();
    
        loop {
            let x = rng.random_range(1..self.width-1); 
            let y = rng.random_range(1..self.height-1);
    
            if self.get(x, y).char == ' ' && self.is_surrounded_by_clear_area(x, y) {
                self.set(MapTile::new(x, y, '🏠', TileType::Base));
                break;
            }
        }
    }
    
    // ensure that the base is surrounded by clear area
    fn is_surrounded_by_clear_area(&self, x: usize, y: usize) -> bool {
        let directions = [
            (-1, -1), (0, -1), (1, -1),  
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];
    
        directions.iter().all(|(dx, dy)| {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            self.get(nx, ny).char == ' '
        })
    }

    //for debugging purposes
    pub fn display_in_terminal(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y).char);
            }
            println!();
        }
    }
}
