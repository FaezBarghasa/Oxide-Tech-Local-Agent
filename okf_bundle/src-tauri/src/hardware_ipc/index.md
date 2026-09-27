# hardware_ipc

## Classs

- [ChipInfoDto](ChipInfoDto.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [CoreInfoDto](CoreInfoDto.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [FlashRequest](FlashRequest.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [FlashResult](FlashResult.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [MemoryRegionDto](MemoryRegionDto.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ProbeDeviceDto](ProbeDeviceDto.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ProbeDevicesResult](ProbeDevicesResult.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [SystemTelemetryPayload](SystemTelemetryPayload.md) — [derive(Debug, Clone, Serialize, Deserialize)]

## Functions

- [fetch_real_system_telemetry](fetch_real_system_telemetry.md)
- [flash_firmware](flash_firmware.md)
- [get_chip_info](get_chip_info.md)
- [get_system_telemetry](get_system_telemetry.md) — [tauri::command]
- [hardware_flash_firmware](hardware_flash_firmware.md) — [tauri::command]
- [hardware_get_chip_info](hardware_get_chip_info.md) — [tauri::command]
- [hardware_list_probes](hardware_list_probes.md) — [tauri::command]
- [list_probe_devices](list_probe_devices.md)
