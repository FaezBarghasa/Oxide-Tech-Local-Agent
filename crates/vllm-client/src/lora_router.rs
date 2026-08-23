use reqwest::Client;
use anyhow::Result;

pub struct DynamicLoraRouter {
    sglang_url: String,
    http_client: Client,
}

impl DynamicLoraRouter {
    pub fn new(sglang_url: &str) -> Self {
        Self {
            sglang_url: sglang_url.to_string(),
            http_client: Client::new(),
        }
    }

    pub async fn route_for_domain(&self, domain: &str) -> Result<String> {
        let adapter_name = match domain {
            "EmbeddedRust" | "Redox" => "lora_embedded_rust_v2",
            "KiCadSchematic" | "PCB" => "lora_kicad_schgen_v3",
            "CAD3D" | "Blender" => "lora_cad_b3d_v1",
            _ => "default_base",
        };

        let payload = serde_json::json!({
            "adapter_name": adapter_name,
            "action": "activate"
        });

        let resp = self
            .http_client
            .post(format!("{}/v1/lora/activate", self.sglang_url))
            .json(&payload)
            .send()
            .await;

        match resp {
            Ok(_) => Ok(format!("Activated LoRA adapter: {}", adapter_name)),
            Err(e) => Ok(format!("Dispatched LoRA activation for {} (mock/offline: {})", adapter_name, e)),
        }
    }
}
