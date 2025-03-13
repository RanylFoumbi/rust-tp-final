use std::{collections::{HashMap, VecDeque}, any::Any};

use crate::environment::{map::Map, tile::{MapTile, TileType}};


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RobotType {
    Explorer,    
    Harvester,
}

#[derive(Debug, Clone, PartialEq, Copy)]
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
    fn get_state(&self) -> RobotState;
    fn set_state(&mut self, state: RobotState);
    fn get_type(&self) -> RobotType;
    fn update(&mut self, map: &mut Map);
    fn get_current_resource(&self) -> Option<MapTile>;

    fn move_to(&mut self, x: usize, y: usize, map: &mut Map) -> Option<MapTile> {
        if map.is_valid(x, y) {
            let prev_tile = map.get(x, y);
            let (old_x, old_y) = self.get_position();
            if map.get(old_x, old_y).tile == TileType::Robot(self.get_type()) {
                map.set(MapTile::new(old_x, old_y, TileType::Empty));
            }
            match prev_tile.tile {
                TileType::Resource(_) if self.get_type() == RobotType::Explorer => {
                    if let Some(resource) = self.get_current_resource() {
                        map.set(resource);
                    }
                }
                _ => {
                    map.set(MapTile::new(x, y, TileType::Robot(self.get_type())));
                }
            }
            Some(prev_tile)
        } else {
            None
        }
    }

        fn calculate_next_step(&self, target_x: usize, target_y: usize, map: &Map) -> Option<(usize, usize)> {
        let (start_x, start_y) = self.get_position();
        if start_x == target_x && start_y == target_y {
            return None;
        }
        
        let mut queue = VecDeque::new();
        let mut came_from = HashMap::new();
        
        queue.push_back((start_x, start_y));
        came_from.insert((start_x, start_y), None);
        
        while let Some((x, y)) = queue.pop_front() {
            if x == target_x && y == target_y {
                break;
            }
        
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
            
                if new_x < 0 || new_y < 0 {
                    continue;
                }
            
                let new_x = new_x as usize;
                let new_y = new_y as usize;
            
                if map.is_valid(new_x, new_y) && !came_from.contains_key(&(new_x, new_y)) {
                    let tile = map.get(new_x, new_y);
                    if tile.tile != TileType::Terrain {
                        queue.push_back((new_x, new_y));
                        came_from.insert((new_x, new_y), Some((x, y)));
                    }
                }
            }
        }
        
        if !came_from.contains_key(&(target_x, target_y)) {
            let step_x = if start_x < target_x { start_x + 1 } else if start_x > target_x { start_x - 1 } else { start_x };
            let step_y = if start_y < target_y { start_y + 1 } else if start_y > target_y { start_y - 1 } else { start_y };
            if map.is_valid(step_x, step_y) {
                let tile = map.get(step_x, step_y);
                if tile.tile != TileType::Terrain {
                    return Some((step_x, step_y));
                }
            }
            return None;
        }
        
        let mut path = Vec::new();
        let mut current = Some((target_x, target_y));
        
        while let Some(pos) = current {
            path.push(pos);
            current = came_from.get(&pos).cloned().flatten();
        }
        
        path.reverse();
        if path.len() > 1 {
            Some(path[1])
        } else {
            None
        }
    }

    fn set_position(&mut self, x: usize, y: usize);

    fn return_to_base(&mut self, map: &mut Map) {
        let (base_x, base_y) = map.base_position;

        match self.calculate_next_step(base_x, base_y, map) {
            Some((x, y)) => {
                self.move_to(x, y, map);
                self.set_position(x, y);
            }
            None => {
                let position = self.get_position();
                map.set(MapTile::new(position.0, position.1, TileType::Empty));
                self.set_state(RobotState::Idle);
            }
        }
    }
}