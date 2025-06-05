use std::time::{Duration, Instant};

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
    initial_seconds: u32,
    last_tick: Instant,
}

impl Timer {
    pub fn with_duration(seconds: u32) -> Timer {
        Timer {
            state: TimerState::Initial,
            remaining_seconds: seconds,
            initial_seconds: seconds,
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
        self.remaining_seconds = self.initial_seconds;
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
                    }
                }
                self.last_tick = now;
            }
        }
    }

}
