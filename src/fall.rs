pub struct Fall {
    pub frame_counter: i8,
    lock_delay_triggered: bool,
}

pub const FRAME_RATE: u32 = 60;

impl Fall {
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

    pub fn inc_frame_counter(self: &mut Self, level: i8) -> bool {
        if self.lock_delay_triggered {
            self.lock_delay_triggered = false;
            self.frame_counter = Fall::max_frame_count(level) - (FRAME_RATE / 2) as i8;
            return false;
        }

        self.frame_counter += 1;
        if self.frame_counter >= Fall::max_frame_count(level) {
            self.frame_counter = 0;
            true
        } else {
            false
        }
    }

    pub fn reset(self: &mut Self) {
        self.lock_delay_triggered = false;
        self.frame_counter = 0;
    }

    pub fn lock_delay(self: &mut Self) {
        self.lock_delay_triggered = true;
    }

    pub fn new() -> Self {
        Fall{frame_counter: 0, lock_delay_triggered: false}
    }
}
