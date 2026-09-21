use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ARM Cortex-M Interrupt Vector Table (IVT) parsed from flash origin (e.g., 0x08000000)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmVectorTable {
    pub initial_sp: u32,
    pub reset_handler: u32,
    pub nmi_handler: u32,
    pub hardfault_handler: u32,
    pub memmanage_handler: u32,
    pub busfault_handler: u32,
    pub usagefault_handler: u32,
    pub svcall_handler: u32,
    pub pendsv_handler: u32,
    pub systick_handler: u32,
    pub external_irqs: Vec<(u32, u32)>, // (IRQ_number, Handler_address)
}

impl ArmVectorTable {
    pub fn parse(bytes: &[u8], _base_address: u32) -> Option<Self> {
        if bytes.len() < 64 {
            return None;
        }

        let read_u32 = |offset: usize| -> u32 {
            if offset + 4 <= bytes.len() {
                u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
            } else {
                0
            }
        };

        let initial_sp = read_u32(0x00);
        let reset_handler = read_u32(0x04);
        let nmi_handler = read_u32(0x08);
        let hardfault_handler = read_u32(0x0C);
        let memmanage_handler = read_u32(0x10);
        let busfault_handler = read_u32(0x14);
        let usagefault_handler = read_u32(0x18);
        let svcall_handler = read_u32(0x2C);
        let pendsv_handler = read_u32(0x38);
        let systick_handler = read_u32(0x3C);

        // Sanity check: Cortex-M handlers in Thumb mode have bit 0 set (odd address)
        if (reset_handler & 1) == 0 && reset_handler != 0 {
            // Not a standard Cortex-M vector table
        }

        let mut external_irqs = Vec::new();
        let irq_count = (bytes.len().saturating_sub(0x40)) / 4;
        for i in 0..irq_count.min(128) {
            let addr = read_u32(0x40 + i * 4);
            if addr != 0 {
                external_irqs.push((i as u32, addr));
            }
        }

        Some(Self {
            initial_sp,
            reset_handler,
            nmi_handler,
            hardfault_handler,
            memmanage_handler,
            busfault_handler,
            usagefault_handler,
            svcall_handler,
            pendsv_handler,
            systick_handler,
            external_irqs,
        })
    }
}

/// Shannon Entropy Scanner for detecting compressed/encrypted firmware payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyChunk {
    pub offset: usize,
    pub size: usize,
    pub entropy: f64, // 0.0 (uniform) to 8.0 (completely random/encrypted)
    pub classification: String,
}

pub struct EntropyScanner;

impl EntropyScanner {
    pub fn scan(bytes: &[u8], chunk_size: usize) -> Vec<EntropyChunk> {
        let mut chunks = Vec::new();
        if bytes.is_empty() || chunk_size == 0 {
            return chunks;
        }

        for (i, chunk) in bytes.chunks(chunk_size).enumerate() {
            let entropy = Self::calculate_shannon_entropy(chunk);
            let classification = if entropy > 7.2 {
                "Encrypted / Compressed Payload".to_string()
            } else if entropy > 5.0 {
                "Compiled Code / Text".to_string()
            } else if entropy > 1.0 {
                "Structured Data / Tables".to_string()
            } else {
                "Padding / Zeroes".to_string()
            };

            chunks.push(EntropyChunk {
                offset: i * chunk_size,
                size: chunk.len(),
                entropy,
                classification,
            });
        }

        chunks
    }

    pub fn calculate_shannon_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut counts = [0usize; 256];
        for &b in data {
            counts[b as usize] += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &counts {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }

        entropy
    }
}

/// SVD Peripheral Memory-Mapped Register Mapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvdPeripheralMap {
    pub peripherals: HashMap<u32, (String, u32)>, // BaseAddr -> (Name, Size)
}

