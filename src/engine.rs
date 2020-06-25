pub mod engine {
    use crate::view::View as View;
    use crate::playfield as playfield;
    use crate::figures::figures as figures;
    use std::fmt;
    use std::cmp;

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
            };

            write!(f, "{}", result)
        }
    }

    pub struct Game<'a> {
        playfield: playfield::Playfield,
        view: &'a dyn View,
        state: State,
        active_tetro: playfield::FieldTetromino,
        next_tetro: figures::Tetromino,
        lines_cleared: u32,
        level: u8,
        score: u32,
        frame_counter: u8,
        view_outdated: bool,
    }

    pub fn new_game(playfield: playfield::Playfield, view: &impl View) -> Game {
        Game {
            view: view,
            playfield: playfield,
            state: State::Dropped,
            active_tetro: playfield::FieldTetromino::default(),
            next_tetro: figures::Tetromino::new_random(),
            lines_cleared: 0,
            level: 0,
            score: 0,
            frame_counter: 0,
            view_outdated: true,
        }
    }

    pub fn get_state(game: &Game) -> State {
        game.state
    }

    pub fn draw_frame(game: &mut Game) {
        if game.view_outdated {
            game.view.show_static(game.level, game.score, game.lines_cleared);
            game.view.show_next(&game.next_tetro);
            game.view_outdated = false;
            let mut ghost_tetro = game.active_tetro;
            while game.playfield.move_tetro(&mut ghost_tetro, playfield::Dir::Down) {};
            game.view.show_playfield(&game.playfield, &game.active_tetro, &ghost_tetro);
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
        game.view_outdated = true;
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

    fn score_increment(level: u8, cleared_lines: u8) -> u32 {
        /* Level 1 line         2 lines         3 lines         4 lines
         * 0     40             100             300             1200
         * 1     80             200             600             2400
         * 2     120            300             900             3600
         * .......
         * n     40 * (n + 1)   100 * (n + 1)   300 * (n + 1)   1200 * (n + 1)
         */
        let line_coeff: u32 = match cleared_lines {
            1 => 40,
            2 => 100,
            3 => 300,
            _ => 1200,
        };

        line_coeff * (level as u32 + 1)
    }

    fn move_active(game: &mut Game, dir: playfield::Dir) -> bool {
        game.view_outdated = true;
        game.playfield.move_tetro(&mut game.active_tetro, dir)
    }

    fn turn_active(game: &mut Game) -> bool {
        game.view_outdated = true;
        game.playfield.turn_tetro(&mut game.active_tetro)
    }

    fn inc_frame_counter(game: &mut Game) -> bool {

        let max_frame_count = match game.level {
            0..=8 => 48 - 5 * game.level,
            9 => 6,
            10..=12 => 5,
            13..=15 => 4,
            16..=18 => 3,
            19..=28 => 2,
            _ => 1
        };

        game.frame_counter += 1;
        if game.frame_counter >= max_frame_count {
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
                    while move_active(game, playfield::Dir::Down) {};
                    game.frame_counter = 30;
                }
            },
            State::Touched => {
                game.state = State::Dropped;
            },
            State::Dropped => {
                if event == Event::Timeout {
                    let removed = remove_filled(&mut game.playfield);
                    game.lines_cleared += removed as u32;
                    game.level = cmp::max(game.level, (game.lines_cleared / 10) as u8);

                    if removed > 0 {
                        game.score += score_increment(game.level, removed);
                    } else {
                        game.state = create_new_tetro(game);
                    }
                }
            },
            State::End => {
                /* do nothing for now */
            }
        }
    }
}
