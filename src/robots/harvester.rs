use std::clone;

use super::robot::{Robot, RobotState, RobotType};
use crate::environment::{map::Map, tile::{MapTile, Resource, TileType}};

pub struct Harvester {
    pub x: usize,
    pub y: usize,
    pub energy: u32,
    pub cargo_capacity: u32,
    pub state: RobotState,
    pub target_resource: Option<(usize, usize, Resource)>,
}

impl Robot for Harvester {
    fn new(x: usize, y: usize) -> Self {
        Harvester {
            x: x,
            y: y,
            energy: 200,
            cargo_capacity: 5,
            state: RobotState::MovingToResource,
            target_resource: None,
        }
    }

    fn get_type(&self) -> RobotType {
        RobotType::Harvester
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

    fn get_current_resource(&self) -> Option<MapTile> {
        None
    }

    fn update(&mut self, map: &mut Map) {
        match self.state {
            RobotState::MovingToResource  =>{
                let (target_x, target_y, _) = self.target_resource.unwrap();
                let step = self.calculate_next_step(target_x, target_y, map);
                match step {
                    Some((x, y)) => {
                        self.move_to(x, y, map);
                        if x == target_x && y == target_y {
                            self.set_state(RobotState::Harvesting);
                        }
                    },
                    None => {
                        self.set_state(RobotState::ReturningToBase);
                    }
                }
            },
            RobotState::Harvesting => {
                self.harvest(map);
            },
            RobotState::ReturningToBase => {
                self.return_to_base(map);
            },
            _ => {}
        }
    }
}

impl Harvester {

    pub fn harvest(&mut self, map: &mut Map) -> Option<Resource> {
        let (_, _, resource) = self.target_resource?;
        
        self.decrease_energy(1);
        
        let amount = resource.scale.min(self.cargo_capacity);
        let remaining = resource.scale - amount;
        
        map.set(MapTile::new(
            self.x,
            self.y,
            if remaining > 0 {
                TileType::Resource(Resource::new(remaining, resource.resource_type))
            } else {
                TileType::Empty
            }
        ));

        self.set_state(RobotState::ReturningToBase);

        Some(Resource::new(amount, resource.resource_type))
    }

    fn decrease_energy(&mut self, amount: u32) {
        self.energy = self.energy.saturating_sub(amount);
    }

    // pub fn get_energy(&self) -> u32 {
    //     self.energy
    // }

    // pub fn set_target_resource(&mut self, x: usize, y: usize, resource: Resource) {
    //     self.target_resource = Some((x, y, resource));
    // }
}