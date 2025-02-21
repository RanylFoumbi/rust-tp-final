mod map;
mod robots;
mod windows;

use map::Map;
use robots::Explorer;
use windows::graphic_ui::open_window;

fn main() {
    let mut map = Map::new(25, 25, 6);
    if let Some((x, y)) = map.place_robot_near_base() {
        let explorer = Explorer::new(x, y);
        map.place_robot(explorer.x, explorer.y, explorer.icon); // Ajoute l'explorateur sur la carte
        println!("Robot explorateur placé en ({}, {}).", x, y);
    } else {
        println!("!!! Impossible de placer le robot : aucune case libre !");
    }
    //TODO: Remove this line
    map.display_in_terminal();
    open_window(map).unwrap();
}