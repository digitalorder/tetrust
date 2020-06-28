pub mod engine {
    use crate::view::{View, ShowArgs};
    use crate::playfield as playfield;
    use crate::figures::figures as figures;
    use std::fmt;
    use std::cmp;

    pub struct Config {
        pub no_ghost: bool,
        pub level: u8,
    }

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

    struct UpdatableView {
        updated: bool,
    }

    impl UpdatableView {
        fn update(self: &mut Self) {
            self.updated = true;
        }

        fn show(self: &mut Self, view: &impl View, args: &ShowArgs) {
            if self.updated {
                view.show_subview(args);
            }
            self.updated = false;
        }
    }

    impl Default for UpdatableView {
        fn default() -> Self {UpdatableView{updated: true}}
    }

    type StaticUpdatableView = UpdatableView;

    struct Score {
        view: UpdatableView,
        level: i8,
        score: u32,
        lines_cleared: u32,
    }

    impl Score {
        fn score_increment(level: i8, cleared_lines: u8) -> u32 {
            /* Level 1 line         2 lines         3 lines         4 lines
             * 0     40             100             300             1200
             * 1     80             200             600             2400
             * 2     120            300             900             3600
             * .......
             * n     40 * (n + 1)   100 * (n + 1)   300 * (n + 1)   1200 * (n + 1)
             */
            let line_coeff: u32 = match cleared_lines {
                0 => 0,
                1 => 40,
                2 => 100,
                3 => 300,
                _ => 1200,
            };

            line_coeff * (level as u32 + 1)
        }

        fn update(self: &mut Self, lines_cleared: u8) {
            self.lines_cleared += lines_cleared as u32;
            self.level = cmp::max(self.level, (self.lines_cleared / 10) as i8);
            self.score += Score::score_increment(self.level, lines_cleared);
            self.view.update();
        }

        fn show(self: &mut Self, view: &impl View) {
            self.view.show(view, &ShowArgs::ScoreArgs{level: self.level, lines: self.lines_cleared, score: self.score});
        }

        fn new(level: i8) -> Self {
            Score {
                view: UpdatableView::default(),
                level: if level < 29 { level as i8 } else { 29 },
                score: 0,
                lines_cleared: 0
            }
        }
    }

    pub struct Game {
        playfield: playfield::Playfield,
        state: State,
        active_tetro: playfield::FieldTetromino,
        next_updated: bool,
        next_tetro: figures::Tetromino,
        static_view: StaticUpdatableView,
        score: Score,
        frame_counter: i8,
        playfield_updated: bool,
        no_ghost: bool,
    }

    pub fn new_game(config: Config, playfield: playfield::Playfield) -> Game {
        Game {
            playfield: playfield,
            static_view: StaticUpdatableView::default(),
            score: Score::new(config.level as i8),
            state: State::Dropped,
            active_tetro: playfield::FieldTetromino::default(),
            next_updated: true,
            next_tetro: figures::Tetromino::new_random(),
            frame_counter: 0,
            playfield_updated: true,
            no_ghost: config.no_ghost,
        }
    }

    pub fn get_state(game: &Game) -> State {
        game.state
    }

    pub fn draw_frame(game: &mut Game, view: &impl View) {
        game.score.show(view);
        game.static_view.show(view, &ShowArgs::StaticArgs{});
        if game.next_updated {
            game.next_updated = false;
            view.show_next(&mut game.next_tetro);
        }
        if game.playfield_updated {
            game.playfield_updated = false;
            let ghost_tetro = if game.no_ghost {
                playfield::FieldTetromino::default()
            } else {
                let mut ghost_tetro = game.active_tetro;
                while game.playfield.move_tetro(&mut ghost_tetro, playfield::Dir::Down) {};
                ghost_tetro
            };
            view.show_playfield(&game.playfield, &game.active_tetro, &ghost_tetro);
        }
    }

    fn remove_filled(playfield: &mut playfield::Playfield) -> u8 {
        let mut result = 0;

        for r in (0..playfield::HEIGHT).rev() {
            if playfield.row_filled(r) {
                result += 1;
                playfield.delete_row(r);
            }
        }

        result
    }

    fn create_new_tetro(game: &mut Game) -> State {
        game.next_updated = true;
        game.active_tetro = playfield::FieldTetromino{
            coords: playfield::Coords{row: playfield::HEIGHT - 1,
                                      col: playfield::WIDTH / 2 - 2},
            tetro: game.next_tetro
        };
        game.next_tetro = figures::Tetromino::new_random();

        if game.playfield.can_place(&game.active_tetro.tetro, &game.active_tetro.coords) {
            State::ActiveTetro
        } else {
            State::End
        }
    }

    fn move_active(game: &mut Game, dir: playfield::Dir) -> bool {
        if game.playfield.move_tetro(&mut game.active_tetro, dir) {
            game.playfield_updated = true;
            true
        } else {
            false
        }
    }

    fn turn_active(game: &mut Game) -> bool {
        if game.playfield.turn_tetro(&mut game.active_tetro) {
            game.playfield_updated = true;
            true
        } else {
            false
        }
    }

    fn max_frame_count(level: i8) -> i8 {
        match level {
            0..=8 => 48 - 5 * level,
            9 => 6,
            10..=12 => 5,
            13..=15 => 4,
            16..=18 => 3,
            19..=28 => 2,
            _ => 1
        }
    }

    fn inc_frame_counter(game: &mut Game) -> bool {
        game.frame_counter += 1;
        if game.frame_counter >= max_frame_count(game.score.level) {
            game.frame_counter = 0;
            true
        } else {
            false
        }
    }

    fn move_down(game: &mut Game) -> State {
        if !move_active(game, playfield::Dir::Down) {
            let _ = game.playfield.place(&game.active_tetro.tetro, game.active_tetro.coords);
            game.active_tetro.tetro.shape = figures::Shape::NoShape;
            State::Dropped
        } else {
            State::ActiveTetro
        }
    }

    pub fn calculate_frame(game: &mut Game, event: Event) {
        match game.state {
            State::ActiveTetro => {
                if event == Event::Timeout {
                    if inc_frame_counter(game) {
                        game.state = move_down(game);
                    }
                } else if event == Event::KeyDown {
                    game.state = move_down(game);
                } else if event == Event::KeyLeft {
                    move_active(game, playfield::Dir::Left);
                } else if event == Event::KeyRight {
                    move_active(game, playfield::Dir::Right);
                } else if event == Event::KeyTurn {
                    turn_active(game);
                } else if event == Event::KeyDrop {
                    while move_active(game, playfield::Dir::Down) {
                        /* reset frame counter only if there was a drop,
                         * otherwise holding drop key would effectively
                         * freeze game
                         */
                        game.frame_counter = max_frame_count(game.score.level) - 30;
                    };
                } else if event == Event::KeyHold {
                    game.frame_counter = 0;
                }
            },
            State::Touched => {
                game.state = State::Dropped;
            },
            State::Dropped => {
                if event == Event::Timeout {
                    let removed = remove_filled(&mut game.playfield);
                    game.score.update(removed);
                    game.state = create_new_tetro(game);
                    game.playfield_updated = true;
                }
            },
            State::End => {
                /* do nothing for now */
            }
        }
    }
}
