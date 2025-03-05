mod simulation;
mod environment;
mod robots;
mod windows;

use crate::simulation::Simulation;
use crate::windows::open_window;

fn main() -> iced::Result {
    let simulation = Simulation::new(25, 25, 42);
    simulation.start();
    open_window(simulation)
}
