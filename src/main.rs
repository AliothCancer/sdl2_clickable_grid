pub mod simulation;


use simulation::Simulation;



fn main() {
    
    let mut sim = Simulation::default();
    sim.main_loop();
    
}


