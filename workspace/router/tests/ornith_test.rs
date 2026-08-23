use router::lora_router::{DynamicLoraRouter as RouterLoraRouter, LoraAdapterType};
use vllm_client::DynamicLoraRouter as VllmLoraRouter;

#[tokio::test]
async fn test_ornith_lora_routing_and_tuning() {
    let router = RouterLoraRouter::new();

    // 1. Test Domain Prediction for Embedded Rust
    let prompt_rust = "Write an async SPI DMA driver for STM32H7 bare-metal firmware in no_std";
    let adapter_rust = router.select_adapter_for_prompt(prompt_rust);
    assert_eq!(adapter_rust, LoraAdapterType::FirmwareEmbedded);

    // 2. Test Domain Prediction for KiCad PCB Design
    let prompt_kicad = "Generate schematic netlist for buck converter circuit with bypass capacitors";
    let adapter_kicad = router.select_adapter_for_prompt(prompt_kicad);
    assert_eq!(adapter_kicad, LoraAdapterType::PcbCad);

    // 3. Test SGLang Dynamic LoRA Activation Dispatcher
    let lora_client = VllmLoraRouter::new("http://127.0.0.1:30000");
    let activation_res = lora_client.route_for_domain("EmbeddedRust").await;
    assert!(activation_res.is_ok());
    let msg = activation_res.unwrap();
    assert!(msg.contains("lora_embedded_rust_v2"));
}
