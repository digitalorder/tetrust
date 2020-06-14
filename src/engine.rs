pub mod engine {
    use crate::view::View as View;
    use crate::playfield as playfield;
    use crate::figures::figures as figures;

    pub struct Game<'a> {
        playfield: playfield::Playfield,
        view: &'a dyn View,
        active_tetro: playfield::ActiveTetromino,
    }

    pub fn new_game(playfield: playfield::Playfield, view: &impl View) -> Game {
        Game{
            view: view,
            playfield: playfield,
            active_tetro: Default::default(),
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
        if game.active_tetro.shape == figures::Shape::NoShape {
            let _ = game.playfield.new_active(
                figures::Shape::OShape,
                &playfield::Coords{row: playfield::HEIGHT,
                    col: playfield::WIDTH / 2 - 2}
            );
        }
    }
}
