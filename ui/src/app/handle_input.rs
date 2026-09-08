use super::*;
use eframe::egui;


impl App { 
    pub fn handle_input(&mut self, ui: &mut egui::Ui) -> () {

        self.handle_board_input(ui);
        
        // autoplay
        if ui.input(|i| i.key_pressed(egui::Key::Space)) {
            println!("{}", self.game);
            self.game.autoplay();
            self.update_state();
        }

        // print game
        if ui.input(|i| i.key_pressed(egui::Key::C)) {
            println!("{}", self.game);
        }

    }



    fn handle_board_input(&mut self, ui: &mut egui::Ui) -> () {
        // drag started
        if ui.input(|i| i.pointer.primary_pressed()) {
            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                if 0.0 < pos.x && pos.x < self.square_size *8.0 && 0.0 < pos.y && pos.y < self.square_size *8.0 {
                    let chess_pos = ((pos.y / self.square_size) as u8)*8 + (pos.x / self.square_size) as u8;
                    
                    match self.selected {
                        // if some piece is already selected
                        Some(selected) => {
                            // if move is legal
                            if self.state.legal[selected].get(chess_pos) {}
                            // choose other friendly piece other than selected
                            else if self.state.board[chess_pos].is_some_and(|p| p.color == self.state.player) {
                                self.selected = Some(chess_pos);
                                self.dragged = Some(pos);
                            }
                            // reset selected
                            else {
                                self.selected = None;
                                self.dragged = None;
                            }
                        },
                        None => {
                            // choose friendly piece
                            if self.state.board[chess_pos].is_some_and(|p| p.color == self.state.player) {
                                self.selected = Some(chess_pos);
                                self.dragged = Some(pos);
                            }
                        }
                    }
                    
                } else {
                    self.selected = None;
                }
            } else {
                self.selected = None;
            }
        }


        // if dragging
        if ui.input(|i| i.pointer.is_decidedly_dragging()) {
            // update piece pos to follow the mouse
            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                if self.dragged.is_some() {
                    self.dragged = Some(pos)
                }
            }
        }


        // drag ended
        if ui.input(|i| i.pointer.primary_released()) {
            if let Some(selected) = self.selected {
                let pos = ui.input(|i| i.pointer.interact_pos());
                if let Some(pos) = pos {
                    if 0.0 < pos.x && pos.x < self.square_size *8.0 && 0.0 < pos.y && pos.y < self.square_size *8.0 {
                        let chess_pos = ((pos.y / self.square_size) as u8)*8 + (pos.x / self.square_size) as u8;
                        
                        if Some(chess_pos) != self.selected {
                            if timed(|| self.game.play((selected, chess_pos))) {
                                println!("eval: {}", timed(|| self.game.nnue_eval()));
                                self.update_state();
                                self.selected = None;
                            }
                        }

                    } else {
                        self.selected = None;
                    }
                }
            }
            self.dragged = None;
        }
    }





    fn update_state(&mut self) {
        self.state = self.game.state();
    }
}