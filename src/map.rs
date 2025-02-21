use noise::{NoiseFn, Perlin};
use rand::Rng;

pub const TERRAIN_SCALE: f64 = 8.0;
pub const RESOURCE_SCALE: f64 = 4.0;
const RESOURCE_PROBABILITY: f64 = 0.1;
const THRESHOLD: f64 = 0.3;

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<char>,
    pub seed: u32,
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let mut map = Map {
            width,
            height,
            grid: vec![' '; width * height],
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

    pub fn get(&self, x: usize, y: usize) -> char {
        self.grid[self.get_index(x, y)]
    }

    fn set(&mut self, x: usize, y: usize, value: char) {
        let idx = self.get_index(x, y);
        self.grid[idx] = value;
    }

    fn generate_terrain(&mut self) {
        let perlin: Perlin = Perlin::new(self.seed);

        for y in 0..self.height {
            for x in 0..self.width {
                let noise_value = perlin.get([x as f64 / TERRAIN_SCALE, y as f64 / TERRAIN_SCALE]);
                if noise_value > THRESHOLD {
                    self.set(x, y, '⛰');
                }
            }
        }
    }

    fn place_resources(&mut self) {
        let perlin = Perlin::new(self.seed);
        let mut rng = rand::rng();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) != ' ' {
                    continue;
                }

                let noise_value =
                    perlin.get([x as f64 / RESOURCE_SCALE, y as f64 / RESOURCE_SCALE]);
                if noise_value > THRESHOLD && rng.random_bool(RESOURCE_PROBABILITY) {
                    self.set(x, y, '⚡');
                } else if noise_value > THRESHOLD && rng.random_bool(RESOURCE_PROBABILITY) {
                    self.set(x, y, '💎');
                }
            }
        }
    }

    fn place_science_base(&mut self) {
        let mut rng = rand::thread_rng();
    
        loop {
            let x = rng.gen_range(1..self.width - 1); 
            let y = rng.gen_range(1..self.height - 1);
    
            if self.get(x, y) == ' ' && self.is_surrounded_by_clear_area(x, y) {
                self.set(x, y, '🏠');
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
            self.get(nx, ny) == ' '
        })
    }

    pub fn place_robot_near_base(&self) -> Option<(usize, usize)> {
        let mut base_pos = None;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == '🏠' {
                    base_pos = Some((x, y));
                    break;
                }
            }
        }

        let (bx, by) = base_pos?;
        
        let directions = [
            (bx.wrapping_sub(1), by), (bx + 1, by), 
            (bx, by.wrapping_sub(1)), (bx, by + 1),
        ];

        for &(x, y) in &directions {
            if x < self.width && y < self.height && self.get(x, y) == ' ' {
                return Some((x, y));
            }
        }
        None
    }

    pub fn place_robot(&mut self, x: usize, y: usize, icon: char) {
        if x < self.width && y < self.height {
            self.set(x, y, icon);
        }
    }

    //for debugging purposes
    pub fn display_in_terminal(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y));
            }
            println!();
        }
    }
}
