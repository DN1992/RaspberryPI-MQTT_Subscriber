use std::env;
use rumqttc::{MqttOptions, Client, QoS, Event, Incoming};
use rusqlite::{params, Connection};
use chrono::Utc;

fn main() {
    // -------- Read environment variables --------
    let broker = env::var("MQTT_BROKER").unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = env::var("MQTT_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()
        .expect("Invalid MQTT_PORT");

    let topic = env::var("MQTT_TOPIC").unwrap_or_else(|_| "test/topic".to_string());
    let username = env::var("MQTT_USERNAME").unwrap_or_default();
    let password = env::var("MQTT_PASSWORD").unwrap_or_default();

    println!("MQTT broker: {}:{}", broker, port);
    println!("Subscribing to topic: {}", topic);

    // -------- SQLite setup --------
    let conn = Connection::open("data/mqtt_data.db")
        .expect("Failed to open SQLite database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            topic TEXT NOT NULL,
            payload TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create table");

    // -------- MQTT setup --------
    let mut mqttoptions = MqttOptions::new("rust_mqtt_subscriber", broker, port);

    if !username.is_empty() {
        mqttoptions.set_credentials(username, password);
    }

    mqttoptions.set_keep_alive(10);

    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe(&topic, QoS::AtMostOnce)
        .expect("Failed to subscribe");

    println!("Subscriber started. Waiting for messages…");

    // -------- Main loop --------
    for event in connection.iter() {
        if let Ok(Event::Incoming(Incoming::Publish(p))) = event {
            let payload = String::from_utf8_lossy(&p.payload);
            let timestamp = Utc::now().to_rfc3339();

            println!(
                "[{}] {} → {}",
                timestamp,
                p.topic,
                payload
            );

            conn.execute(
                "INSERT INTO messages (timestamp, topic, payload)
                 VALUES (?1, ?2, ?3)",
                params![timestamp, p.topic, payload],
            ).expect("Failed to insert message");
        }
    }
}
