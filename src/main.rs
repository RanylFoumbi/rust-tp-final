mod environment {
    pub mod map;
    pub mod tile;
}
mod simulation {
    pub mod simulation;
}
mod robots{
    pub mod robot;
    pub mod harvester;
    pub mod explorer;
}
mod windows{
    pub mod graphic_ui;
    pub mod utils;
}

use environment::map::Map;
use robots::{explorer, harvester, robot::Robot};
use windows::utils::open_window;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let map = Arc::new(Mutex::new(Map::new(50, 50, 8)));
    let (x, y) = map.lock().unwrap().base_position;

    let window_map = Arc::clone(&map);
    let window_handle = thread::spawn(move || {
        open_window(window_map).unwrap();
    });

    let mut harvester  = harvester::Harvester::new(x, y);
    let mut explorer = explorer::Explorer::new(x, y);  
    loop {
        let mut map_guard = map.lock().unwrap();
        explorer.update(&mut map_guard);
        harvester.update(&mut map_guard);
        drop(map_guard);
        thread::sleep(Duration::from_millis(16))
    } 
}