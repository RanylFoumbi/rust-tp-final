use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum RobotType {
    Explorer,    
    Harvester, 
    Scientist,
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    Energy,     
    Mineral,
}

pub trait Robot {
    fn new(x: usize, y: usize) -> Self;
    fn move_robot(&mut self, map: &crate::map::Map);
    fn get_position(&self) -> (usize, usize);
}
