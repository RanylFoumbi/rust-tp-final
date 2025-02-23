use std::collections::HashSet;
use rand::Rng;

use crate::map::Map;

#[derive(Debug, Clone, PartialEq)]
pub enum RobotType {
    Explorer,    
    Harvester,
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    Energy,     
    Mineral
}

pub trait Robot {
    fn new() -> Self where Self: Sized;
    
    fn get_position(&self) -> (usize, usize);
    fn get_energy(&self) -> u32;
    fn get_cargo(&self) -> &Vec<ResourceType>;
    fn move_to(&mut self, x: usize, y: usize, map: &Map) -> bool {
        if map.is_valid(x, y) {
            self.set_position(x, y);
            self.decrease_energy(1);
            true
        } else {
            false
        }
    }
    
    
    fn set_position(&mut self, x: usize, y: usize);
    fn decrease_energy(&mut self, amount: u32);
}