use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::time::Duration;
use crate::map::Map;
use crate::robots::{Robot, Explorer, Harvester, Scientist};

pub struct Simulation {
    map: Arc<Mutex<Map>>, 
    barrier: Arc<Barrier>, 
}

impl Simulation {
    pub fn new(map: Map) -> Self {
        let num_threads = 3; // TODO : To replace with the number of robots
        Simulation {
            map: Arc::new(Mutex::new(map)),
            barrier: Arc::new(Barrier::new(num_threads + 1)), // +1 because the main thread is also waiting
        }
    }

    pub fn run(&self) {
        let map_clone = Arc::clone(&self.map);
        let barrier_clone = Arc::clone(&self.barrier);

        let explorer_thread = thread::spawn({
            let map_clone = Arc::clone(&map_clone);
            let barrier_clone = Arc::clone(&barrier_clone);
            move || {
                let (x, y) = map_clone.lock().unwrap().place_robot_near_base().unwrap_or((0, 0));
                let mut explorer = Explorer::new(x, y);
                
                loop {
                    thread::sleep(Duration::from_millis(500));

                    {
                        let mut map = map_clone.lock().unwrap();
                        explorer.move_robot(&mut map);
                    }

                    barrier_clone.wait(); // waiting to synchronize all threads
                }
            }
        });

        let harvester_thread = thread::spawn({
            let map_clone = Arc::clone(&map_clone);
            let barrier_clone = Arc::clone(&barrier_clone);
            move || {
                let (x, y) = map_clone.lock().unwrap().place_robot_near_base().unwrap_or((0, 0));
                let mut harvester = Harvester::new(x, y);
                
                loop {
                    thread::sleep(Duration::from_millis(500));

                    {
                        let mut map = map_clone.lock().unwrap();
                        harvester.move_robot(&mut map);
                    }

                    barrier_clone.wait();
                }
            }
        });

        let scientist_thread = thread::spawn({
            let map_clone = Arc::clone(&map_clone);
            let barrier_clone = Arc::clone(&barrier_clone);
            move || {
                let (x, y) = map_clone.lock().unwrap().place_robot_near_base().unwrap_or((0, 0));
                let mut scientist = Scientist::new(x, y);
                
                loop {
                    thread::sleep(Duration::from_millis(500));

                    {
                        let mut map = map_clone.lock().unwrap();
                        scientist.move_robot(&mut map);
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
        scientist_thread.join().unwrap();
    }
}
