use std::collections::HashSet;
use rand::Rng;

use crate::environment::{map::Map, tile::ResourceType};

#[derive(Debug, Clone, PartialEq)]
pub enum RobotType {
    Explorer,    
    Harvester,
}

pub enum RobotState {
    Exploring,        
    MovingToResource, 
    Harvesting,       
    ReturningToBase, 
    Idle            
}

pub trait Robot {
    fn new(x: usize, y: usize) -> Self where Self: Sized;
    
    fn get_position(&self) -> (usize, usize);
    fn get_energy(&self) -> u32;
    fn get_cargo(&self) -> &Vec<ResourceType>;
    fn get_state(&self) -> &RobotState;
    fn set_state(&mut self, state: RobotState);
    fn move_to(&mut self, x: usize, y: usize, map: &Map) -> bool {
        if map.is_valid(x, y) {
            self.set_position(x, y);
            self.decrease_energy(1);
            true
        } else {
            false
        }
    }

    fn calculate_path(&self, target_x: usize, target_y: usize) -> Vec<(usize, usize)> {
        let (start_x, start_y) = self.get_position();
        vec![(start_x, start_y), (target_x, target_y)]
    }

    fn move_towards(&mut self, target_x: usize, target_y: usize, map: &Map) -> bool {
        let (current_x, current_y) = self.get_position();

        let dx = if target_x > current_x { 1 } 
                else if target_x < current_x { -1 } 
                else { 0 };
        
        let dy = if target_y > current_y { 1 } 
                else if target_y < current_y { -1 } 
                else { 0 };

        let new_x = (current_x as i32 + dx) as usize;
        let new_y = (current_y as i32 + dy) as usize;

        self.move_to(new_x, new_y, map)
    }
    
    
    fn set_position(&mut self, x: usize, y: usize);
    fn decrease_energy(&mut self, amount: u32);
}