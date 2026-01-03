use rumqttc::QoS;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub mqtt_broker_url: String,
    pub mqtt_port: u16,
    pub mqtt_username: Option<String>,
    pub mqtt_password: Option<String>,
    pub mqtt_topic: String,
    pub mqtt_qos: QoS,
    pub database_url: String,
    pub mqtt_client_id: String,
    pub mqtt_use_tls: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenv().ok();
        
        let mqtt_broker_url = env::var("MQTT_BROKER_URL")
            .unwrap_or_else(|_| "localhost".to_string());
        
        let mqtt_port = env::var("MQTT_PORT")
            .unwrap_or_else(|_| "1883".to_string())
            .parse::<u16>()?;
        
        let mqtt_username = env::var("MQTT_USERNAME").ok();
        let mqtt_password = env::var("MQTT_PASSWORD").ok();
        
        let mqtt_topic = env::var("MQTT_TOPIC")
            .unwrap_or_else(|_| "#".to_string());
        
        let mqtt_qos = match env::var("MQTT_QOS")
            .unwrap_or_else(|_| "1".to_string())
            .as_str() 
        {
            "0" => QoS::AtMostOnce,
            "2" => QoS::ExactlyOnce,
            _ => QoS::AtLeastOnce,
        };
        
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://user:password@localhost:5432/mqtt_data".to_string());
        
        let mqtt_client_id = env::var("MQTT_CLIENT_ID")
            .unwrap_or_else(|_| "rust-mqtt-subscriber".to_string());
        
        let mqtt_use_tls = env::var("MQTT_USE_TLS")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        
        Ok(Config {
            mqtt_broker_url,
            mqtt_port,
            mqtt_username,
            mqtt_password,
            mqtt_topic,
            mqtt_qos,
            database_url,
            mqtt_client_id,
            mqtt_use_tls,
        })
    }
}