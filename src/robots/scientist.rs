use super::robot::{Robot, RobotType};
use std::collections::HashSet;
use crate::map::Map;

pub struct Scientist {
    pub robot_type: RobotType,
    pub icon: char,
    pub x: usize,
    pub y: usize,
    pub energy: u32,
    pub cargo: Vec<char>,
    pub discovered_map: HashSet<(usize, usize, char)>,
}

impl Robot for Scientist {
    fn new(x: usize, y: usize) -> Self {
        Scientist {
            robot_type: RobotType::Scientist,
            icon: '🔬',
            x,
            y,
            energy: 250,
            cargo: vec![],
            discovered_map: HashSet::new(),
        }
    }

    fn move_robot(&mut self, map: &Map) {
        //TODO: Implement scientist movement
    }

    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