impl Default for SvdPeripheralMap {
    fn default() -> Self {
        let mut m = Self {
            peripherals: HashMap::new(),
        };
        // Common STM32F4 / Cortex-M memory map defaults
        m.register_peripheral(0x4000_0000, "TIM2", 0x400);
        m.register_peripheral(0x4000_0400, "TIM3", 0x400);
        m.register_peripheral(0x4000_0800, "TIM4", 0x400);
        m.register_peripheral(0x4000_4400, "USART2", 0x400);
        m.register_peripheral(0x4000_4800, "USART3", 0x400);
        m.register_peripheral(0x4000_5400, "I2C1", 0x400);
        m.register_peripheral(0x4000_5800, "I2C2", 0x400);
        m.register_peripheral(0x4001_1000, "USART1", 0x400);
        m.register_peripheral(0x4001_3000, "SPI1", 0x400);
        m.register_peripheral(0x4002_0000, "GPIOA", 0x400);
        m.register_peripheral(0x4002_0400, "GPIOB", 0x400);
        m.register_peripheral(0x4002_0800, "GPIOC", 0x400);
        m.register_peripheral(0x4002_0C00, "GPIOD", 0x400);
        m.register_peripheral(0x4002_3800, "RCC", 0x400);
        m.register_peripheral(0x4002_3C00, "FLASH", 0x400);
        m.register_peripheral(0xE000_E010, "SysTick", 0x10);
        m.register_peripheral(0xE000_E100, "NVIC", 0x300);
        m
    }
}

impl SvdPeripheralMap {
    pub fn register_peripheral(&mut self, base_addr: u32, name: &str, size: u32) {
        self.peripherals.insert(base_addr, (name.to_string(), size));
    }

    pub fn lookup(&self, address: u32) -> Option<String> {
        for (&base, (name, size)) in &self.peripherals {
            if address >= base && address < base + size {
                let offset = address - base;
                return Some(format!("{}+0x{:X}", name, offset));
            }
        }
        None
    }
}

/// RTOS Signature and RTIC/Embassy Detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtosDetectionResult {
    pub detected_rtos: Option<String>,
    pub confidence: f32,
    pub signatures_found: Vec<String>,
}

pub struct RtosDetector;

impl RtosDetector {
    pub fn detect(bytes: &[u8]) -> RtosDetectionResult {
        let mut signatures = Vec::new();

        // Convert slice to lossy ASCII string for pattern matching
        let ascii_str = String::from_utf8_lossy(bytes);

        if ascii_str.contains("vTaskStartScheduler")
            || ascii_str.contains("xTaskCreate")
            || ascii_str.contains("FreeRTOS")
        {
            signatures.push("FreeRTOS Task & Kernel API".to_string());
        }

        if ascii_str.contains("z_kernel")
            || ascii_str.contains("CONFIG_ZEPHYR")
            || ascii_str.contains("k_thread_create")
        {
            signatures.push("Zephyr RTOS Kernel Symbols".to_string());
        }

        if ascii_str.contains("embassy_executor") || ascii_str.contains("embassy_time") {
            signatures.push("Embassy Async Runtime for Embedded Rust".to_string());
        }

        if ascii_str.contains("rtic::") || ascii_str.contains("rtic_core") {
            signatures.push("RTIC (Real-Time Interrupt-driven Concurrency)".to_string());
        }

        let detected = if !signatures.is_empty() {
            Some(signatures[0].clone())
        } else {
            None
        };

        let confidence = if !signatures.is_empty() { 0.95 } else { 0.0 };

        RtosDetectionResult {
            detected_rtos: detected,
            confidence,
            signatures_found: signatures,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_scanner() {
        let zeros = vec![0u8; 1024];
        let chunks = EntropyScanner::scan(&zeros, 256);
        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0].entropy, 0.0);
        assert_eq!(chunks[0].classification, "Padding / Zeroes");
    }

    #[test]
    fn test_svd_peripheral_lookup() {
        let mapper = SvdPeripheralMap::default();
        let name = mapper.lookup(0x4002_0004); // GPIOA_OTYPER
        assert_eq!(name, Some("GPIOA+0x4".to_string()));
    }

    #[test]
    fn test_rtos_detection() {
        let mock_fw = b"Booting STM32... Calling vTaskStartScheduler and initializing FreeRTOS.";
        let res = RtosDetector::detect(mock_fw);
        assert!(res.detected_rtos.is_some());
        assert_eq!(res.confidence, 0.95);
    }
}
