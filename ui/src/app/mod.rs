use engine::*;
use eframe::egui;

mod render;
use std::collections::HashMap;

mod handle_input;


// mod render;

struct App {
    game: Game,

    textures: HashMap<String, egui::TextureHandle>,

    selected: Option<Pos>,
    dragged: Option<egui::Pos2>,

    square_size: f32,
}

impl App {
    fn new(cc: &eframe::CreationContext) -> Self {
        App {
            game: Game::new(Board::default()),

            textures: App::load_textures(cc),

            selected: None,
            dragged: None,

            square_size: 80.0,
        }
    }
}


impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        self.handle_input(ui);
        
        self.render(ui);
    }
}


pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 640.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "MiChess", options,
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(App::new(cc)))),
    );
}