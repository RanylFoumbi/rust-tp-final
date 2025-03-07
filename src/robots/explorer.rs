use super::robot::{Robot, RobotState, RobotType, CHAR_HEIGHT, CHAR_WIDTH};
use crate::environment::{
    map::Map,
    tile::{MapTile, Resource, ResourceType, TileType},
};
use rand::Rng;
use std::{collections::HashSet, thread, time::Duration};

pub struct Explorer {
    pub x: usize,
    pub y: usize,
    pub cargo: Vec<MapTile>,
    pub cargo_capacity: u32,
    pub state: RobotState,
}

impl Robot for Explorer {
    fn new(x: usize, y: usize) -> Self {
        Explorer {
            x: x,
            y: y,
            cargo: Vec::new(),
            cargo_capacity: 5,
            state: RobotState::Exploring,
        }
    }

    fn get_type(&self) -> RobotType {
        RobotType::Explorer
    }

    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn get_state(&self) -> &RobotState {
        &self.state
    }

    fn set_state(&mut self, state: RobotState) {
        self.state = state;
    }

    fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn update(&mut self, map: &mut Map) {
        match self.state {
            RobotState::Exploring => {
                self.explore(map);
            }
            RobotState::ReturningToBase => {
                self.return_to_base(map);
            }
            _ => {}
        }
    }
}

impl Explorer {
    pub fn explore(&mut self, map: &mut Map) {
        let mut rng = rand::rng();
        
        // Choose random direction (up, down, left, right)
        let move_horizontal = rng.random_bool(0.5);
        
        let (direction_x, direction_y) = if move_horizontal {
            (if rng.random_bool(0.5) { 1 } else { -1 }, 0)
        } else {
            (0, if rng.random_bool(0.5) { 1 } else { -1 })
        };
        
        // Calculate new position (one step at a time)
        let new_x = (self.x as i32 + direction_x).clamp(0, (map.width - 1) as i32) as usize;
        let new_y = (self.y as i32 + direction_y).clamp(0, (map.height - 1) as i32) as usize;
        
        if self.move_to(new_x, new_y, map) {
            let tile = map.get(self.x, self.y).tile;
            match tile {
                TileType::Resource(_) => {
                    if self.cargo.len() < self.cargo_capacity as usize {
                        self.cargo.push(MapTile::new(self.x, self.y, tile));
                    } else {
                        self.state = RobotState::ReturningToBase;
                    }
                }
                _ => {}
            }
        }
    
        thread::sleep(Duration::from_millis(100));
    }
}
