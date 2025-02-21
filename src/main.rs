mod map;
mod windows{
    pub mod graphic_ui;
}

use map::Map;
use windows::graphic_ui::open_window;

fn main() {
    let map = Map::new(100, 100, 6);

    //TODO: Remove this line
    map.display_in_terminal();

    open_window(map).unwrap();


}