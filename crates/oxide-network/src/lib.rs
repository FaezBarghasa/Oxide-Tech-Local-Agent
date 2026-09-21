use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;

pub struct LanDiscovery {
    mdns: ServiceDaemon,
}

impl LanDiscovery {
    pub fn new() -> Result<Self, mdns_sd::Error> {
        let mdns = ServiceDaemon::new()?;
        Ok(Self { mdns })
    }

    pub fn broadcast_service(&self, port: u16) -> Result<(), mdns_sd::Error> {
        let service_type = "_oxide-agent._tcp.local.";
        let instance_name = "oxide_tech_os";
        let host_name = "oxide.local.";
        let ip = "127.0.0.1";
        let properties: HashMap<String, String> =
            [("version".to_string(), "0.5.0".to_string())].into();

        let service_info =
            ServiceInfo::new(service_type, instance_name, host_name, ip, port, properties)?;

        self.mdns.register(service_info)?;
        tracing::info!(
            "mDNS broadcast active for _oxide-agent._tcp.local on port {}",
            port
        );
        Ok(())
    }
}
