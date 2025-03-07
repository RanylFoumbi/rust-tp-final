use crate::environment::map::Map;
use crate::environment::tile::{MapTile, Resource, TileType};
use crate::robots::robot::RobotType;
use crate::robots::{explorer::Explorer, harvester::Harvester, robot::Robot};
use crate::windows::utils::open_window;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Simulation {
    pub map: Arc<RwLock<Map>>,
    pub energy_count: u32,
    pub resource_count: u32,
    pub running: Arc<AtomicBool>,
    explorer_threads: Arc<Mutex<VecDeque<thread::JoinHandle<()>>>>,
    harvester_threads: Arc<Mutex<VecDeque<thread::JoinHandle<()>>>>,
    located_resources: Arc<Mutex<VecDeque<Vec<(usize, usize, Resource)>>>>,
}

impl Simulation {
    pub fn new() -> Self {
        let map = Arc::new(RwLock::new(Map::new(25, 25, 8)));

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
        let map_guard = self.map.read().unwrap();
        let base_pos = map_guard.base_position;
        drop(map_guard); 
    
        let mut robot: Box<dyn Robot + Send> = match robot_type {
            RobotType::Explorer => Box::new(Explorer::new(
                base_pos.0.saturating_sub(1),
                base_pos.1
            )),
            RobotType::Harvester => Box::new(Harvester::new(
                base_pos.0.saturating_add(1),
                base_pos.1
            )),
        };
    
        let map = Arc::clone(&self.map);
        let running = Arc::clone(&self.running);
        let thread_handle = thread::spawn(move || {
            loop {
                if !running.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_millis(500));
                    continue;
                }
    
                let mut map_guard = map.write().unwrap();
                robot.update(&mut map_guard);
                drop(map_guard);
    
                thread::sleep(Duration::from_millis(500));
            }
        });
    
        match robot_type {
            RobotType::Explorer => {
                let mut explorer_threads = self.explorer_threads.lock().unwrap();
                explorer_threads.push_back(thread_handle);
            }
            RobotType::Harvester => {
                let mut harvester_threads = self.harvester_threads.lock().unwrap();
                harvester_threads.push_back(thread_handle);
            }
        }
    
        println!("Created new {:?} robot near base at {:?}", robot_type, base_pos);
    }
}