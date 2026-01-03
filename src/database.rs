use sqlx::{postgres::PgPoolOptions, PgPool};
use log::info;
use std::fs;

#[derive(Debug, sqlx::FromRow)]
pub struct MqttMessage {
    pub id: i64,
    pub topic: String,
    pub payload: String,
    pub qos: i32,
    pub retain: bool,
    pub timestamp: chrono::NaiveDateTime,
}

pub async fn init_db(database_url: &str) -> Result<PgPool, anyhow::Error> {
    info!("Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    info!("Successfully connected to database");
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), anyhow::Error> {
    info!("Running database migrations...");
    
    let migration_sql = fs::read_to_string("migrations/0001_create_messages_table.sql")
        .map_err(|e| anyhow::anyhow!("Failed to read migration file: {}", e))?;
    
    sqlx::query(&migration_sql)
        .execute(pool)
        .await?;
    
    info!("Database migrations completed successfully");
    Ok(())
}

pub async fn insert_message(
    pool: &PgPool,
    topic: &str,
    payload: &[u8],
    qos: rumqttc::QoS,
    retain: bool,
) -> Result<(), anyhow::Error> {
    let payload_str = String::from_utf8_lossy(payload).to_string();
    let qos_int = match qos {
        rumqttc::QoS::AtMostOnce => 0,
        rumqttc::QoS::AtLeastOnce => 1,
        rumqttc::QoS::ExactlyOnce => 2,
    };
    
    sqlx::query(
        "INSERT INTO mqtt_messages (topic, payload, qos, retain, timestamp) VALUES ($1, $2, $3, $4, NOW())",
    )
    .bind(topic)
    .bind(payload_str)
    .bind(qos_int)
    .bind(retain)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_recent_messages(
    pool: &PgPool,
    limit: i64,
) -> Result<Vec<MqttMessage>, anyhow::Error> {
    let messages = sqlx::query_as::<_, MqttMessage>(
        "SELECT id, topic, payload, qos, retain, timestamp FROM mqtt_messages ORDER BY timestamp DESC LIMIT $1"
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(messages)
}