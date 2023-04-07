use std::{error::Error, time::Duration};

use log::{debug, error};
use paho_mqtt::{
    Client, ConnectOptions, ConnectOptionsBuilder, Message, SslOptionsBuilder, SslVersion,
};

use crate::utils::config::Config;

pub struct Publisher {
    client: Client,
    connection_options: ConnectOptions,
}

impl Publisher {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config::load().unwrap();
        let create_ops = paho_mqtt::CreateOptionsBuilder::new()
            .server_uri(config.mqtt.url.clone())
            .client_id(
                config
                    .mqtt
                    .client
                    .clone()
                    .unwrap_or("rustybot-server-publisher".into()),
            )
            .mqtt_version(0)
            .finalize();
        let mut conn_opts_builder = &mut ConnectOptionsBuilder::new();
        conn_opts_builder = conn_opts_builder
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .user_name(config.mqtt.username.clone())
            .password(config.mqtt.password.clone());

        if config.mqtt.tls {
            conn_opts_builder = conn_opts_builder.ssl_options(
                SslOptionsBuilder::new()
                    .ssl_version(SslVersion::Tls_1_2)
                    .verify(false)
                    .finalize(),
            );
        }

        if let Ok(client) = paho_mqtt::Client::new(create_ops) {
            Ok(Self {
                client: client,
                connection_options: conn_opts_builder.finalize(),
            })
        } else {
            error!(target: "mqtt", "Unable to create MQTT client");
            Err("Unable to create MQTT client".into())
        }
    }

    pub fn publish(&self, message: Message) -> Result<(), Box<dyn Error>> {
        if let Err(e) = self.client.connect(self.connection_options.clone()) {
            error!(target: "mqtt", "{}", format!("Unable to connect to MQTT: {:?}", e));
            return Err(Box::new(e));
        }
        if let Err(e) = self.client.publish(message.clone()) {
            error!(target: "mqtt", "{}", format!("Unable to send message `{:?}` to MQTT: {:?}", message, e));
            return Err(Box::new(e));
        }
        debug!(target: "mqtt", "Message successfully published to topic `{}`", message.topic());
        Ok(())
    }

    pub fn publish_message(&self, message: String, topic: String) -> Result<(), Box<dyn Error>> {
        let message: Message = Message::new(topic, message, 2);
        return self.publish(message);
    }

    pub fn close(&mut self) -> Result<(), Box<dyn Error>> {
        if self.client.is_connected() {
            self.client.disconnect(None).unwrap();
        }
        Ok(())
    }
}
