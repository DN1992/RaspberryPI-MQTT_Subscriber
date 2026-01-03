use crate::{config::Config, database};
use log::{info, error, debug};
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, Transport};
use sqlx::PgPool;
use std::time::Duration;

pub struct MqttClient {
    config: Config,
    db_pool: PgPool,
}

impl MqttClient {
    pub fn new(config: &Config, db_pool: PgPool) -> Self {
        Self {
            config: config.clone(),
            db_pool,
        }
    }
    
    pub async fn subscribe_and_listen(&mut self) -> Result<(), anyhow::Error> {
        let mut mqttoptions = MqttOptions::new(
            &self.config.mqtt_client_id,
            &self.config.mqtt_broker_url,
            self.config.mqtt_port,
        );
        
        mqttoptions.set_keep_alive(Duration::from_secs(60));
        
        if let (Some(username), Some(password)) = (&self.config.mqtt_username, &self.config.mqtt_password) {
            mqttoptions.set_credentials(username, password);
        }
        
        if self.config.mqtt_use_tls {
            mqttoptions.set_transport(Transport::tls_with_default_config());
        }
        
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 100);
        
        info!("Subscribing to topic: {}", self.config.mqtt_topic);
        client
            .subscribe(&self.config.mqtt_topic, self.config.mqtt_qos)
            .await?;
        
        info!("MQTT client connected and subscribed. Listening for messages...");
        
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Incoming::Publish(publish))) => {
                    debug!("Received message on topic: {}", publish.topic);
                    
                    if let Err(e) = database::insert_message(
                        &self.db_pool,
                        &publish.topic,
                        &publish.payload,
                        publish.qos,
                        publish.retain,
                    ).await {
                        error!("Failed to store message in database: {}", e);
                    } else {
                        debug!("Successfully stored message from topic: {}", publish.topic);
                    }
                }
                Ok(Event::Incoming(incoming)) => {
                    debug!("Incoming event: {:?}", incoming);
                }
                Ok(Event::Outgoing(outgoing)) => {
                    debug!("Outgoing event: {:?}", outgoing);
                }
                Err(e) => {
                    error!("Connection error: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            }
        }
    }
}