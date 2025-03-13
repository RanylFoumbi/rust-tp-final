use crate::environment::map::Map;
use crate::environment::tile::Resource;
// use crate::environment::tile::Resource;
use crate::robots::robot::{RobotState, RobotType};
use crate::robots::{explorer::Explorer, harvester::Harvester, robot::Robot};
use crate::windows::utils::open_window;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, Mutex};
use std::{clone, thread};
use std::time::Duration;

#[derive(Clone)]
pub struct Simulation {
    pub map: Arc<RwLock<Map>>,
    pub energy_count: u32,
    pub resource_count: u32,
    pub running: Arc<AtomicBool>,
    pub speed: Arc<Mutex<u64>>,
    explorer_threads: Arc<Mutex<VecDeque<thread::JoinHandle<()>>>>,
    harvester_threads: Arc<Mutex<VecDeque<thread::JoinHandle<()>>>>,
    pub located_resources: Arc<Mutex<VecDeque<Vec<(usize, usize, Resource)>>>>,
}

impl Simulation {
    pub fn new() -> Self {
        let map = Arc::new(RwLock::new(Map::new(25, 25, 8)));

        Simulation {
            map,
            energy_count: 0,
            resource_count: 0,
            speed: Arc::new(Mutex::new(500)),
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

    pub fn increase_speed(&mut self) {
        let mut speed = self.speed.lock().unwrap();
        if *speed > 100 {
            *speed -= 100;
        }
    }

    pub fn decrease_speed(&mut self) {
        let mut speed = self.speed.lock().unwrap();
        *speed += 100;
    }

    pub fn create_robot(&mut self, robot_type: RobotType) {
        let map_guard = self.map.read().unwrap();
        let base_pos = map_guard.base_position;
        drop(map_guard); 
    
        let mut robot: Box<dyn Robot + Send> = match robot_type {
            RobotType::Explorer => Box::new(Explorer::new(
                base_pos.0,
                base_pos.1
            )),
            RobotType::Harvester => Box::new(Harvester::new(
                base_pos.0,
                base_pos.1
            )),
        };
    
        let map = Arc::clone(&self.map);
        let running = Arc::clone(&self.running);
        let speed = Arc::clone(&self.speed);
        let self_clone  = self.clone();
        let thread_handle = thread::spawn(move || {
            loop {
                if robot.get_state() == RobotState::Idle {
                    self_clone.robot_coming_back(&robot);
                    break;
                }
                let sleep_time = {
                    let speed = speed.lock().unwrap();
                    *speed
                };
                if !running.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_millis(sleep_time));
                    continue;
                }
    
                let mut map_guard = map.write().unwrap();
                robot.update(&mut map_guard);
                drop(map_guard);
    
                thread::sleep(Duration::from_millis(sleep_time));
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

    fn robot_coming_back(&self, robot: &Box<dyn Robot + Send>) {
        match robot.get_type() {
            RobotType::Explorer => {
                let found_resource = robot.get_current_resource();
                if let Some((res_x, res_y, resource)) = found_resource {
                    let mut located_resources = self.located_resources.lock().unwrap();
                    let resource_exists = located_resources.iter().any(|resources| {
                        resources.iter().any(|(x, y, _)| *x ==  res_x && *y == res_y)
                    });
    
                    if !resource_exists {
                        located_resources.push_back(vec![(res_x, res_y, resource)]);
                    }
                }
            }
            RobotType::Harvester => {
            }
        }
    }

}