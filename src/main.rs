use app::App;
pub use eframe::egui;

mod map;
mod app;


fn main() -> eframe::Result {

    let native_options = eframe::NativeOptions {
        viewport     : egui::ViewportBuilder::default()
                        .with_min_inner_size([600.0, 600.0]),
                        ..Default::default()
    };
    eframe::run_native("deneme", native_options, Box::new(|cc| Ok(Box::new(App::default()))))
}
