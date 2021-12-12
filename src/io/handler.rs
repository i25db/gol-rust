use std::sync::Arc;
use std::time::Duration;

use eyre::Result;
use log::{error, info};

use super::IoEvent;
use crate::app::App;

pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Sleep(duration) => self.do_sleep(duration).await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        let mut app = self.app.lock().await;
        app.loaded(); // update app loading state
    }

    async fn do_initialize(&mut self) -> Result<()> {
        info!("🚀 Initialize the application");
        let mut app = self.app.lock().await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        app.initialized(); // we could update the app state
        info!("👍 Application initialized");

        Ok(())
    }

    async fn do_sleep(&mut self, duration: Duration) -> Result<()> {
        info!("😴 Go to sleep for {:?}...", duration);
        tokio::time::sleep(duration).await; // Sleeping
        info!("⏰ Wake up !");
        // Notify the app for having slept
        let mut app = self.app.lock().await;
        app.slept();
        Ok(())
    }
}
