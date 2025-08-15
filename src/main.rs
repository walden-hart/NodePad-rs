mod graph;
mod app;

use crate::app::NodePadApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "NodePad-rs",
        options,
        Box::new(|_cc| Ok(Box::<NodePadApp>::default())),
    )
}
