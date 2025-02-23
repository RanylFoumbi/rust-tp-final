mod environment {
    pub mod map;
    pub mod tile;
}
mod simulation;
mod windows{
    pub mod graphic_ui;
    pub mod utils;
}

use environment::map::Map;
use windows::utils::open_window;

fn main() {
    let map = Map::new(50, 50, 5);
    open_window(map).unwrap();
}