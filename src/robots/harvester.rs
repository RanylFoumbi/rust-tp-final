use super::robot::{Robot, RobotType};
use std::collections::HashSet;
use crate::map::Map;

pub struct Harvester {
    pub robot_type: RobotType,
    pub icon: char,
    pub x: usize,
    pub y: usize,
    pub energy: u32,
    pub cargo: Vec<char>,
    pub discovered_map: HashSet<(usize, usize, char)>,
}

impl Robot for Harvester {
    fn new(x: usize, y: usize) -> Self {
        Harvester {
            robot_type: RobotType::Harvester,
            icon: '🚜',
            x,
            y,
            energy: 200,
            cargo: vec![],
            discovered_map: HashSet::new(),
        }
    }

    fn move_robot(&mut self, map: &Map) {
        //TODO: Implement harvester movement
    }

    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
