#[cfg(test)]
mod tests {
    use edge_swarm::HardwareTelemetryPacket;

    #[test]
    fn test_postcard_packet_roundtrip() {
        let packet = HardwareTelemetryPacket {
            device_id: 0xDEADBEEF,
            timestamp_ms: 123456789,
            cpu_frequency_mhz: 168,
            vdda_millivolts: 3300,
            core_temperature_c: 42.5,
            rtt_log_snippet: "[RTT] STM32F4 Core running OK".to_string(),
        };

        let encoded = packet.to_bytes().expect("encoding must succeed");
        let decoded = HardwareTelemetryPacket::from_bytes(&encoded).expect("decoding must succeed");

        assert_eq!(packet, decoded);
    }
}
