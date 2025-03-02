use crate::environment::map::Map;
use crate::environment::tile::{MapTile, Resource, TileType};
use crate::robots::robot::RobotType;
use crate::robots::{explorer::Explorer, harvester::Harvester, robot::Robot};
use crate::windows::utils::open_window;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Simulation {
    pub map: Arc<Mutex<Map>>,
    pub energy_count: u32,
    pub resource_count: u32,
    pub running: Arc<AtomicBool>,
    explorer_threads: Arc<Mutex<VecDeque<thread::JoinHandle<()>>>>,
    harvester_threads: Arc<Mutex<VecDeque<thread::JoinHandle<()>>>>,
    located_resources: Arc<Mutex<VecDeque<Vec<(usize, usize, Resource)>>>>,
}

impl Simulation {
    pub fn new() -> Self {
        let map = Arc::new(Mutex::new(Map::new(50, 50, 8)));

        Simulation {
            map,
            energy_count: 0,
            resource_count: 0,
            running: Arc::new(AtomicBool::new(false)),
            explorer_threads: Arc::new(Mutex::new(VecDeque::new())),
            harvester_threads: Arc::new(Mutex::new(VecDeque::new())),
            located_resources: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn play(&self) {
        self.running.store(true, Ordering::SeqCst);
    }

    pub fn pause(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn run(&mut self) {
        let _ = open_window(self);
    }

    pub fn create_robot(&mut self, robot_type: RobotType) {
        
    }
}
