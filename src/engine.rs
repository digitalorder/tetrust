pub mod engine {
    use crate::view::{View};
    use crate::playfield as playfield;
    use crate::playfield_ctrl::{PlayfieldCtrl};
    use crate::score_ctrl::{ScoreCtrl};
    use crate::next_tetro_ctrl::{NextTetroCtrl};
    use crate::static_ctrl::{StaticCtrl};
    use crate::updateable_view::Ctrl;
    use std::fmt;

    pub struct Config {
        pub no_ghost: bool,
        pub level: u8,
    }

    #[derive(PartialEq)]
    pub enum State {
        CompletionPhase,
        FallingPhase,
        LockedPhase,
        GameOver,
    }

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let result = match self {
                State::FallingPhase => "falling",
                State::CompletionPhase => "completion",
                State::GameOver => "gameover",
                State::LockedPhase => "locked",
            };

            write!(f, "{}", result)
        }
    }

    #[derive(Clone, PartialEq)]
    pub enum Event {
        Timeout,
        Reschedule,
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
                Event::Reschedule => "R",
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
            state: State::CompletionPhase,
        }
    }

    pub fn is_finished(game: &Game) -> bool {
        game.state == State::GameOver
    }

    pub fn draw_frame(game: &mut Game, view: &impl View) {
        game.score.show(view);
        game.static_ctrl.show(view);
        game.next_tetro.show(view);
        game.playfield.show(view);
    }

    fn create_new_tetro(game: &mut Game) -> State {
        if game.playfield.new_active(game.next_tetro.pop()) {
            /* tetro can be placed in start position */
            State::FallingPhase
        } else {
            State::GameOver
        }
    }

    fn is_user_move(event: Event) -> bool {
        event == Event::KeyLeft || event == Event::KeyRight || event == Event::KeyTurn || event == Event::KeyDown || event == Event::KeyDrop || event == Event::KeyHold
    }

    fn handle_user_move(game: &mut Game, event: Event) -> State {
        let (move_success, fall_space) =
            if event == Event::KeyDown {
                game.playfield.move_active(playfield::Dir::Down)
            } else if event == Event::KeyLeft {
                game.playfield.move_active(playfield::Dir::Left)
            } else if event == Event::KeyRight {
                game.playfield.move_active(playfield::Dir::Right)
            } else if event == Event::KeyTurn {
                game.playfield.turn_active()
            } else if event == Event::KeyDrop {
                while game.playfield.move_active(playfield::Dir::Down) == (true, true) {};
                game.playfield.move_active(playfield::Dir::Down)
            } else if event == Event::KeyHold {
                let active_shape = game.playfield.active_shape();

                match game.next_tetro.swap(active_shape) {
                    Ok(tetro) => {game.playfield.new_active(tetro);},
                    _ => /* do nothing */{},
                };
                (true, true)
            } else {
                (true, true)
            };

        if !move_success && event == Event::KeyDown {
            State::CompletionPhase
        } else if move_success && !fall_space {
            game.score.lock_delay();
            State::LockedPhase
        } else {
            State::FallingPhase
        }
    }

    pub fn calculate_frame(game: &mut Game, event: Event) -> bool {
        let mut reschedule = false;
        match game.state {
            State::FallingPhase => {
                if event == Event::Timeout {
                    if game.score.inc_frame_counter() {
                        game.state = handle_user_move(game, Event::KeyDown);
                    }
                } else if is_user_move(event.clone()) {
                    game.state = handle_user_move(game, event);
               }
            },
            State::LockedPhase => {
                if event == Event::Timeout {
                    if game.score.inc_frame_counter() {
                        game.state = State::CompletionPhase;
                        reschedule = true;
                    }
                } else if is_user_move(event.clone()) {
                    game.state = handle_user_move(game, event);
                }
            }
            State::CompletionPhase => {
                if event == Event::Timeout || event == Event::Reschedule {
                    game.playfield.place_active();
                    let removed = game.playfield.remove_filled();
                    game.score.update(removed);
                    game.state = create_new_tetro(game);
                }
            },
            State::GameOver => {
                /* do nothing for now */
            }
        }
        reschedule
    }
}
