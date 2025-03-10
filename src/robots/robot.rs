use std::{collections::{HashMap, VecDeque}, any::Any};
use rand::Rng;

use crate::environment::{map::Map, tile::{MapTile, Resource, TileType}};

use super::explorer::Explorer;

#[derive(Debug, Clone, PartialEq, Copy)]
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

pub trait Robot: Any {
    fn new(x: usize, y: usize) -> Self where Self: Sized;
    
    fn get_position(&self) -> (usize, usize);
    fn get_state(&self) -> &RobotState;
    fn set_state(&mut self, state: RobotState);
    fn get_type(&self) -> RobotType;
    fn update(&mut self, map: &mut Map);
    fn get_current_resource(&self) -> Option<MapTile>;

    fn move_to(&mut self, x: usize, y: usize, map: &mut Map) -> Option<MapTile> {
        if map.is_valid(x, y) {
            let prev_tile = map.get(x, y);
            let (old_x, old_y) = self.get_position();
            map.set(MapTile::new(old_x, old_y, TileType::Empty));
            map.set(MapTile::new(x, y, TileType::Robot(self.get_type())));
            Some(prev_tile)
        } else {
            None
        }
    }

    fn calculate_next_step(&self, target_x: usize, target_y: usize, map: &Map) -> Option<(usize, usize)> {
        let (start_x, start_y) = self.get_position();
        let mut queue = VecDeque::new();
        let mut came_from = HashMap::new();
    
        queue.push_back((start_x, start_y));
        came_from.insert((start_x, start_y), None);
    
        while let Some((x, y)) = queue.pop_front() {
            if x == target_x && y == target_y {
                break;
            }
    
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = x.wrapping_add(dx as usize);
                let new_y = y.wrapping_add(dy as usize);
    
                if map.is_valid(new_x, new_y) && !came_from.contains_key(&(new_x, new_y)) {
                    queue.push_back((new_x, new_y));
                    came_from.insert((new_x, new_y), Some((x, y)));
                }
            }
        }
    
        let mut path = Vec::new();
        let mut current = Some((target_x, target_y));
    
        while let Some(pos) = current {
            path.push(pos);
            current = came_from.get(&pos).cloned().flatten();
        }

        path.reverse();
        path.get(0).cloned()
    }

    fn set_position(&mut self, x: usize, y: usize);

    fn return_to_base(&mut self, map: &mut Map) {
        let (base_x, base_y) = map.base_position;

        if let Some((x, y)) = self.calculate_next_step(base_x, base_y, map) {
            self.move_to(x, y, map);
            self.set_position(x, y);
        }
    }
}