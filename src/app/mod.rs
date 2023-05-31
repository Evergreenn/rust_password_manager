use log::{debug, error, warn};

use self::actions::Actions;
use self::state::{AppData, AppState};
use crate::app::actions::Action;
use crate::inputs::key::Key;
use crate::io::IoEvent;

pub mod actions;
pub mod state;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

enum InputMode {
    Normal,
    Editing,
}

/// The main application, containing the state
pub struct App {
    /// We could dispatch an IO event
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    /// Contextual actions
    actions: Actions,
    /// State
    is_loading: bool,
    state: AppState,
    input_mode: InputMode,
    input_buffer: String,
    pub data: AppData,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::default();
        let data = AppData::default();
        let input_mode = InputMode::Normal;
        let input_buffer = String::new();

        Self {
            io_tx,
            actions,
            is_loading,
            state,
            data,
            input_mode,
            input_buffer,
        }
    }

    /// Handle a user action
    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);
            match self.input_mode {
                InputMode::Normal => self.do_normal_action(*action).await,
                InputMode::Editing => self.do_editing_action(*action).await,
            }
        } else {
            warn!("No action accociated to {}", key);
            AppReturn::Continue
        }
    }

    /// Handle a user action in editing mode
    async fn do_editing_action(&mut self, action: Action) -> AppReturn {
        match action {
            Action::RemoveChar => {
                self.input_buffer.pop();
                AppReturn::Continue
            }
            // Action::WriteChar(c) => {
            //     self.input_buffer.push(c);
            //     AppReturn::Continue
            // }
            Action::Validate => {
                self.toggle_input_mode();
                self.state.toggle_creation_popup();
                // self.data.create_key(/* key */);
                AppReturn::Continue
            }
            _ => AppReturn::Continue,
        }
    }

    /// Handle a user action in normal mode
    async fn do_normal_action(&mut self, action: Action) -> AppReturn {
        match action {
            Action::Quit => AppReturn::Exit,
            Action::Help => {
                self.state.toggle_help();
                AppReturn::Continue
            }
            Action::MoveUp => {
                self.data.keys.previous();
                AppReturn::Continue
            }
            Action::MoveDown => {
                self.data.keys.next();
                AppReturn::Continue
            }
            Action::CreateKey => {
                self.toggle_input_mode();
                self.state.toggle_creation_popup();
                // self.data.create_key(/* key */);
                AppReturn::Continue
            }
            _ => AppReturn::Continue,
            // Action::Sleep => {

            //     if let Some(duration) = self.state.duration().cloned() {
            //         // Sleep is an I/O action, we dispatch on the IO channel that's run on another thread
            //         self.dispatch(IoEvent::Sleep(duration)).await
            //     }
            //     AppReturn::Continue
            // }
            // IncrementDelay and DecrementDelay is handled in the UI thread
            // Action::IncrementDelay => {
            //     self.state.increment_delay();
            //     AppReturn::Continue
            // }
            // // Note, that we clamp the duration, so we stay >= 0
            // Action::DecrementDelay => {
            //     self.state.decrement_delay();
            // }
        }
    }

    /// We could update the app or dispatch event on tick
    pub async fn update_on_tick(&mut self) -> AppReturn {
        // here we just increment a counter
        self.state.incr_tick();
        AppReturn::Continue
    }

    /// Send a network event to the IO thread
    pub async fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in io/handler.rs
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            self.is_loading = false;
            error!("Error from dispatch {}", e);
        };
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }
    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn initialized(&mut self) {
        // Update contextual actions
        self.actions = vec![
            Action::Quit,
            Action::Help,
            Action::MoveUp,
            Action::MoveDown,
            Action::CreateKey,
            // Action::Sleep,
            // Action::IncrementDelay,
            // Action::DecrementDelay,
        ]
        .into();
        self.state = AppState::initialized()
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }

    pub fn toggle_input_mode(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Normal => InputMode::Editing,
            InputMode::Editing => InputMode::Normal,
        }
    }
}
