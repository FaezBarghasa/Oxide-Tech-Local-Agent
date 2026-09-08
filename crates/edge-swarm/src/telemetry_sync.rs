use crate::packet::HardwareTelemetryPacket;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;

pub struct EdgeSwarmBridge {
    client_id: String,
    broker_host: String,
    broker_port: u16,
}

impl EdgeSwarmBridge {
    pub fn new(
        client_id: impl Into<String>,
        broker_host: impl Into<String>,
        broker_port: u16,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            broker_host: broker_host.into(),
            broker_port,
        }
    }

    /// Dispatch telemetry packet over MQTT v5 broker to synchronize digital twin.
    pub async fn publish_telemetry(
        &self,
        topic: &str,
        packet: &HardwareTelemetryPacket,
    ) -> Result<(), String> {
        let payload = packet
            .to_bytes()
            .map_err(|e| format!("Postcard encoding failed: {e:?}"))?;
        let mut mqttoptions =
            MqttOptions::new(&self.client_id, &self.broker_host, self.broker_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        tokio::spawn(async move {
            while let Ok(_event) = eventloop.poll().await {
                // Event loop pump
            }
        });

        client
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .await
            .map_err(|e| format!("MQTT publish error: {e:?}"))?;

        Ok(())
    }
}
