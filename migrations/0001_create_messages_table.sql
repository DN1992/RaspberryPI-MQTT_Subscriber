CREATE TABLE IF NOT EXISTS mqtt_messages (
    id BIGSERIAL PRIMARY KEY,
    topic VARCHAR(1024) NOT NULL,
    payload TEXT NOT NULL,
    qos INTEGER NOT NULL,
    retain BOOLEAN NOT NULL DEFAULT false,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_mqtt_messages_topic ON mqtt_messages(topic);
CREATE INDEX IF NOT EXISTS idx_mqtt_messages_timestamp ON mqtt_messages(timestamp);
CREATE INDEX IF NOT EXISTS idx_mqtt_messages_topic_timestamp ON mqtt_messages(topic, timestamp DESC);