use rand::Rng;

use super::robot::{Robot, ResourceType, RobotState};
use crate::map::Map;
use std::collections::HashSet;

pub struct Harvester {
    pub x: usize,
    pub y: usize,
    pub energy: u32,
    pub cargo: Vec<ResourceType>,
    pub discovered_map: HashSet<(usize, usize, char)>,
    pub icon: char,
    pub cargo_capacity: usize,
    pub state: RobotState,
    pub base_position: (usize, usize),
    pub target_resource: Option<(usize, usize, ResourceType)>,
}

impl Robot for Harvester {
    fn new() -> Self {
        Harvester {
            x: 0,
            y: 0,
            energy: 200,
            cargo: Vec::new(),
            discovered_map: HashSet::new(),
            icon: '🚜',
            cargo_capacity: 5,
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

impl Harvester {
    
    pub fn harvest(&mut self, map: &mut Map) -> Option<ResourceType> {
        if self.cargo.len() >= self.cargo_capacity {
            return None;
        }

        let current_tile = map.get(self.x, self.y);
        let resource = match current_tile {
            '💎' => Some(ResourceType::Mineral),
            '⚡' => Some(ResourceType::Energy),
            _ => None,
        };

        if let Some(resource_type) = resource {
            self.decrease_energy(1);
            self.cargo.push(resource_type.clone());
            map.set(self.x, self.y, ' ');
            Some(resource_type)
        } else {
            None
        }
    }

    pub fn get_cargo_capacity(&self) -> usize {
        self.cargo_capacity
    }

    pub fn is_cargo_full(&self) -> bool {
        self.cargo.len() >= self.cargo_capacity
    }

    pub fn scan_for_resources(&self, map: &Map) -> Option<(usize, usize, ResourceType)> {
        for dx in -3..=3 {
            for dy in -3..=3 {
                let new_x = self.x as i32 + dx;
                let new_y = self.y as i32 + dy;
                
                if new_x >= 0 && new_y >= 0 {
                    let x = new_x as usize;
                    let y = new_y as usize;
                    
                    if map.is_valid(x, y) {
                        match map.get(x, y) {
                            '💎' => return Some((x, y, ResourceType::Mineral)),
                            '⚡' => return Some((x, y, ResourceType::Energy)),
                            _ => continue,
                        }
                    }
                }
            }
        }
        None
    }

    pub fn update(&mut self, map: &mut Map) {
        match self.state {
            RobotState::Exploring => {
                if let Some(resource) = self.scan_for_resources(map) {
                    self.target_resource = Some(resource);
                    self.state = RobotState::MovingToResource;
                } else {
                    let mut rng = rand::rng();
                    let (dx, dy) = (rng.random_range(-1..=1), rng.random_range(-1..=1));
                    let new_x = (self.x as i32 + dx) as usize;
                    let new_y = (self.y as i32 + dy) as usize;
                    self.move_to(new_x, new_y, map);
                }
            },
            RobotState::MovingToResource => {
                if let Some((target_x, target_y, _)) = self.target_resource {
                    if self.x == target_x && self.y == target_y {
                        self.state = RobotState::Harvesting;
                    } else {
                        self.move_towards(target_x, target_y, map);
                    }
                }
            },
            RobotState::Harvesting => {
                if self.cargo.len() < self.cargo_capacity {
                    if let Some(_) = self.harvest(map) {
                    } else {
                        self.state = RobotState::Exploring;
                        self.target_resource = None;
                    }
                } else {
                    self.state = RobotState::ReturningToBase;
                }
            },
            RobotState::ReturningToBase => {
                let (base_x, base_y) = self.base_position;
                if self.x == base_x && self.y == base_y {
                    self.cargo.clear();
                    self.state = RobotState::Exploring;
                } else {
                    self.move_towards(base_x, base_y, map);
                }
            },
            RobotState::Idle => {}
        }
    }
}