pub mod engine {
    use crate::view::View as View;
    use crate::playfield as playfield;
    use crate::figures::figures as figures;

    pub struct Game<'a> {
        playfield: playfield::Playfield,
        view: &'a dyn View,
        active_shape: figures::Tetromino,
        active_coords: playfield::Coords,
    }

    pub fn new_game(playfield: playfield::Playfield, view: &impl View) -> Game {
        Game{view: view, playfield: playfield,
            active_shape: figures::Tetromino::new(figures::Shape::NoShape),
            active_coords: playfield::Coords{row: 0, col: 0}
        }
    }

    pub fn draw_frame(game: &Game) {
        println!("+----------+");
        for i in (0..playfield::HEIGHT).rev() {
            let row = game.view.show_row(&game.playfield, i);
            print!("|");
            for c in &row {
                print!("{}", c);
            }
            print!("|\n");
        }
        println!("+----------+");
    }

    pub fn calculate_frame(game: &mut Game) {
        if game.active_shape.shape == figures::Shape::NoShape {
            game.active_shape = figures::Tetromino::new(figures::Shape::OShape);
            game.active_coords = playfield::Coords{row: playfield::HEIGHT,
                                                   col: playfield::WIDTH / 2 - 2};
        }

        let _ = game.playfield.place(&game.active_shape, game.active_coords);
    }
}
