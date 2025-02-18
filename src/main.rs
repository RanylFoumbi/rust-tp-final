mod map;
mod window;

use map::Map;
use window::open_window;

fn main() {
    let map = Map::new(80, 40, 5);
    open_window(map).unwrap();
}