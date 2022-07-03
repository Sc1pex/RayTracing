mod app;
mod ray;
mod ray_tracer;

use app::App;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Ray Tracing",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
