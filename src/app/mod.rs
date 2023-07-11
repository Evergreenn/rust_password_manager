use arboard::Clipboard;
use log::{debug, error, info};

use self::actions::confirmation_actions::ConfirmationActions;
use self::actions::editing_actions::EditingActions;
use self::actions::normal_actions::Actions;
use self::state::{AppData, AppState};
use crate::app::actions::confirmation_actions::ConfirmationAction;
use crate::app::actions::editing_actions::EditingAction;
use crate::app::actions::normal_actions::Action;
use crate::config::Config;
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

#[derive(Debug, PartialEq, Eq)]
enum InputMode {
    Normal,
    Editing,
    Confirmation,
}

/// The main application, containing the state
pub struct App {
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    actions: Actions,
    editing_actions: EditingActions,
    confirmation_actions: ConfirmationActions,
    is_loading: bool,
    state: AppState,
    input_mode: InputMode,
    input_buffer: String,
    pub data: AppData,
    pub clipboard: Clipboard,
    pub config: Config,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let editing_actions = vec![
            EditingAction::Quit,
            EditingAction::RemoveChar,
            EditingAction::Dismiss,
            EditingAction::Validate,
            EditingAction::WriteChar,
        ]
        .into();
        let confirmation_actions =
            vec![ConfirmationAction::Cancel, ConfirmationAction::Confirm].into();
        let is_loading = false;
        let state = AppState::default();
        let data = AppData::default();
        let input_mode = InputMode::Normal;
        let input_buffer = String::new();
        let clipboard = Clipboard::new().unwrap();
        let config = Config::default();

        Self {
            io_tx,
            actions,
            editing_actions,
            confirmation_actions,
            is_loading,
            state,
            data,
            input_mode,
            input_buffer,
            clipboard,
            config,
        }
    }

    pub fn get_input_buffer(&self) -> &str {
        &self.input_buffer
    }

    pub fn clear_input_buffer(&mut self) {
        self.input_buffer.clear();
    }

    /// Handle a user action
    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        match self.input_mode {
            InputMode::Normal => match self.actions.find(key) {
                Some(action) => self.do_normal_action(*action).await,
                None => {
                    // warn!("No action accociated to {}", key);
                    AppReturn::Continue
                }
            },
            InputMode::Editing => match self.editing_actions.find(key) {
                Some(action) => self.do_editing_action(*action, key).await,
                None => self.do_editing_action(EditingAction::WriteChar, key).await,
            },
            InputMode::Confirmation => match self.confirmation_actions.find(key) {
                Some(action) => self.do_confirmation_action(*action).await,
                None => {
                    // warn!("No action accociated to {}", key);
                    AppReturn::Continue
                }
            },
        }
    }

    async fn do_confirmation_action(&mut self, action: ConfirmationAction) -> AppReturn {
        match action {
            ConfirmationAction::Cancel => {}
            ConfirmationAction::Confirm => {
                if self.state.is_delete_popup() {
                    let key = self.data.keys.state.selected();
                    if let Some(key) = key {
                        if let Some(item) = self.data.keys.items.get(key) {
                            self.dispatch(IoEvent::Delete(item.clone())).await;
                            self.dispatch(IoEvent::Refresh).await;
                        }
                    }
                }
            }
        };

        self.toggle_confirmation_mode();
        AppReturn::Continue
    }

    /// Handle a user action in editing mode
    async fn do_editing_action(&mut self, action: EditingAction, key: Key) -> AppReturn {
        match action {
            EditingAction::Quit => {
                self.dispatch(IoEvent::Close).await;
                AppReturn::Exit
            }
            EditingAction::RemoveChar => {
                self.input_buffer.pop();
                AppReturn::Continue
            }
            EditingAction::Dismiss => {
                self.toggle_input_mode();
                self.state.toggle_creation_popup();
                self.input_buffer.clear();
                AppReturn::Continue
            }

            EditingAction::Validate => {
                if self.state.is_initialized() {
                    let key = crate::models::key::Key::new(
                        None,
                        self.input_buffer.clone(),
                        &self.config.password_options,
                    );
                    self.dispatch(IoEvent::RegisterKey(key)).await;
                    self.dispatch(IoEvent::Refresh).await;
                    self.toggle_input_mode();
                    self.state.toggle_creation_popup();
                    self.input_buffer.clear();
                } else {
                    debug!("Initialization");
                    self.dispatch(IoEvent::Initialize).await;
                    // self.state.set_initialization(self.input_buffer.clone());
                }

                AppReturn::Continue
            }
            EditingAction::WriteChar => {
                self.input_buffer.push(key.to_char());
                AppReturn::Continue
            } // _ => {
              //     warn!("No action accociated to {}", key);
              //     AppReturn::Continue
              // }
        }
    }

    /// Handle a user action in normal mode
    async fn do_normal_action(&mut self, action: Action) -> AppReturn {
        match action {
            Action::Quit => {
                self.dispatch(IoEvent::Close).await;
                AppReturn::Exit
            }
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
                AppReturn::Continue
            }
            Action::DeleteKey => {
                let key = self.data.keys.state.selected();
                if let Some(key) = key {
                    if let Some(_) = self.data.keys.items.get(key) {
                        self.toggle_confirmation_mode();
                        self.state.toggle_deletion_popup();
                    }
                }
                AppReturn::Continue
            }
            Action::CopyPassword => {
                let key = self.data.keys.state.selected();
                if let Some(key) = key {
                    if let Some(item) = self.data.keys.items.get_mut(key) {
                        item.update_last_used_at();
                        let updated = item.update_in_database();
                        if let Err(err) = updated {
                            error!("Cannot update key: {:?}", err);
                        } else {
                            info!("🔑 Key updated");
                        }
                        let item = item.clone();
                        self.dispatch(IoEvent::Copy(item)).await;
                    }
                }
                AppReturn::Continue
            }
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
            Action::CopyPassword,
            Action::DeleteKey,
        ]
        .into();
        self.editing_actions = vec![
            EditingAction::Quit,
            EditingAction::Validate,
            EditingAction::RemoveChar,
            EditingAction::WriteChar,
            EditingAction::Dismiss,
        ]
        .into();
        self.state = AppState::initialized()
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }

    //TODO
    pub fn toggle_input_mode(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Normal => InputMode::Editing,
            InputMode::Editing => InputMode::Normal,
            _ => InputMode::Normal,
        }
    }

    pub fn toggle_confirmation_mode(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Normal => InputMode::Confirmation,
            InputMode::Editing => InputMode::Confirmation,
            InputMode::Confirmation => {
                self.state.toggle_confirmation_popup();
                self.state.toggle_deletion_popup();
                InputMode::Normal
            }
        }
    }
}
