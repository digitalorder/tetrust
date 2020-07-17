pub mod engine {
    use crate::view::{View};
    use crate::playfield as playfield;
    use crate::playfield_ctrl::{PlayfieldCtrl};
    use crate::score_ctrl::{ScoreCtrl};
    use crate::next_tetro_ctrl::{NextTetroCtrl};
    use crate::static_ctrl::{StaticCtrl};
    use crate::updateable_view::Ctrl;
    use crate::fall::{Fall};
    use std::fmt;

    pub struct Config {
        pub no_ghost: bool,
        pub level: u8,
    }

    #[derive(Clone, PartialEq)]
    pub enum State {
        CompletionPhase,
        FallingPhase,
        LockedPhase,
        PatternPhase,
        AnimationPhase,
        /* Eliminate Phase is subphase of Completion */
        GameOver,
    }

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let result = match self {
                State::FallingPhase => "falling",
                State::CompletionPhase => "completion",
                State::GameOver => "gameover",
                State::LockedPhase => "locked",
                State::PatternPhase => "pattern",
                State::AnimationPhase => "animation",
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
        fall: Fall,
    }

    pub fn new_game(config: Config, playfield: playfield::Playfield) -> Game {
        Game {
            playfield: PlayfieldCtrl::new(playfield, config.no_ghost),
            static_ctrl: StaticCtrl::default(),
            next_tetro: NextTetroCtrl::new(),
            score: ScoreCtrl::new(config.level as i8),
            state: State::CompletionPhase,
            fall: Fall::new(),
        }
    }

    pub fn is_finished(game: &Game) -> bool {
        game.state == State::GameOver
    }

    pub fn draw_frame(game: &mut Game, view: &mut impl View) {
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

    fn handle_user_move(game: &mut Game, event: Event) -> (State, bool) {
        let (move_success, fall_space) = match event {
            Event::KeyDown => game.playfield.move_active(playfield::Dir::Down),
            Event::KeyLeft => game.playfield.move_active(playfield::Dir::Left),
            Event::KeyRight => game.playfield.move_active(playfield::Dir::Right),
            Event::KeyTurn => game.playfield.turn_active(),
            Event::KeyDrop => {
                let (move_success, _) = game.playfield.move_active(playfield::Dir::Down);
                while game.playfield.move_active(playfield::Dir::Down) == (true, true || false) {};
                (move_success, false)
            },
            Event::KeyHold => {
                let active_shape = game.playfield.active_shape();

                match game.next_tetro.swap(active_shape) {
                    Ok(tetro) => {game.playfield.new_active(tetro);},
                    _ => /* do nothing */{},
                };
                (true, true)
            },
            _ => (true, true),
        };

        if !move_success && event == Event::KeyDown {
            (State::PatternPhase, true)
        } else if move_success && !fall_space {
            game.fall.lock_delay();
            (State::LockedPhase, false)
        } else {
            (State::FallingPhase, false)
        }
    }

    pub fn calculate_frame(game: &mut Game, event: Event) {
        let mut reschedule = true;
        while reschedule {
            let result = match game.state {
                State::FallingPhase | State::LockedPhase => {
                    /* replace timeout drop with KeyDown event to simplify further handling */
                    let event = if event == Event::Timeout && game.fall.inc_frame_counter(game.score.level) {
                        Event::KeyDown
                    } else {
                        event.clone()
                    };

                    handle_user_move(game, event.clone())
                },
                State::PatternPhase => {
                    game.playfield.place_active();
                    game.playfield.start_animation();
                    (State::AnimationPhase, true)
                },
                State::AnimationPhase => {
                    if event == Event::Timeout && !game.playfield.animate() {
                        (State::CompletionPhase, true)
                    } else {
                        (State::AnimationPhase, false)
                    }
                },
                State::CompletionPhase => {
                    /* elimimination phase */
                    let removed_rows_count = game.playfield.remove_filled();
                    /* completion phase */
                    game.score.update(removed_rows_count as u8);
                    game.fall.reset();
                    (create_new_tetro(game), false)
                },
                State::GameOver => {
                    /* do nothing for now */
                    (State::GameOver, false)
                }
            };
            game.state = result.0;
            reschedule = result.1;
        }
    }
}
