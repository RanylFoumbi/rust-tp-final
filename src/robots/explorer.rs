use crate::robots::{Robot, RobotState, RobotType}; // Corrigé
use crate::environment::{Map, MapTile, Resource, ResourceType, TileType};
use rand::Rng;
use std::{collections::HashSet, thread, time::Duration};


#[derive(Debug)]
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
            x,
            y,
            cargo: Vec::new(),
            cargo_capacity: 5,
            state: RobotState::Exploring,
        }
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
}

impl Explorer {
    pub fn explore(&mut self, map: &mut Map) {
        let mut rng = rand::thread_rng();
        
        let move_horizontal = rng.gen_bool(0.5);
        
        let (direction_x, direction_y) = if move_horizontal {
            (if rng.gen_bool(0.5) { 1 } else { -1 }, 0)
        } else {
            (0, if rng.gen_bool(0.5) { 1 } else { -1 })
        };
        
        let new_x = (self.x as isize + direction_x).max(0).min((map.width - 1) as isize) as usize;
        let new_y = (self.y as isize + direction_y).max(0).min((map.height - 1) as isize) as usize;
        
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
    }
}
