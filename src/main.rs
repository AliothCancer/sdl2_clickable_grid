mod simulation;
mod window;

use window::SimWindow;
use simulation::Simulation;
fn main(){
    let mut sim = Simulation::new();

    let title = "Fluid simulation";
    let mut sim_win = SimWindow::new(title,sim);
    sim_win.main_loop();

}