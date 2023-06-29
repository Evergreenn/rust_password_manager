// use std::time::Duration;

use crate::models::key::Key;

pub mod handler;
// For this dummy application we only need two IO event
#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize, // Launch to initialize the application
    // Sleep(Duration), // Just take a little break
    Copy(Key), // Copy the key
    RegisterKey(Key),
    Refresh,
    Delete(Key),
    Close,
}
