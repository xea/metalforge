// 17use metalforge_lib::sanity_check;

use crate::ui::UI;

mod ui;

fn main() {
    let mut ui = UI::new();

    ui.run();
}
