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
use windows::utils::open_window;
use std::sync::{Arc, Mutex};

fn main() {
    let map = Arc::new(Mutex::new(Map::new(50, 50, 8)));
    let _ = open_window(map);
}