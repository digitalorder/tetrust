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

    struct PlayfieldUpdatableView {
        view: UpdatableView,
        playfield: playfield::Playfield,
        no_ghost: bool,
        active_tetro: playfield::FieldTetromino,
    }

    impl PlayfieldUpdatableView {
        fn move_active(self: &mut Self, dir: playfield::Dir) -> bool {
            if self.playfield.move_tetro(&mut self.active_tetro, dir) {
                self.view.update();
                true
            } else {
                false
            }
        }

        fn turn_active(self: &mut Self) -> bool {
            if self.playfield.turn_tetro(&mut self.active_tetro) {
                self.view.update();
                true
            } else {
                false
            }
        }

        fn place_active(self: &mut Self) {
            let _ = self.playfield.place(&self.active_tetro.tetro, self.active_tetro.coords);
            self.active_tetro.tetro.shape = figures::Shape::NoShape;
            self.view.update();
        }

        fn new_active(self: &mut Self, tetro: &figures::Tetromino) -> bool{
            self.active_tetro = playfield::FieldTetromino{
                coords: playfield::Coords{row: playfield::HEIGHT - 1,
                                          col: playfield::WIDTH / 2 - 2},
                tetro: *tetro
            };
            self.view.update();
            self.playfield.can_place(&self.active_tetro.tetro, &self.active_tetro.coords)
        }

        fn remove_filled(self: &mut Self) -> u8 {
            let mut result = 0;

            for r in (0..playfield::HEIGHT).rev() {
                if self.playfield.row_filled(r) {
                    result += 1;
                    self.playfield.delete_row(r);
                    self.view.update();
                }
            }

            result
        }

        fn show(self: &mut Self, view: &impl View) {
            let ghost_tetro = if self.no_ghost {
                playfield::FieldTetromino::default()
            } else {
                let mut ghost_tetro = self.active_tetro;
                while self.playfield.move_tetro(&mut ghost_tetro, playfield::Dir::Down) {};
                ghost_tetro
            };

            self.view.show(view, &ShowArgs::PlayfieldArgs{
                                    playfield: &self.playfield,
                                    active_tetro: &self.active_tetro,
                                    ghost_tetro: &ghost_tetro
                                 });
        }

        fn new(playfield: playfield::Playfield, no_ghost: bool) -> Self {
            PlayfieldUpdatableView{
                playfield: playfield,
                view: UpdatableView::default(),
                no_ghost: no_ghost,
                active_tetro: playfield::FieldTetromino::default(),
            }
        }
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
            self.view.show(view, &ShowArgs::NextTetroArgs{tetro: self.next_tetro});
        }

        fn new() -> Self {
            NextView{view: UpdatableView::default(), next_tetro: figures::Tetromino::new_random()}
        }
    }

    pub struct Game {
        playfield: PlayfieldUpdatableView,
        state: State,
        next_tetro_view: NextView,
        static_view: StaticUpdatableView,
        score: Score,
        frame_counter: i8,
    }

    pub fn new_game(config: Config, playfield: playfield::Playfield) -> Game {
        Game {
            playfield: PlayfieldUpdatableView::new(playfield, config.no_ghost),
            static_view: StaticUpdatableView::default(),
            next_tetro_view: NextView::new(),
            score: Score::new(config.level as i8),
            state: State::Dropped,
            frame_counter: 0,
        }
    }

    pub fn get_state(game: &Game) -> State {
        game.state
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
                    if inc_frame_counter(game) {
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
