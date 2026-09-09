use super::*;
use eframe::egui;
use std::collections::HashMap;

const TEXTURES_NAME: [&str; 12] = [
    "w_pawn", "w_knight", "w_bishop", "w_rook", "w_queen", "w_king",
    "b_pawn", "b_knight", "b_bishop", "b_rook", "b_queen", "b_king",
];



impl App {
    pub fn load_textures(cc: &eframe::CreationContext) -> HashMap<String, egui::TextureHandle> {
        // upload textures on init
        let mut textures = HashMap::new();

        for texture_name in TEXTURES_NAME {
            let path = format!("{}/textures/{}.png", env!("CARGO_MANIFEST_DIR"), texture_name);
            let image = image::open(path).unwrap();
            let size = [image.width() as usize, image.height() as usize];
            let pixels = image.to_rgba8();
            let texture = cc.egui_ctx.load_texture(
                texture_name,
                egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                egui::TextureOptions::default(),
            );
            textures.insert(texture_name.to_string(), texture);
        }

        textures
    }



    pub fn render(&self, ui: &mut egui::Ui) -> () {
        self.render_board(ui);
    }


    fn render_board(&self, ui: &mut egui::Ui) -> () {
        let painter: &egui::Painter = ui.painter();
        
        // render board
        for row in 0..8 {
            for col in 0..8 {
                let x = col as f32 * self.square_size;
                let y = row as f32 * self.square_size;
                
                let color = if (row + col) % 2 == 0 {
                    egui::Color32::from_rgb(240, 217, 181)  // light
                } else {
                    egui::Color32::from_rgb(181, 136, 99)   // dark
                };
                
                painter.rect_filled(
                    egui::Rect::from_min_size(egui::pos2(x, y),
                    egui::vec2(self.square_size, self.square_size)),
                    0.0, color,
                );
            }
        }

        
        // render pieces
        for row in 0..8 {
            for col in 0..8 {
                let x = col as f32 * self.square_size;
                let y = row as f32 * self.square_size;

                if let Some(piece) = self.game.state.board[(row*8 + col) as u8] {
                    if !(self.dragged.is_some() && self.selected.is_some_and(|s| s == (row*8 + col) as u8)) {
                        self.render_piece(&painter, piece, egui::pos2(x, y));
                    }

                }
            }
        }

        // render selected
        if let Some(selected) = self.selected {
            for row in 0..8 {
                for col in 0..8 {
                    let x = col as f32 * self.square_size;
                    let y = row as f32 * self.square_size;

                    if self.game.cache.legal[selected].get(row*8 + col) {
                        self.render_legal(&painter,egui::pos2(x, y), self.square_size);
                    }
                }
            }
        }

        // render dragged piece
        if let Some(dragged) = self.dragged && let Some(selected) = self.selected {
            self.render_piece(&painter, self.game.state.board[selected].unwrap(), 
                egui::pos2(dragged.x - self.square_size/2.0, dragged.y - self.square_size/2.0)
            );
        }


    }



    fn render_piece(&self, painter: &egui::Painter, piece: Piece, pos: egui::Pos2) -> () {
        let texture_name = format!("{}_{}", 
            match piece.color { Color::White => "w", Color::Black => "b" },
            match piece.role {
                Role::King   => "king",
                Role::Queen  => "queen",
                Role::Rook   => "rook",
                Role::Bishop => "bishop",
                Role::Knight => "knight",
                Role::Pawn   => "pawn",
            }
        );

        let texture = &self.textures[&texture_name];

        painter.image(
            texture.id(),
            egui::Rect::from_min_size(pos, egui::vec2(self.square_size, self.square_size)),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }


    fn render_legal(&self, painter: &egui::Painter, pos: egui::Pos2, square_size: f32) -> () {
        painter.circle_filled(
            egui::pos2(pos.x + square_size/2.0, pos.y + square_size/2.0), 
            square_size/4.0,                     
            egui::Color32::from_rgba_unmultiplied(30, 30, 30, 80),
        );
    }
}