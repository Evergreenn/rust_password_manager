use std::sync::Arc;

use arboard::Clipboard;
use eyre::Result;
use log::{error, info};

use super::IoEvent;
use crate::app::App;
use crate::models::key::Key;

/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Copy(key) => self.do_copy(key).await,
            IoEvent::RegisterKey(key) => self.register_key(key).await,
            IoEvent::Refresh => self.refresh_application_state().await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        let mut app = self.app.lock().await;
        app.loaded();
    }

    async fn refresh_application_state(&mut self) -> Result<()> {
        info!("🔄 Refresh application state");
        let mut app = self.app.lock().await;
        app.data.load_key_list();
        Ok(())
    }

    async fn register_key(&mut self, key: Key) -> Result<()> {
        let save = key.persist();
        if let Err(err) = save {
            error!("Cannot save key: {:?}", err);
        } else {
            info!("🔑 Key saved");
        }
        Ok(())
    }

    async fn do_copy(&mut self, key: Key) -> Result<()> {
        let mut app = self.app.lock().await;
        let clipped = app.clipboard.set_text(key.password());
        if let Err(err) = clipped {
            error!("Cannot copy to clipboard: {:?}", err);
        } else {
            info!("📝 Copy password to clipboard");
        }

        Ok(())
    }

    async fn do_initialize(&mut self) -> Result<()> {
        info!("🚀 Initialize the application");
        let mut app = self.app.lock().await;

        //TODO: get the configuration

        info!("💾 Retrieve data");
        crate::repository::init_database_schemas()?;
        app.data.load_key_list();

        app.initialized(); // we could update the app state
        info!("🍾 Application initialized");

        Ok(())
    }
}
