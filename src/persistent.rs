use std::sync::LazyLock;
use std::time::Duration;
use colored::Colorize;
use mongodb::Client;
use tokio::sync::{Mutex, RwLock, RwLockReadGuard};
use tokio::sync::broadcast::Sender;
use tokio::task::JoinHandle;
use crate::error::connection::PersistentMongoConnectionError;
use standard_error::traits::{StandardErrorCodeTrait, StandardErrorDescriptionTrait};


pub struct PersistentMongo {
    address: RwLock<Option<String>>,
    client: RwLock<Option<Client>>,
    monitor_handle: Mutex<Option<JoinHandle<()>>>,

    shutdown_signal_channel: LazyLock<Sender<()>>,
}

impl PersistentMongo {
    pub const fn default() -> Self {
        PersistentMongo {
            address: RwLock::const_new(None),
            client: RwLock::const_new(None),
            monitor_handle: Mutex::const_new(None),

            shutdown_signal_channel: LazyLock::new(|| Sender::new(1)),
        }
    }

    /// Initiates the background monitoring task.
    ///
    /// This method spawns:
    /// - A monitoring thread to check the health of the connection pool.
    pub async fn initiate(&'static self) {
        *self.monitor_handle.lock().await = Some(self.monitor_thread());
    }

    /// Asynchronously creates a MongoDB client from the provided connection address.
    pub async fn create_client(address: impl AsRef<str>) -> Result<Client, PersistentMongoConnectionError> {
        Client::with_uri_str(&address)
            .await
            .map_err(|_| PersistentMongoConnectionError::FailedToEstablishConnection)
    }

    pub async fn client(&self) -> RwLockReadGuard<Client> {
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        loop {
            interval.tick().await;
            let client = self.client.read().await;
            if client.is_some() {
                return RwLockReadGuard::map(client, |f| f.as_ref().unwrap());
            }
        }
    }

    /// Sets the connection address and attempts to establish a MongoDB connection.
    ///
    /// On success, updates the internal state with the provided address and client.
    /// If the connection fails, logs an error message.
    pub async fn set_address(&self, address: impl AsRef<str> + ToString) {
        if let Some(old_address) = self.address.read().await.as_ref() {
            if old_address == address.as_ref() {
                println!(
                    "{}",
                    "MONGO - SET_ADDRESS - Address unchanged and client is still active. \
                    Skipping client re-establishment."
                        .green()
                        .dimmed()
                );
                return;
            }
        }
        println!(
            "{}",
            "MONGO - SET_ADDRESS - Establishing client..."
                .to_string()
                .blue()
        );
        match Self::create_client(&address).await {
            Ok(client) => {
                println!(
                    "{}",
                    "MONGO - SET_ADDRESS - Client established successfully!".green()
                );
                *self.address.write().await = Some(address.to_string());
                *self.client.write().await = Some(client);
            }
            Err(e) => {
                println!(
                    "{}",
                    format!(
                        "MONGO - SET_ADDRESS - {} - {}",
                        e.code(),
                        e.description().unwrap_or_default()
                    )
                        .red()
                );
            }
        }
    }

    /// Spawns an asynchronous background task that monitors the MongoDB connection.
    ///
    /// This task periodically checks the connection health. If the connection is lost,
    /// it will attempt to re-establish it using the stored address.
    pub fn monitor_thread(&'static self) -> JoinHandle<()> {
        let mut shutdown = self.shutdown_signal_channel.subscribe();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(250));
            loop {
                tokio::select! {
                    _ = shutdown.recv() => {
                        break;
                    },
                    _ = interval.tick() => {
                        let mut client_lost = false;
                        {
                            // client.list_databases().await;
                            let client = self.client.read().await;
                            if let Some(client) = client.as_ref() {
                                if client.list_database_names().await.is_err() {
                                    client_lost = true;
                                }
                            } else {
                                client_lost = true;
                            }
                        }
                        if client_lost {
                            if let Some(address) = self.address.read().await.as_ref() {
                                if let Ok(client) = Self::create_client(&address).await {
                                    *self.client.write().await = Some(client);
                                }
                            }
                        }
                    }
                }
            }
        })
    }

    pub async fn shutdown(&self) {
        print!("\n");

        let _ = self.shutdown_signal_channel.send(());
        println!(
            "{}",
            "MONGO - SHUTDOWN - Shutdown signal was propagated to internal threads!".cyan()
        );
        if let Err(_) = tokio::time::timeout(Duration::from_secs(5), async {
            if let Some(monitor_handle) = self.monitor_handle.lock().await.as_mut() {
                let _ = monitor_handle.await;
                println!("{}", "MONGO - SHUTDOWN - Monitor thread killed!".cyan());
            }
        })
            .await
        {
            self.monitor_handle.lock().await.as_mut().map(|f| f.abort());
            println!(
                "{}",
                "MONGO - SHUTDOWN - Some tasks failed to terminate on time!".yellow()
            );
        }

        if let Some(client) = self.client.write().await.take() {
            client.shutdown().await;
            println!(
                "{}",
                "MONGO - SHUTDOWN - Mongo client closed gracefully!".cyan()
            );
        }
        println!("{}", "MONGO - SHUTDOWN - Goodbye!".cyan())
    }
}