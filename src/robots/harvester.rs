use super::robot::{Robot, RobotState, RobotType};
use crate::environment::{
    map::Map,
    tile::{MapTile, Resource, TileType},
};

pub struct Harvester {
    id: usize,
    x: usize,
    y: usize,
    cargo_capacity: u32,
    state: RobotState,
    target_resource: Option<(usize, usize, Resource, Option<bool>)>,
}

impl Robot for Harvester {
    fn new(x: usize, y: usize, id: usize) -> Self {
        Harvester {
            id: id,
            x: x,
            y: y,
            cargo_capacity: 5,
            state: RobotState::Harvesting,
            target_resource: None,
        }
    }
    fn get_id(&self) -> usize {
        self.id
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

    fn get_current_resource(&self) -> Option<(usize, usize, Resource, Option<bool>)> {
        if let Some((x, y, resource, remind)) = self.target_resource {
            Some((x, y, resource, remind))
        } else {
            None
        }
    }

    fn set_target_resource(&mut self, target: Option<(usize, usize, Resource, Option<bool>)>) {
        self.target_resource = target;
    }

    fn update(&mut self, map: &mut Map) {
        match self.state {
            RobotState::Harvesting => {
                self.harvest(map);
            }
            RobotState::ReturningToBase => {
                self.return_to_base(map);
            }
            _ => {}
        }
    }
}

impl Harvester {
    pub fn harvest(&mut self, map: &mut Map) {
        if let Some((x, y, resource, _)) = self.target_resource {
            let step = self.calculate_next_step(x, y, map);
            match step {
                Some((x, y)) => {
                    self.move_to(x, y, map);
                    self.set_position(x, y);
                }
                None => {
                    let amount = resource.scale.min(self.cargo_capacity);
                    let remaining = resource.scale - amount;

                    map.set(MapTile::new(
                        x,
                        y,
                        if remaining > 0 {
                            self.set_target_resource(Some((
                                x,
                                y,
                                Resource::new(remaining, resource.resource_type),
                                Some(true),
                            )));
                            TileType::Resource(Resource::new(remaining, resource.resource_type))
                        } else {
                            self.set_target_resource(None);
                            TileType::Empty
                        },
                    ));
                    self.set_state(RobotState::ReturningToBase);
                }
            }
        }
    }
}
