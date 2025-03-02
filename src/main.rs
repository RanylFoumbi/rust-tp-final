mod environment {
    pub mod map;
    pub mod tile;
}
mod simulation {
    pub mod simulation;
}
mod robots{
    pub mod robot;
    pub mod harvester;
    pub mod explorer;
}
mod windows{
    pub mod graphic_ui;
    pub mod utils;
}

use simulation::simulation::Simulation;

fn main() {
    let mut simulation = Simulation::new();
    simulation.run();
}