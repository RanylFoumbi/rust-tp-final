use std::sync::{Arc, Barrier, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use crate::environment::map::Map;
use crate::robots::{Robot};
use crate::robots::explorer::Explorer;
use crate::robots::harvester::Harvester;

pub struct Simulation {
    robots: Arc<Mutex<Vec<Box<dyn Robot + Send>>>>,
    barrier: Arc<Barrier>,
    pub map: Arc<Mutex<Map>>,
    running: Arc<AtomicBool>,
}

impl Simulation {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let map = Arc::new(Mutex::new(Map::new(width, height, seed)));
        let robots = Arc::new(Mutex::new(Vec::new()));
        let barrier = Arc::new(Barrier::new(1));
        let running = Arc::new(AtomicBool::new(true));

        let mut simulation = Simulation { robots, barrier, map, running };
        simulation.spawn_initial_robots();
        simulation
    }

    pub fn add_robot(&mut self, robot: Box<dyn Robot + Send>) {
        let mut robots = self.robots.lock().unwrap();
        robots.push(robot);
        self.barrier = Arc::new(Barrier::new(robots.len() + 1));
    }

    fn spawn_initial_robots(&mut self) {
        let base_pos = self.map.lock().unwrap().base_position;
        let explorer_pos = (base_pos.0.saturating_sub(1), base_pos.1);
        let harvester_pos = (base_pos.0.saturating_add(1), base_pos.1);

        let mut new_robots: Vec<Box<dyn Robot + Send>> = Vec::new();
        new_robots.push(Box::new(Explorer::new(explorer_pos.0, explorer_pos.1)));
        new_robots.push(Box::new(Harvester::new(harvester_pos.0, harvester_pos.1)));

        for robot in new_robots {
            self.add_robot(robot);
        }

        println!("Explorateur ajouté en {:?}, Harvester ajouté en {:?}", explorer_pos, harvester_pos);
    }

    pub fn start(&self) {
        let robots = Arc::clone(&self.robots);
        let barrier_main = Arc::clone(&self.barrier); 
        let map = Arc::clone(&self.map);
        let running = Arc::clone(&self.running);
    
        thread::spawn({
            let barrier = Arc::clone(&barrier_main);
            let running = Arc::clone(&running);
            move || {
                loop {
                    if !running.load(Ordering::SeqCst) {
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }
    
                    barrier.wait();
                    println!("Tick terminé !");
                    thread::sleep(Duration::from_millis(500));
                }
            }
        });
    
        for i in 0..robots.lock().unwrap().len() {
            let robots = Arc::clone(&robots);
            let barrier = Arc::clone(&barrier_main);
            let map = Arc::clone(&map);
            let running = Arc::clone(&running);
    
            thread::spawn(move || {
                loop {
                    if !running.load(Ordering::SeqCst) {
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }
    
                    {
                        let mut map_guard = map.lock().unwrap();
                        let mut robots_guard = robots.lock().unwrap();
    
                        if let Some(robot) = robots_guard.get_mut(i) {
                            robot.update(&mut map_guard);
                        }
                    }
    
                    barrier.wait();
                    thread::sleep(Duration::from_millis(500));
                }
            });
        }
    }
    

    pub fn pause(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn play(&self) {
        self.running.store(true, Ordering::SeqCst);
    }
}
