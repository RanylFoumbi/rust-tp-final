mod map;
mod simulation;
mod windows{
    pub mod graphic_ui;
    pub mod utils;
}

use map::Map;
use windows::utils::open_window;

fn main() {
    let map = Map::new(50, 50, 8);
    open_window(map).unwrap();
}