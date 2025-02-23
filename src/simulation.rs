use crate::environment::map::Map;



pub enum SimulationState {
    Play,
    Pause
}

pub struct Simulation {
    pub state: SimulationState,
    pub energy_count: u32,
    pub resource_count: u32,
    pub scientist_area_count: u32,
}

impl Simulation {
    pub fn new(map: &Map) -> Simulation {
        Simulation {
            state: SimulationState::Pause,
            energy_count: 0,
            resource_count: 0,
            scientist_area_count: 0,
        }
    }
}