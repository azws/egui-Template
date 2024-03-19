use eframe::egui;


fn main() -> Result<(), eframe::Error>{
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([800_f32, 600_f32]),
        ..Default::default()
    };
    eframe::run_native(
        "egui App",
        options,
        Box::new(|_cc| {
            Box::<App>::default()
        })
    )
}


struct App{

}


impl Default for App {
    fn default() -> Self {
        Self {

        }
    }
}


impl App {

}


impl eframe::App for App{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
}
