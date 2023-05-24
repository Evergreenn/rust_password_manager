// use std::time::Duration;

use crate::models::key::Key;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        // duration: Duration,
        counter_tick: u64,
        show_help: bool,
    },
}

impl AppState {
    pub fn initialized() -> Self {
        // let duration = Duration::from_secs(1);
        let counter_tick = 0;
        Self::Initialized {
            // duration,
            counter_tick,
            show_help: false,
        }
    }

    pub fn toggle_help(&mut self) {
        if let Self::Initialized { show_help, .. } = self {
            *show_help = !*show_help;
        }
    }

    pub fn is_help(&self) -> bool {
        if let Self::Initialized { show_help, .. } = self {
            *show_help
        } else {
            false
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}

pub enum AppData {
    NoData,
    KeyList { keys: Vec<Key> },
}

impl AppData {}

impl Default for AppData {
    fn default() -> Self {
        Self::NoData
    }
}
