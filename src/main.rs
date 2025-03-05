mod simulation;
use simulation::Simulation;
mod environment;
mod robots;

fn main() {
    let mut simulation = Simulation::new(25,25, 42);
    simulation.start();
}