use crate::updateable_view::{UpdatableView, Ctrl};
use crate::view::{View, ShowArgs};
use std::cmp;

const MAX_LEVEL: i8 = 29;

pub struct ScoreCtrl {
    view: UpdatableView,
    pub level: i8,
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

    pub fn new(level: i8) -> Self {
        ScoreCtrl {
            view: UpdatableView::default(),
            level: if level < MAX_LEVEL && level >= 0 { level as i8 } else { MAX_LEVEL },
            score: 0,
            lines_cleared: 0,
        }
    }
}

impl Ctrl for ScoreCtrl {
    fn show(self: &mut Self, view: &mut impl View) {
        self.view.show(view, &ShowArgs::ScoreArgs{level: self.level, lines: self.lines_cleared, score: self.score});
    }
}
