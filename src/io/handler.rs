use std::sync::Arc;

use eyre::Result;
use log::{error, info};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

use super::IoEvent;
use crate::app::App;
use crate::crypto::utils::{decrypt_small_file, encrypt_small_file};
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
            IoEvent::Close => self.close_application().await,
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

    async fn close_application(&mut self) -> Result<()> {
        info!("🚪 Close the application");
        let password = b"password";
        let salt = b"salt";
        // number of iterations
        let n = 4096;

        let mut key1 = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password, salt, n, &mut key1);
        let res = encrypt_small_file("./keys.db", "./keys.db.encrypt", &key1);
        if let Err(err) = res {
            error!("Cannot encrypt file: {:?}", err);
        } else {
            info!("🔒 File encrypted");
            std::fs::remove_file("./keys.db")?;
        }
        Ok(())
    }

    async fn do_initialize(&mut self) -> Result<()> {
        info!("🚀 Initialize the application");
        let mut app = self.app.lock().await;

        //TODO: get the configuration
        //

        let password = b"password";
        let salt = b"salt";
        // number of iterations
        let n = 4096;

        let mut key1 = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password, salt, n, &mut key1);

        // let key = String::from("01234567890123456789012345678901").into_bytes();

        let res = decrypt_small_file("./keys.db.encrypt", "./keys.db", &key1);
        if let Err(err) = res {
            error!("Cannot decrypt file: {:?}", err);
        } else {
            info!("🔓 File decrypted");
            std::fs::remove_file("./keys.db.encrypt")?;
            info!("💾 Retrieve data");
            crate::repository::init_database_schemas("keys.db")?;
            app.data.load_key_list();

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            app.initialized(); // we could update the app state
            info!("🍾 Application initialized");
        }
        Ok(())
    }
}
