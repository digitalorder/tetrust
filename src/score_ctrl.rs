use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};
use std::cmp;

pub struct ScoreCtrl {
    view: UpdatableView,
    frame_counter: i8,
    level: i8,
    score: u32,
    lines_cleared: u32,
}

impl ScoreCtrl {
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

    pub fn update(self: &mut Self, lines: u8) {
        self.lines_cleared += lines as u32;
        self.level = cmp::max(self.level, (self.lines_cleared / 10) as i8);
        self.score += ScoreCtrl::score_increment(self.level, lines as u8);
        self.view.update();
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

    pub fn inc_frame_counter(self: &mut Self) -> bool {
        self.frame_counter += 1;
        if self.frame_counter >= ScoreCtrl::max_frame_count(self.level) {
            self.frame_counter = 0;
            true
        } else {
            false
        }
    }

    pub fn lock_delay(self: &mut Self) {
        self.frame_counter = ScoreCtrl::max_frame_count(self.level) - 30;
    }

    pub fn new(level: i8) -> Self {
        ScoreCtrl {
            view: UpdatableView::default(),
            level: if level < 29 { level as i8 } else { 29 },
            score: 0,
            lines_cleared: 0,
            frame_counter: 0,
        }
    }
}

impl Ctrl for ScoreCtrl {
    fn show(self: &mut Self, view: &impl View) {
        self.view.show(view, &ShowArgs::ScoreArgs{level: self.level, lines: self.lines_cleared, score: self.score});
    }
}
