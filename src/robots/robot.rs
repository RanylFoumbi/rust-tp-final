use std::collections::HashSet;
use rand::Rng;

use crate::map::Map;

#[derive(Debug, Clone, PartialEq)]
pub enum RobotType {
    Explorer,    
    Harvester, 
    Scientist,
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    Energy,     
    Mineral
}

#[derive(Debug)]
pub struct Robot {
    pub id: usize,
    pub robot_type: RobotType,
    pub x: usize,
    pub y: usize,
    pub icon: char,
    pub energy: u32,
    pub cargo: Vec<ResourceType>,
    pub discovered_map: HashSet<(usize, usize, char)>
}
