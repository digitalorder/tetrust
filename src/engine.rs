pub mod engine {
    use crate::view::View as View;
    use crate::playfield as playfield;
    use crate::figures::figures as figures;
    use std::fmt;

    #[derive(Copy, Clone, PartialEq)]
    pub enum State {
        Dropped,
        ActiveTetro,
        Touched,
        End,
    }

    #[derive(Copy, Clone, PartialEq)]
    pub enum Event {
        Timeout,
        KeyLeft,
        KeyRight,
        KeyTurn,
        KeyDown,
        KeyExit
    }

    impl fmt::Display for Event {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let result = match self {
                Event::Timeout => "🕒",
                Event::KeyLeft => "⬅️",
                Event::KeyRight => "➡️",
                Event::KeyExit => "🚪",
                _ => "❓",
            };

            write!(f, "{}", result)
        }
    }

    pub struct Game<'a> {
        playfield: playfield::Playfield,
        view: &'a dyn View,
        state: State,
    }

    pub fn new_game(playfield: playfield::Playfield, view: &impl View) -> Game {
        Game{
            view: view,
            playfield: playfield,
            state: State::Dropped,
        }
    }

    pub fn get_state(game: &Game) -> State {
        game.state
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

    pub fn calculate_frame(game: &mut Game, event: Event) {
        if event == Event::Timeout {
            if game.state == State::Dropped {
                match game.playfield.new_active(
                    figures::Shape::OShape,
                    &playfield::Coords{row: playfield::HEIGHT,
                        col: playfield::WIDTH / 2 - 2}
                ) {
                    Ok(()) => {
                        game.state = State::ActiveTetro
                    },
                    Err(_) => {
                        game.state = State::End
                    }
                };
            } else if game.state == State::ActiveTetro {
                if !game.playfield.move_active(playfield::Dir::Down) {
                    let _ = game.playfield.place_active();
                    game.state = State::Touched;
                }
            } else if game.state == State::Touched {
                game.state = State::Dropped;
            }
        } else if event == Event::KeyLeft && (game.state == State::ActiveTetro || game.state == State::Touched) {
            game.playfield.move_active(playfield::Dir::Left);
        } else if event == Event::KeyRight && (game.state == State::ActiveTetro || game.state == State::Touched) {
            game.playfield.move_active(playfield::Dir::Right);
        }
    }
}
