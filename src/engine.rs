pub mod engine {
    use crate::view::{View, ShowArgs};
    use crate::playfield as playfield;
    use crate::playfield_ctrl::{PlayfieldCtrl};
    use crate::score_ctrl::{ScoreCtrl};
    use crate::figures::figures as figures;
    use crate::updateable_view::UpdatableView;
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

    struct StaticUpdatableView {
        view: UpdatableView,
    }

    impl StaticUpdatableView {
        fn show(self: &mut Self, view: &impl View) {
            self.view.show(view, &ShowArgs::StaticArgs{});
        }
    }

    impl Default for StaticUpdatableView {
        fn default() -> Self {StaticUpdatableView{view: UpdatableView::default()}}
    }

    struct NextView {
        view: UpdatableView,
        next_tetro: figures::Tetromino,
    }

    impl NextView {
        fn new_tetro(self: &mut Self) {
            self.next_tetro = figures::Tetromino::new_random();
            self.view.update();
        }

        fn show(self: &mut Self, view: &impl View) {
            self.view.show(view, &ShowArgs::NextTetroArgs{tetro: self.next_tetro.clone()});
        }

        fn new() -> Self {
            NextView{view: UpdatableView::default(), next_tetro: figures::Tetromino::new_random()}
        }
    }

    pub struct Game {
        playfield: PlayfieldCtrl,
        state: State,
        next_tetro_view: NextView,
        static_view: StaticUpdatableView,
        score: ScoreCtrl,
    }

    pub fn new_game(config: Config, playfield: playfield::Playfield) -> Game {
        Game {
            playfield: PlayfieldCtrl::new(playfield, config.no_ghost),
            static_view: StaticUpdatableView::default(),
            next_tetro_view: NextView::new(),
            score: ScoreCtrl::new(config.level as i8),
            state: State::Dropped,
        }
    }

    pub fn is_finished(game: &Game) -> bool {
        game.state == State::End
    }

    pub fn draw_frame(game: &mut Game, view: &impl View) {
        game.score.show(view);
        game.static_view.show(view);
        game.next_tetro_view.show(view);
        game.playfield.show(view);
    }

    fn create_new_tetro(game: &mut Game) -> State {
        let no_intersect = game.playfield.new_active(&game.next_tetro_view.next_tetro);
        game.next_tetro_view.new_tetro();
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
