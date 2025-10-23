mod graph;
mod app;

use crate::app::NodePadApp;
use log::info;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use log::LevelFilter;

    simple_logger::SimpleLogger::new().with_level(LevelFilter::Debug).init().unwrap();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "NodePad-rs",
        options,
        Box::new(|cc| {
            if let Some(render_state) = cc.wgpu_render_state.as_ref() {
                let info = render_state.adapter.get_info();
                info!("Using WGPU backend: {}", info.backend);
                info!("Device name: {}", info.name);
            }
            Ok(Box::<NodePadApp>::default())
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast;
    use log::Level;

    console_log::init_with_level(Level::Debug).expect("the console log didn't initialize");

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let _ = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    if let Some(render_state) = cc.wgpu_render_state.as_ref() {
                        let info = render_state.adapter.get_info();
                        info!("Using WGPU backend: {}", info.backend);
                        info!("Device name: {}", info.name);
                    }
                    Ok(Box::new(NodePadApp::default()))
                }),
            )
            .await;
    });
}