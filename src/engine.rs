pub mod engine {
    use crate::view::{View};
    use crate::playfield as playfield;
    use crate::playfield_ctrl::{PlayfieldCtrl};
    use crate::score_ctrl::{ScoreCtrl};
    use crate::next_tetro_ctrl::{NextTetroCtrl};
    use crate::static_ctrl::{StaticCtrl};
    use std::fmt;

    pub struct Config {
        pub no_ghost: bool,
        pub level: u8,
    }

    #[derive(PartialEq)]
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

    #[derive(Clone, PartialEq)]
    pub enum Event {
        Timeout,
        KeyLeft,
        KeyRight,
        KeyTurn,
        KeyDown,
        KeyDrop,
        KeyHold,
        KeyExit,
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
                Event::KeyDrop => "⬆️",
                Event::KeyHold => "✋",
            };

            write!(f, "{}", result)
        }
    }

    pub struct Game {
        playfield: PlayfieldCtrl,
        state: State,
        next_tetro: NextTetroCtrl,
        static_ctrl: StaticCtrl,
        score: ScoreCtrl,
    }

    pub fn new_game(config: Config, playfield: playfield::Playfield) -> Game {
        Game {
            playfield: PlayfieldCtrl::new(playfield, config.no_ghost),
            static_ctrl: StaticCtrl::default(),
            next_tetro: NextTetroCtrl::new(),
            score: ScoreCtrl::new(config.level as i8),
            state: State::Dropped,
        }
    }

    pub fn is_finished(game: &Game) -> bool {
        game.state == State::End
    }

    pub fn draw_frame(game: &mut Game, view: &impl View) {
        game.score.show(view);
        game.static_ctrl.show(view);
        game.next_tetro.show(view);
        game.playfield.show(view);
    }

    fn create_new_tetro(game: &mut Game) -> State {
        let no_intersect = game.playfield.new_active(&game.next_tetro.pop());
        if no_intersect {
            State::ActiveTetro
        } else {
            State::End
        }
    }

    fn move_down(game: &mut Game) -> State {
        let move_success = game.playfield.move_active(playfield::Dir::Down);

        if move_success {
            State::ActiveTetro
        } else {
            game.playfield.place_active();
            State::Dropped
        }
    }

    pub fn calculate_frame(game: &mut Game, event: Event) {
        match game.state {
            State::ActiveTetro => {
                if event == Event::Timeout {
                    if game.score.inc_frame_counter() {
                        game.state = move_down(game);
                    }
                } else if event == Event::KeyDown {
                    game.state = move_down(game);
                } else if event == Event::KeyLeft {
                    game.playfield.move_active(playfield::Dir::Left);
                } else if event == Event::KeyRight {
                    game.playfield.move_active(playfield::Dir::Right);
                } else if event == Event::KeyTurn {
                    game.playfield.turn_active();
                } else if event == Event::KeyDrop {
                    while game.playfield.move_active(playfield::Dir::Down) {
                        /* reset frame counter only if there was a drop,
                         * otherwise holding drop key would effectively
                         * freeze game
                         */
                        game.score.lock_delay();
                    };
                } else if event == Event::KeyHold {
                    game.score.hold();
                }
            },
            State::Touched => {
                game.state = State::Dropped;
            },
            State::Dropped => {
                if event == Event::Timeout {
                    let removed = game.playfield.remove_filled();
                    game.score.update(removed);
                    game.state = create_new_tetro(game);
                }
            },
            State::End => {
                /* do nothing for now */
            }
        }
    }
}
