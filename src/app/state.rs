// use std::time::Duration;

use std::fmt::Display;

use tui::widgets::ListState;

use crate::models::key::Key;
use crate::repository::keys::{insert_key_to_db, retrive_keys_from_db};

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        // duration: Duration,
        counter_tick: u64,
        show_help: bool,
        show_creation_popup: bool,
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
            show_creation_popup: false,
        }
    }

    pub fn toggle_help(&mut self) {
        if let Self::Initialized { show_help, .. } = self {
            *show_help = !*show_help;
        }
    }

    pub fn toggle_creation_popup(&mut self) {
        if let Self::Initialized {
            show_creation_popup,
            ..
        } = self
        {
            *show_creation_popup = !*show_creation_popup;
        }
    }

    pub fn is_creation_popup(&self) -> bool {
        if let Self::Initialized {
            show_creation_popup,
            ..
        } = self
        {
            *show_creation_popup
        } else {
            false
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
    fn default() -> AppState {
        Self::Init
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct AppData {
    pub keys: StatefulList<Key>,
}

impl AppData {
    pub fn load_key_list(&mut self) -> () {
        let keys = retrive_keys_from_db().unwrap();
        self.keys = StatefulList::with_items(keys);
    }

    pub fn create_key(&mut self, key: Key) -> () {
        insert_key_to_db(&key).unwrap();
        self.keys.items.push(key);
    }
}

impl Display for AppData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut keys = String::new();
        for key in &self.keys.items {
            keys.push_str(&format!("{}\n", key.name()));
        }
        write!(f, "{}", keys)
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            keys: StatefulList::with_items(Vec::new()),
        }
    }
}

// Container for the data we want to display
// pub enum AppData {
//     NoData,
//     KeyList: StatefulList<Key>,
// }

// impl AppData {
//     pub fn set_key_list(&mut self) {
//         let keys = retrive_keys_from_db().unwrap();
//         *self = Self::KeyList { keys };
//     }
//     // pub fn set_key_list(keys: Vec<Key>) -> Self {
//     //     Self::KeyList { keys }
//     // }

//     pub fn is_key_list(&self) -> bool {
//         matches!(self, &Self::KeyList { .. })
//     }

//     pub fn get_key_list(&self) -> Option<Vec<Key>> {
//         if let Self::KeyList { keys } = self {
//             Some(keys.clone())
//         } else {
//             None
//         }
//     }

//     // pub fn get_key_list(&self) -> Vec<Key> {
//     //     // if let Self::KeyList { keys } = self {
//     //     self
//     //     // } else {
//     //     // None
//     //     // }
//     // }
// }
