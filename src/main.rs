mod map;
mod simulation;
mod robots{
    pub mod robot;
    pub mod harvester;
    pub mod explorer;
}
mod windows{
    pub mod graphic_ui;
    pub mod utils;
}

use map::Map;
use robots::{explorer, harvester, robot::Robot};
use windows::utils::open_window;

fn main() {
    let mut map = Map::new(50, 50, 8);
    let harvester  = harvester::Harvester::new();
    let explorer = explorer::Explorer::new();   
    open_window(map).unwrap();
}