use rumqttc::{Client, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

pub struct MqttClient {
    client: Client,
}

impl MqttClient {
    pub fn connect() -> Self {
        let mut mqtt_options = MqttOptions::new("apator_wmbus_mqtt", "localhost", 1883);
        mqtt_options.set_keep_alive(Duration::from_secs(30));

        let (client, mut connection) = Client::new(mqtt_options, 10);

        let mut mqtt_client = Self { client };

        thread::spawn(move || for (_i, _notification) in connection.iter().enumerate() {});

        mqtt_client
    }

    pub fn publish(&self, topic: &str, message: &str, retain: bool) {
        let result = self.client.try_publish(topic, QoS::AtLeastOnce, retain, message);

        if let Err(e) = result {
            eprintln!("mqtt error: {:}", e);
        }
    }
}
