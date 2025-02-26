use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::time::Duration;
use crate::environment::map::Map;
use crate::robots::{explorer::Explorer, harvester::Harvester, robot::Robot};


pub enum SimulationState {
    Play,
    Pause
}

pub struct Simulation {
    map: Arc<Mutex<Map>>, 
    pub barrier: Arc<Barrier>, 
    pub energy_count: u32,
    pub resource_count: u32,
    pub scientist_area_count: u32,
    pub state: SimulationState
}

impl Simulation {
    pub fn new(map: Arc<Mutex<Map>>) -> Self {
        let num_threads = 3; // TODO : To replace with the number of robots
        Simulation {
            map: map,
            barrier: Arc::new(Barrier::new(num_threads + 1)), // +1 because the main thread is also waiting
            energy_count: 0,
            resource_count: 0,
            scientist_area_count: 0,
            state: SimulationState::Play
        }
    }

    pub fn run(&self) {
        let map_clone = Arc::clone(&self.map);
        let barrier_clone = Arc::clone(&self.barrier);
        let (x, y) = self.map.lock().unwrap().base_position;

        let explorer_thread = thread::spawn({
            let map_clone = Arc::clone(&map_clone);
            let barrier_clone = Arc::clone(&barrier_clone);
            move || {
                let mut explorer = Explorer::new(x, y);
                
                loop {
                    thread::sleep(Duration::from_millis(500));

                    {
                        let mut map = map_clone.lock().unwrap();
                        explorer.update(&mut map);
                    }

                    barrier_clone.wait(); // waiting to synchronize all threads
                }
            }
        });

        let harvester_thread = thread::spawn({
            let map_clone = Arc::clone(&map_clone);
            let barrier_clone = Arc::clone(&barrier_clone);
            move || {
                 
                let mut harvester = Harvester::new(x, y);
                
                loop {
                    thread::sleep(Duration::from_millis(500));

                    {
                        let mut map = map_clone.lock().unwrap();
                        harvester.update(&mut map);
                    }

                    barrier_clone.wait();
                }
            }
        });

        loop {
            thread::sleep(Duration::from_millis(500));
            barrier_clone.wait(); // main thread waiting for all threads to finish
        }

        explorer_thread.join().unwrap();
        harvester_thread.join().unwrap();
    }
}
