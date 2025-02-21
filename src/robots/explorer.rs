use crate::robots::robot::{Robot, RobotType};
use std::collections::HashSet;
use crate::map::Map;

pub struct Explorer {
    pub robot_type: RobotType,
    pub icon: char,
    pub x: usize,
    pub y: usize,
    pub energy: u32,
    pub cargo: Vec<char>,
    pub discovered_map: HashSet<(usize, usize, char)>,
}

impl Explorer {
    pub fn new(x: usize, y: usize) -> Self {
        Explorer {
            robot_type: RobotType::Explorer,
            icon: '🤖',
            x,
            y,
            energy: 150,
            cargo: vec![],
            discovered_map: HashSet::new(),
        }
    }
}


impl Robot for Explorer {
    fn new(x: usize, y: usize) -> Self {
        Explorer {
            robot_type: RobotType::Explorer,
            icon: '🤖',
            x,
            y,
            energy: 150,
            cargo: vec![],
            discovered_map: HashSet::new(),
        }
    }

    fn move_robot(&mut self, map: &Map) {
        // Déplacement temporaire (juste pour tester)
        self.x = (self.x + 1) % map.width;
    }

    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
