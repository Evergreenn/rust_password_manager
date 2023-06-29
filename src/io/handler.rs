use std::path::Path;
use std::sync::Arc;

use eyre::Result;
use log::{error, info};

use super::IoEvent;
use crate::app::App;
use crate::config;
use crate::crypto::utils::{decrypt_small_file, encrypt_small_file, gen_key_from_password};
use crate::models::key::Key;

/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
    password: String,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self {
            app,
            password: String::new(),
        }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Copy(key) => self.do_copy(key).await,
            IoEvent::RegisterKey(key) => self.register_key(key).await,
            IoEvent::Refresh => self.refresh_application_state().await,
            IoEvent::Close => self.close_application().await,
            IoEvent::Delete(key) => self.delete_key(key).await,
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

    async fn delete_key(&mut self, key: Key) -> Result<()> {
        let mut app = self.app.lock().await;
        let deleted = key.remove_from_database();
        if let Err(err) = deleted {
            error!("Cannot delete key: {:?}", err);
        } else {
            info!("🗑 Key deleted");
        }
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

    async fn close_application(&mut self) -> Result<()> {
        info!("🚪 Close the application");

        let key1 = gen_key_from_password(self.password.clone());

        let res = encrypt_small_file("./keys.db", "./keys.db.encrypt", &key1);
        if let Err(err) = res {
            error!("Cannot encrypt file: {:?}", err);
        } else {
            info!("🔒 File encrypted");
            std::fs::remove_file("./keys.db")?;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        Ok(())
    }

    async fn do_initialize(&mut self) -> Result<()> {
        info!("🚀 Initialize the application");
        let mut app = self.app.lock().await;

        app.toggle_input_mode();

        // self.password = app.get_input_buffer().clone().to_string();
        self.password = <String>::clone(&app.get_input_buffer().to_string());

        //TODO: get the configuration

        let loaded = config::Config::load_config(&app.config);

        info!("⚙️ Config loaded : {:#?}", loaded.color_scheme);

        if std::path::Path::exists(Path::new("./keys.db.encrypt")) {
            info!("🔒 File encrypted");

            let key1 = gen_key_from_password(self.password.clone());

            let res = decrypt_small_file("./keys.db.encrypt", "./keys.db", &key1);
            if let Err(err) = res {
                error!("Cannot decrypt file: {:?}", err);

                app.toggle_input_mode();
            } else {
                info!("🔓 File decrypted");
                std::fs::remove_file("./keys.db.encrypt")?;
                info!("💾 Retrieve data");
                app.data.load_key_list();

                app.initialized(); // we could update the app state
                info!("🍾 Application initialized");
            }
        } else {
            info!("🔒 File not encrypted");
            crate::repository::init_database_schemas("keys.db")?;
            app.initialized(); // we could update the app state
            info!("🍾 Application initialized");
        }

        app.clear_input_buffer();
        Ok(())
    }
}
