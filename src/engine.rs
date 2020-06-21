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

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let result = match self {
                State::ActiveTetro => "active",
                State::Touched => "touched",
                State::Dropped => "dropped",
                State::End => "end",
            };

            write!(f, "{}", result)
        }
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
                Event::KeyDown => "⬇️",
                Event::KeyExit => "🚪",
                Event::KeyTurn => "🔁",
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
        game.view.show_static();
        game.view.show_playfield(&game.playfield);
    }

    fn row_filled(playfield: &playfield::Playfield, row: i8) -> bool {
        for c in 0..playfield::WIDTH {
            let (shape, is_active) = playfield.shape_at(&playfield::Coords{col: c, row: row});
            if !is_active && shape == figures::Shape::NoShape {
                return false;
            }
        }

        return true;
    }

    fn remove_filled(playfield: &mut playfield::Playfield) -> bool {
        let mut result = false;

        for r in (0..playfield::HEIGHT).rev() {
            if row_filled(&playfield, r) {
                result = true;
                playfield.delete_row(r);
            }
        }

        result
    }

    fn create_new_tetro(game: &mut Game) {
        let tetro = figures::Tetromino::new_random();
        let place_coords = playfield::Coords{row: playfield::HEIGHT - 1,
                col: playfield::WIDTH / 2 - 2};

        game.playfield.new_active(&tetro, &place_coords);

        if game.playfield.can_place(&tetro, &place_coords) {
            game.state = State::ActiveTetro;
        } else {
            game.state = State::End;
        };
    }

    pub fn calculate_frame(game: &mut Game, event: Event) {
        match game.state {
            State::ActiveTetro => {
                if event == Event::Timeout || event == Event::KeyDown {
                    if !game.playfield.move_active(playfield::Dir::Down) {
                        let _ = game.playfield.place_active();
                        game.state = State::Dropped;
                    }
                } else if event == Event::KeyLeft {
                    game.playfield.move_active(playfield::Dir::Left);
                } else if event == Event::KeyRight {
                    game.playfield.move_active(playfield::Dir::Right);
                } else if event == Event::KeyTurn {
                    game.playfield.turn_active();
                }
            },
            State::Touched => {
                game.state = State::Dropped;
            },
            State::Dropped => {
                if event == Event::Timeout {
                    if !remove_filled(&mut game.playfield) {
                        create_new_tetro(game);
                    }
                }
            },
            State::End => {
                /* do nothing for now */
            }
        }
    }
}
