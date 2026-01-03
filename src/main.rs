mod config;
mod database;
mod mqtt;

use anyhow::Result;
use log::{info, error};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting MQTT Subscriber Service");
    
    let config = config::Config::from_env()?;
    info!("Configuration loaded successfully");
    
    let db_pool = database::init_db(&config.database_url).await?;
    info!("Database connection established");
    
    database::run_migrations(&db_pool).await?;
    info!("Database migrations completed");
    
    let mut mqtt_client = mqtt::MqttClient::new(&config, db_pool.clone());
    
    let mqtt_handle = tokio::spawn(async move {
        if let Err(e) = mqtt_client.subscribe_and_listen().await {
            error!("MQTT client error: {}", e);
        }
    });
    
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
        _ = mqtt_handle => {
            info!("MQTT task completed");
        }
    }
    
    info!("Shutting down MQTT Subscriber Service");
    Ok(())
}