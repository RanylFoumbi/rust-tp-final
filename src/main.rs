mod map;
mod windows{
    pub mod graphic_ui;
}

use map::Map;
use windows::graphic_ui::open_window;

fn main() {
    let map = Map::new(800, 400, 5);

    open_window(map).unwrap();
}