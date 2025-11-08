use metalforge_lib::engine::Engine;
use crate::ui::UI;

mod ui;

fn main() {
    log::info!("Initialising application...");

    let mut engine = Engine::new();
    let mut ui = UI::new(&mut engine);

    ui.run();

    log::info!("Exiting...");
}
