#![deny(clippy::all)]
#![forbid(unsafe_code)]
use app::App;
pub use eframe::egui;
use tokio::runtime::Runtime;

mod map;
mod app;
mod udp_worker;

fn main() /* -> eframe::Result */ {

    let native_options = eframe::NativeOptions {
        viewport     : egui::ViewportBuilder::default()
                        .with_min_inner_size([600.0, 600.0]),
                        ..Default::default()
    };

    eframe::run_native(
        "deneme", 
        native_options, 
        Box::new(|cc| Ok(Box::new(App::new(map::Map::new(1.0, 500, 500), Some(50.0))))));
}
