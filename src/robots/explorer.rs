use super::robot::{Robot, RobotState, RobotType};
use crate::environment::{
    map::Map,
    tile::{Resource, TileType},
};
use rand::Rng;

pub struct Explorer {
    id: usize,
    x: usize,
    y: usize,
    resource: Option<(usize, usize, Resource, Option<bool>)>,
    state: RobotState,
}

impl Robot for Explorer {
    fn new(x: usize, y: usize, id: usize) -> Self {
        Explorer {
            id: id,
            x: x,
            y: y,
            resource: None,
            state: RobotState::Exploring,
        }
    }

    fn get_type(&self) -> RobotType {
        RobotType::Explorer
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn get_state(&self) -> RobotState {
        self.state.clone()
    }

    fn set_state(&mut self, state: RobotState) {
        self.state = state;
    }

    fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn update(&mut self, map: &mut Map) {
        match self.state {
            RobotState::Exploring => {
                self.explore(map);
            }
            RobotState::ReturningToBase => {
                self.return_to_base(map);
            }
            _ => {}
        }
    }

    fn get_current_resource(&self) -> Option<(usize, usize, Resource, Option<bool>)> {
        self.resource
    }

    fn set_target_resource(&mut self, _: Option<(usize, usize, Resource, Option<bool>)>) {}
}

impl Explorer {
    pub fn explore(&mut self, map: &mut Map) {
        let mut rng = rand::rng();

        let move_horizontal = rng.random_bool(0.5);

        let (direction_x, direction_y) = if move_horizontal {
            (if rng.random_bool(0.5) { 1 } else { -1 }, 0)
        } else {
            (0, if rng.random_bool(0.5) { 1 } else { -1 })
        };

        let new_x = (self.x as i32 + direction_x).clamp(0, (map.width - 1) as i32) as usize;
        let new_y = (self.y as i32 + direction_y).clamp(0, (map.height - 1) as i32) as usize;

        match self.move_to(new_x, new_y, map) {
            Some(map_tile) => {
                match map_tile.tile {
                    TileType::Resource(resource) => {
                        self.resource = Some((map_tile.x, map_tile.y, resource, None));
                        print!("Explorer found a resource at ({}, {})\n", new_x, new_y);
                        self.set_state(RobotState::ReturningToBase);
                    }
                    _ => {}
                }
                self.set_position(new_x, new_y);
            }
            None => {}
        }
    }
}
