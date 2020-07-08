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
        PatternPhase,
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

    pub trait Storable {
        fn store(self: &mut Self, row: i8);
        fn count(self: &Self) -> usize;
        fn reset(self: &mut Self);
    }

    pub struct LineStorage {
        lines: [i8; 4],
        write_index: usize,
        read_index: usize,
    }

    impl Storable for LineStorage {
        fn store(self: &mut Self, row: i8) {
            assert_ne!(self.write_index, self.lines.len());
            self.lines[self.write_index] = row;
            self.write_index += 1;
        }

        fn count(self: &Self) -> usize { self.write_index }
        fn reset(self: &mut Self) { self.write_index = 0; }
    }

    impl Iterator for LineStorage {
        type Item = i8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.read_index == self.lines.len() {
                self.read_index = 0;
                return None;
            }
            let result = self.lines[self.read_index];
            self.read_index += 1;
            if self.read_index > self.write_index {
                self.read_index = 0;
                None
            } else {
                Some(result)
            }
        }
    }

    impl Default for LineStorage {
        fn default() -> LineStorage {
            LineStorage{lines: [0, 0, 0, 0], write_index: 0, read_index: 0}
        }
    }

    pub struct Game {
        playfield: PlayfieldCtrl,
        state: State,
        next_tetro: NextTetroCtrl,
        static_ctrl: StaticCtrl,
        score: ScoreCtrl,
        removed: LineStorage,
    }

    pub fn new_game(config: Config, playfield: playfield::Playfield) -> Game {
        Game {
            playfield: PlayfieldCtrl::new(playfield, config.no_ghost),
            static_ctrl: StaticCtrl::default(),
            next_tetro: NextTetroCtrl::new(),
            score: ScoreCtrl::new(config.level as i8),
            state: State::CompletionPhase,
            removed: LineStorage::default(),
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

    fn handle_user_move(game: &mut Game, event: Event) -> State {
        let (move_success, fall_space) = match event {
            Event::KeyDown => game.playfield.move_active(playfield::Dir::Down),
            Event::KeyLeft => game.playfield.move_active(playfield::Dir::Left),
            Event::KeyRight => game.playfield.move_active(playfield::Dir::Right),
            Event::KeyTurn => game.playfield.turn_active(),
            Event::KeyDrop => {
                while game.playfield.move_active(playfield::Dir::Down) == (true, true) {};
                game.playfield.move_active(playfield::Dir::Down)
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
            State::PatternPhase
        } else if move_success && !fall_space {
            game.score.lock_delay();
            State::LockedPhase
        } else {
            State::FallingPhase
        }
    }

    pub fn calculate_frame(game: &mut Game, event: Event) -> bool {
        let mut reschedule = false;
        /* replace timeout drop with KeyDown event to simplify further handling */
        match game.state {
            State::FallingPhase | State::LockedPhase => {
                let event = if event == Event::Timeout && game.score.inc_frame_counter() {
                    Event::KeyDown
                } else {
                    event
                };

                game.state = handle_user_move(game, event);
                if game.state == State::PatternPhase {
                    reschedule = true;
                }
            },
            State::PatternPhase => {
                game.playfield.place_active();
                game.state = State::CompletionPhase;
                reschedule = true;
            },
            State::CompletionPhase => {
                /* elimimination phase */
                game.playfield.find_filled(&mut game.removed);
                game.playfield.remove_filled(&mut game.removed);
                /* completion phase */
                game.score.update(&game.removed);
                game.removed.reset();
                game.state = create_new_tetro(game);
            },
            State::GameOver => {
                /* do nothing for now */
            }
        }
        // print!("{}Event: {} {} ", termion::cursor::Goto(1, 1), event_copy, game.state);
        reschedule
    }
}

#[cfg(test)]
mod tests {
    use super::engine::*;

    #[test]
    fn line_storage_zero_on_creation() {
        let line_storage = LineStorage::default();
        assert_eq!(line_storage.count(), 0);
    }

    #[test]
    fn line_storage_count_one() {
        let mut line_storage = LineStorage::default();
        line_storage.store(0);
        assert_eq!(line_storage.count(), 1);
    }

    #[test]
    fn line_storage_count_two() {
        let mut line_storage = LineStorage::default();
        line_storage.store(0);
        line_storage.store(1);
        assert_eq!(line_storage.count(), 2);
    }

    #[test]
    fn line_storage_iterate_with_next() {
        let mut line_storage = LineStorage::default();
        line_storage.store(0);
        line_storage.store(1);
        assert_eq!(line_storage.next(), Some(0));
        assert_eq!(line_storage.next(), Some(1));
        assert_eq!(line_storage.next(), None);
    }
}
