use super::robot::{Robot, RobotState};
use rand::Rng;
use crate::environment::{map::Map, tile::ResourceType};
use std::collections::HashSet;

pub struct Explorer {
    pub x: usize,
    pub y: usize,
    pub energy: u32,
    pub cargo: Vec<ResourceType>,
    pub discovered_map: HashSet<(usize, usize, char)>,
    pub icon: char,
    pub state: RobotState,
    pub base_position: (usize, usize),
    pub target_resource: Option<(usize, usize, ResourceType)>,
}

impl Robot for Explorer {
    fn new(x: usize, y: usize) -> Self {
        Explorer {
            x: x,
            y: y,
            energy: 150,
            cargo: Vec::new(),
            discovered_map: HashSet::new(),
            icon: '🤖',
            state: RobotState::Exploring,
            base_position: (0, 0),
            target_resource: None,
        }
    }

    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn get_energy(&self) -> u32 {
        self.energy
    }

    fn get_cargo(&self) -> &Vec<ResourceType> {
        &self.cargo
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

    fn decrease_energy(&mut self, amount: u32) {
        self.energy = self.energy.saturating_sub(amount);
    }
}

impl Explorer {
    
    pub fn explore(&mut self, map: &Map) {
        let mut rng = rand::rng();
        let (dx, dy) = (
            rng.random_range(-1..=1),
            rng.random_range(-1..=1)
        );
        
        let new_x = self.x as i32 + dx;
        let new_y = self.y as i32 + dy;
        
        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            
            if self.move_to(new_x, new_y, map) {
                self.discovered_map.insert((
                    new_x,
                    new_y,
                    map.get(new_x, new_y).char
                ));
            }
        }
    }

    pub fn get_discovered_map(&self) -> &HashSet<(usize, usize, char)> {
        &self.discovered_map
    }

    pub fn find_nearest_resource(&self) -> Option<(usize, usize, ResourceType)> {
        self.discovered_map
            .iter()
            .filter(|(_, _, tile)| *tile == '💎' || *tile == '⚡')
            .map(|(x, y, tile)| (*x, *y, 
                if *tile == '💎' { ResourceType::Mineral } 
                else { ResourceType::Energy }
            ))
            .min_by_key(|(x, y, _)| {
                let dx = *x as i32 - self.x as i32;
                let dy = *y as i32 - self.y as i32;
                (dx * dx + dy * dy) as usize  
            })
    }

    pub fn update(&mut self, map: &Map) {
        match self.state {
            RobotState::Exploring => {
                self.explore(map);
                if let Some(resource) = self.find_nearest_resource() {
                    self.target_resource = Some(resource);
                    self.state = RobotState::MovingToResource;
                }
            },
            RobotState::MovingToResource => {
                if let Some((target_x, target_y, _)) = self.target_resource {
                    if self.move_towards(target_x, target_y, map) {
                        if self.x == target_x && self.y == target_y {
                            self.state = RobotState::Exploring;
                        }
                    }
                }
            },
            RobotState::ReturningToBase => {
                let (base_x, base_y) = self.base_position;
                if self.move_towards(base_x, base_y, map) {
                    if self.x == base_x && self.y == base_y {
                        self.cargo.clear();
                        self.state = RobotState::Exploring;
                    }
                }
            },
            _ => {}
        }
    }
}