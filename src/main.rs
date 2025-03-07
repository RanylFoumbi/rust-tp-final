mod simulation;
mod environment;
mod robots;
mod windows;

use simulation::simulation::Simulation;

fn main() {
    let mut simulation = Simulation::new();
    simulation.run();
}