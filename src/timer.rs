use std::time::{Duration, Instant};
use crate::config::DEFAULT_TIMER_SECONDS;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
    Initial,
    Running,
    Paused,
    Finished,
}

pub struct Timer {
    pub state: TimerState,
    pub remaining_seconds: u32,
    last_tick: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            state: TimerState::Initial,
            remaining_seconds: DEFAULT_TIMER_SECONDS,
            last_tick: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        if self.state == TimerState::Initial || self.state == TimerState::Paused {
            self.state = TimerState::Running;
            self.last_tick = Instant::now();
        }
    }

    pub fn pause(&mut self) {
        if self.state == TimerState::Running {
            self.state = TimerState::Paused;
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            TimerState::Running => self.pause(),
            TimerState::Paused | TimerState::Initial => self.start(),
            TimerState::Finished => {}
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Initial;
        self.remaining_seconds = DEFAULT_TIMER_SECONDS;
        self.last_tick = Instant::now();
    }

    pub fn update(&mut self) {
        if self.state == TimerState::Running {
            let now = Instant::now();
            if now.duration_since(self.last_tick) >= Duration::from_secs(1) {
                if self.remaining_seconds > 0 {
                    self.remaining_seconds -= 1;
                    if self.remaining_seconds == 0 {
                        self.state = TimerState::Finished;
                        // Bell sound
                        print!("\x07");
                    }
                }
                self.last_tick = now;
            }
        }
    }

    pub fn format_large_time(&self) -> Vec<String> {
        let minutes = self.remaining_seconds / 60;
        let seconds = self.remaining_seconds % 60;
        let time_str = format!("{:02}:{:02}", minutes, seconds);

        let mut lines = vec![];
        for row in 0..7 {
            let mut line = String::new();
            for ch in time_str.chars() {
                line.push_str(&self.get_large_digit(ch, row));
                line.push(' ');
            }
            lines.push(line);
        }
        lines
    }

    fn get_large_digit(&self, digit: char, row: usize) -> &'static str {
        match digit {
            '0' => match row {
                0 => "████",
                1 => "█  █",
                2 => "█  █",
                3 => "█  █",
                4 => "█  █",
                5 => "█  █",
                6 => "████",
                _ => "    ",
            },
            '1' => match row {
                0 => "  █ ",
                1 => " ██ ",
                2 => "  █ ",
                3 => "  █ ",
                4 => "  █ ",
                5 => "  █ ",
                6 => "████",
                _ => "    ",
            },
            '2' => match row {
                0 => "████",
                1 => "   █",
                2 => "   █",
                3 => "████",
                4 => "█   ",
                5 => "█   ",
                6 => "████",
                _ => "    ",
            },
            '3' => match row {
                0 => "████",
                1 => "   █",
                2 => "   █",
                3 => "████",
                4 => "   █",
                5 => "   █",
                6 => "████",
                _ => "    ",
            },
            '4' => match row {
                0 => "█  █",
                1 => "█  █",
                2 => "█  █",
                3 => "████",
                4 => "   █",
                5 => "   █",
                6 => "   █",
                _ => "    ",
            },
            '5' => match row {
                0 => "████",
                1 => "█   ",
                2 => "█   ",
                3 => "████",
                4 => "   █",
                5 => "   █",
                6 => "████",
                _ => "    ",
            },
            '6' => match row {
                0 => "████",
                1 => "█   ",
                2 => "█   ",
                3 => "████",
                4 => "█  █",
                5 => "█  █",
                6 => "████",
                _ => "    ",
            },
            '7' => match row {
                0 => "████",
                1 => "   █",
                2 => "   █",
                3 => "  █ ",
                4 => "  █ ",
                5 => " █  ",
                6 => " █  ",
                _ => "    ",
            },
            '8' => match row {
                0 => "████",
                1 => "█  █",
                2 => "█  █",
                3 => "████",
                4 => "█  █",
                5 => "█  █",
                6 => "████",
                _ => "    ",
            },
            '9' => match row {
                0 => "████",
                1 => "█  █",
                2 => "█  █",
                3 => "████",
                4 => "   █",
                5 => "   █",
                6 => "████",
                _ => "    ",
            },
            ':' => match row {
                0 => "  ",
                1 => "  ",
                2 => "██",
                3 => "  ",
                4 => "██",
                5 => "  ",
                6 => "  ",
                _ => "  ",
            },
            _ => "    ",
        }
    }

    pub fn format_large_ramen(&self) -> Vec<String> {
        vec![
            "██████   █████  ███    ███ ███████ ███    ██ ██".to_string(),
            "██   ██ ██   ██ ████  ████ ██      ████   ██ ██".to_string(),
            "██████  ███████ ██ ████ ██ █████   ██ ██  ██ ██".to_string(),
            "██   ██ ██   ██ ██  ██  ██ ██      ██  ██ ██   ".to_string(),
            "██   ██ ██   ██ ██      ██ ███████ ██   ████ ██".to_string(),
            "                                              ".to_string(),
            "        🍜 🍜 🍜 🍜 🍜 🍜 🍜 🍜 🍜          ".to_string(),
        ]
    }
}
