import React, { useState, useEffect, useRef, useMemo } from 'react';
import { GraphNode, GraphLink } from '../types';
import {
  Network,
  Search,
  Filter,
  Layers,
  Cpu,
  Boxes,
  Code2,
  Radio,
  Zap,
  Activity,
  CheckCircle2,
  Sparkles,
  ArrowRight,
  ZoomIn,
  ZoomOut,
  RotateCcw,
  Sliders,
  Maximize2,
  Table,
  GitBranch,
  ShieldCheck,
  Flame,
  ExternalLink,
  ChevronRight,
  Info,
  Compass,
} from 'lucide-react';

const INITIAL_NODES: GraphNode[] = [
  // --- PROJECTS (Hub Nodes) ---
  {
    id: 'proj-drone',
    label: 'FCU-X4 Drone Flight Controller',
    category: 'project',
    subLabel: 'Quadcopter Avionics',
    description: 'High-reliability 4-rotor flight controller with dual SPI DMA IMU sampling, Kalman filter state estimator, and KiCad 4-layer power isolated PCB.',
    status: 'deployed',
    drcScore: 100,
    tokens: 3420,
    color: '#f97316',
    radius: 30,
    tags: ['Avionics', 'UAV', 'Flight Control', 'High-G'],
    metrics: { frequency: '480 MHz', voltage: '14.8V (4S)', busSpeed: '50 Mbps SPI', passRate: '99.4%' },
  },
  {
    id: 'proj-esc',
    label: 'RoboDrive-30A BLDC Motor ESC',
    category: 'project',
    subLabel: 'Field-Oriented Control',
    description: '3-phase brushless motor controller utilizing CORDIC accelerated Clarke/Park transforms, shunt current sensing, and 5Mbps CAN-FD telemetry.',
    status: 'active',
    drcScore: 98.5,
    tokens: 2890,
    color: '#f97316',
    radius: 28,
    tags: ['Robotics', 'Motor Control', 'FOC', 'Power Stage'],
    metrics: { frequency: '170 MHz', voltage: '24V / 30A', busSpeed: '5 Mbps CAN-FD', passRate: '98.5%' },
  },
  {
    id: 'proj-ble',
    label: 'MeshSense-NRF Sensor Node',
    category: 'project',
    subLabel: 'BLE Mesh Environmental',
    description: 'Ultra-low-power environmental telemetry node with solar harvesting, gas/VOC classification, and Bluetooth Mesh flood-routing.',
    status: 'deployed',
    drcScore: 100,
    tokens: 2150,
    color: '#f97316',
    radius: 26,
    tags: ['IoT', 'BLE Mesh', 'Environmental', 'Solar'],
    metrics: { frequency: '64 MHz', voltage: '3.3V LiFePO4', busSpeed: '1 Mbps BLE', passRate: '100%' },
  },
  {
    id: 'proj-lora',
    label: 'Oxide-LoraWAN Long Range Gateway',
    category: 'project',
    subLabel: 'Sub-GHz Secure Mesh',
    description: 'Long-range telemetry transceiver utilizing Semtech SX1262 with hardware AES-128 crypto acceleration, solar MPPT, and OLED telemetry.',
    status: 'deployed',
    drcScore: 99.2,
    tokens: 1980,
    color: '#f97316',
    radius: 26,
    tags: ['LoRaWAN', 'Sub-GHz', 'Crypto', 'Gateway'],
    metrics: { frequency: '133 MHz', voltage: '5V / Solar', busSpeed: '62.5 kbps LoRa', passRate: '99.2%' },
  },
  {
    id: 'proj-power',
    label: 'PowerForge-100W Buck-Boost PDU',
    category: 'project',
    subLabel: 'USB-PD 3.1 Smart PDU',
    description: '4-switch synchronous buck-boost power converter supporting USB-PD 3.1 Extended Power Range (28V 5A), INA228 20-bit telemetry, and I2C PMBus.',
    status: 'testing',
    drcScore: 97.8,
    tokens: 1650,
    color: '#f97316',
    radius: 25,
    tags: ['Power', 'Buck-Boost', 'USB-PD', 'PMBus'],
    metrics: { frequency: '48 MHz', voltage: '5V–28V / 5A', busSpeed: '1 MHz I2C', passRate: '97.8%' },
  },
  {
    id: 'proj-arm',
    label: 'ArmCore-V2 6-DOF Robot Kinematics',
    category: 'project',
    subLabel: 'Real-Time Industrial Kinematics',
    description: 'Dual-core robotic manipulator controller with real-time EtherCAT slave interface, Mojo SIMD inverse kinematics, and optical absolute encoders.',
    status: 'prototype',
    drcScore: 96.4,
    tokens: 4120,
    color: '#f97316',
    radius: 28,
    tags: ['Robotics', 'Kinematics', 'EtherCAT', 'Dual-Core'],
    metrics: { frequency: '480 MHz + 240 MHz', voltage: '24V DC', busSpeed: '100 Mbps EtherCAT', passRate: '96.4%' },
  },
  {
    id: 'proj-retimer',
    label: 'SerDes-Bridge PCIe Gen4 / USB 3.2',
    category: 'project',
    subLabel: 'High-Speed Signal Conditioner',
    description: '16 Gbps differential pair retimer board with 100Ω controlled impedance routing on Rogers 4350B substrate and user-space Rust diagnostics.',
    status: 'active',
    drcScore: 99.8,
    tokens: 1420,
    color: '#f97316',
    radius: 25,
    tags: ['High-Speed', 'PCIe Gen4', 'RF Stackup', 'Signal Integrity'],
    metrics: { frequency: '16 Gbps SerDes', voltage: '1.2V / 3.3V', busSpeed: '16 GT/s PCIe', passRate: '99.8%' },
  },
  {
    id: 'proj-studio',
    label: 'Oxide Hardware Agent Studio',
    category: 'project',
    subLabel: 'This AI Engineering Platform',
    description: 'Full-stack AI workspace with Unsloth GRPO RLVR tuning, SGLang TP=2 serving, gRPC KiCad bridge, Tree-Sitter AST compactor, and Mojo RAG.',
    status: 'active',
    drcScore: 100,
    tokens: 8900,
    color: '#f97316',
    radius: 32,
    tags: ['AI Agent', 'Unsloth', 'SGLang', 'gRPC', 'Compiler RL'],
    metrics: { frequency: 'Dual RTX 3090', voltage: '48 GB VRAM', busSpeed: 'gRPC :50051', passRate: '99.8%' },
  },

  // --- MCUs & SOCs ---
  {
    id: 'mcu-stm32h7',
    label: 'STM32H753ZI (Cortex-M7)',
    category: 'mcu',
    subLabel: '480MHz MCU',
    description: 'High-performance ARM Cortex-M7 with double-precision FPU, 2MB Flash, 1MB RAM, and dedicated cryptographic accelerator.',
    color: '#3b82f6',
    radius: 22,
    tags: ['ARM', 'Cortex-M7', 'STM32', 'DSP'],
    metrics: { frequency: '480 MHz', voltage: '3.3V', vramMb: 1 },
  },
  {
    id: 'mcu-stm32g4',
    label: 'STM32G474 (Cortex-M4)',
    category: 'mcu',
    subLabel: 'CORDIC + FMAC Motor MCU',
    description: 'Mixed-signal microcontroller featuring hardware CORDIC trigonometric coprocessor, high-resolution timers (184 ps), and CAN-FD.',
    color: '#3b82f6',
    radius: 20,
    tags: ['ARM', 'Cortex-M4', 'CORDIC', 'Motor Control'],
    metrics: { frequency: '170 MHz', voltage: '3.3V' },
  },
  {
    id: 'mcu-nrf52840',
    label: 'Nordic nRF52840 (Cortex-M4F)',
    category: 'mcu',
    subLabel: 'BLE 5.4 / Thread SoC',
    description: 'Multiprotocol wireless SoC with ARM Cortex-M4 with FPU, 1MB Flash, 256KB RAM, and native IEEE 802.15.4 / Bluetooth 5.4 radio.',
    color: '#3b82f6',
    radius: 20,
    tags: ['Nordic', 'BLE 5.4', 'Thread', 'Ultra-Low-Power'],
    metrics: { frequency: '64 MHz', voltage: '1.8V–3.6V' },
  },
  {
    id: 'mcu-rp2040',
    label: 'Raspberry Pi RP2040',
    category: 'mcu',
    subLabel: 'Dual Cortex-M0+ with PIO',
    description: 'Dual-core ARM Cortex-M0+ with programmable I/O (PIO) state machines, flexible DMA, and multi-channel USB interface.',
    color: '#3b82f6',
    radius: 19,
    tags: ['RP2040', 'PIO', 'Dual Core'],
    metrics: { frequency: '133 MHz', voltage: '3.3V' },
  },
  {
    id: 'mcu-stm32h745',
    label: 'STM32H745 (Dual Core M7/M4)',
    category: 'mcu',
    subLabel: 'Asymmetric Multiprocessing',
    description: 'Dual-core MCU combining 480MHz Cortex-M7 for heavy DSP/kinematics and 240MHz Cortex-M4 for hard real-time I/O.',
    color: '#3b82f6',
    radius: 21,
    tags: ['Dual-Core', 'AMP', 'Kinematics'],
    metrics: { frequency: '480 + 240 MHz', voltage: '3.3V' },
  },
  {
    id: 'mcu-stm32c0',
    label: 'STM32C011 (Cortex-M0+)',
    category: 'mcu',
    subLabel: 'Entry Microcontroller',
    description: 'Compact, cost-effective ARM Cortex-M0+ used for power supervision, PMBus state machine management, and thermal safety cutoffs.',
    color: '#3b82f6',
    radius: 17,
    tags: ['Cortex-M0+', 'Power Mgmt', 'PMBus'],
    metrics: { frequency: '48 MHz', voltage: '2.0V–3.6V' },
  },

  // --- FIRMWARE & CRATES ---
  {
    id: 'fw-embassy',
    label: 'Embassy Async Rust',
    category: 'firmware',
    subLabel: 'crates: embassy-executor, embassy-stm32',
    description: 'Next-generation async embedded Rust framework with zero-cost async/await, DMA-backed peripheral drivers, and low-power executor.',
    color: '#10b981',
    radius: 22,
    tags: ['Rust', 'Async', 'Embassy', 'Zero-Allocation'],
  },
  {
    id: 'fw-rtic',
    label: 'RTIC Real-Time Rust',
    category: 'firmware',
    subLabel: 'Real-Time Interrupt-driven Concurrency',
    description: 'Deterministic concurrency framework for ARM Cortex-M microcontrollers providing lock-free shared resources and priority ceiling emulation.',
    color: '#10b981',
    radius: 19,
    tags: ['Rust', 'Hard Real-Time', 'Lock-Free'],
  },
  {
    id: 'fw-zephyr',
    label: 'Zephyr RTOS & BLE Mesh',
    category: 'firmware',
    subLabel: 'Scalable Open Source RTOS',
    description: 'Comprehensive RTOS with certified Bluetooth Mesh stack, device power management subsystem, and standard sensor framework.',
    color: '#10b981',
    radius: 19,
    tags: ['C/Rust', 'BLE Mesh', 'RTOS'],
  },
  {
    id: 'fw-embedded-hal',
    label: 'embedded-hal v1.0',
    category: 'firmware',
    subLabel: 'Rust Hardware Abstraction Traits',
    description: 'Standard foundational Rust traits for SPI, I2C, UART, Digital I/O, and PWM enabling reusable platform-agnostic driver crates.',
    color: '#10b981',
    radius: 20,
    tags: ['Rust Traits', 'HAL', 'Portable'],
  },
  {
    id: 'fw-mojo-simd',
    label: 'Mojo SIMD Math Kernels',
    category: 'firmware',
    subLabel: 'Vectorized SIMD & RAG Embeddings',
    description: 'Hardware-accelerated SIMD computation kernels for matrix vectorization, cosine similarity embeddings, and inverse kinematic Jacobians.',
    color: '#10b981',
    radius: 20,
    tags: ['Mojo', 'SIMD', 'Matrix Math', 'RAG'],
  },

  // --- HARDWARE & ICS ---
  {
    id: 'hw-bmi088',
    label: 'Bosch BMI088 6-DOF IMU',
    category: 'hardware',
    subLabel: 'Vibration-Robust IMU',
    description: 'Automotive-grade 16-bit triaxial gyroscope and 16-bit accelerometer with active thermal compensation and low noise spectral density.',
    color: '#a855f7',
    radius: 18,
    tags: ['IMU', 'Gyroscope', 'Accelerometer', 'SPI'],
  },
  {
    id: 'hw-drv8301',
    label: 'TI DRV8301 60V Gate Driver',
    category: 'hardware',
    subLabel: '3-Phase BLDC Driver + Dual Shunt Amps',
    description: 'Integrated 3-phase MOSFET gate driver with dual programmable current shunt amplifiers and SPI diagnostic interface.',
    color: '#a855f7',
    radius: 18,
    tags: ['Gate Driver', 'MOSFET', 'Shunt Sense'],
  },
  {
    id: 'hw-bme688',
    label: 'Bosch BME688 Gas & VOC',
    category: 'hardware',
    subLabel: '4-in-1 Environmental Sensor',
    description: 'Gas, humidity, pressure, and temperature sensor with integrated AI gas scanner for VOC and odor classification.',
    color: '#a855f7',
    radius: 18,
    tags: ['Gas', 'VOC', 'Pressure', 'Temp', 'I2C'],
  },
  {
    id: 'hw-sx1262',
    label: 'Semtech SX1262 LoRa Radio',
    category: 'hardware',
    subLabel: '+22dBm Sub-GHz Transceiver',
    description: 'Long-range Sub-GHz transceiver operating from 150–960 MHz with high sensitivity down to -148 dBm and integrated DC-DC converter.',
    color: '#a855f7',
    radius: 18,
    tags: ['LoRa', 'Sub-GHz', 'RF', 'SPI'],
  },
  {
    id: 'hw-lm5176',
    label: 'TI LM5176 4-Switch Buck-Boost',
    category: 'hardware',
    subLabel: 'High Efficiency Sync Controller',
    description: 'Synchronous 4-switch buck-boost DC/DC controller with wide input voltage range (4.2V to 55V) and seamless transition between modes.',
    color: '#a855f7',
    radius: 18,
    tags: ['Buck-Boost', 'Power Stage', '4-Switch'],
  },
  {
    id: 'hw-atecc608a',
    label: 'Microchip ATECC608A Crypto',
    category: 'hardware',
    subLabel: 'Hardware Secure Element',
    description: 'Cryptographic coprocessor with hardware-based secure key storage, ECDSA signing, AES-128 acceleration, and hardware TRNG.',
    color: '#a855f7',
    radius: 17,
    tags: ['Crypto', 'ECDSA', 'AES-128', 'Secure Boot'],
  },

  // --- PROTOCOLS & BUSES ---
  {
    id: 'proto-spidma',
    label: 'Dual SPI DMA (50 MHz)',
    category: 'protocol',
    subLabel: 'Zero-Copy Sensor Stream',
    description: 'High-speed circular DMA buffer transfers over SPI bus enabling 2 kHz IMU sampling with zero CPU intervention.',
    color: '#eab308',
    radius: 17,
    tags: ['SPI', 'DMA', 'Zero-Copy'],
  },
  {
    id: 'proto-canfd',
    label: 'CAN-FD (5 Mbps)',
    category: 'protocol',
    subLabel: 'Flexible Data-Rate Bus',
    description: 'Robust automotive-grade differential communications bus supporting 64-byte payloads at up to 5 Mbps bitrates with CRC-21 verification.',
    color: '#eab308',
    radius: 17,
    tags: ['CAN-FD', 'Differential', 'Automotive'],
  },
  {
    id: 'proto-blemesh',
    label: 'Bluetooth 5.4 Mesh',
    category: 'protocol',
    subLabel: 'Multi-Hop Flood Routing',
    description: 'Standardized mesh networking topology for Bluetooth Low Energy enabling thousands of nodes to communicate reliably without router hubs.',
    color: '#eab308',
    radius: 17,
    tags: ['BLE', 'Mesh', 'Flood Routing'],
  },
  {
    id: 'proto-lorawan',
    label: 'LoRaWAN / P2P LoRa',
    category: 'protocol',
    subLabel: 'Long Range Spread Spectrum',
    description: 'CSS (Chirp Spread Spectrum) modulated radio packets reaching up to 15km line-of-sight in 868/915 MHz ISM bands.',
    color: '#eab308',
    radius: 17,
    tags: ['LoRaWAN', 'CSS', 'Sub-GHz'],
  },
  {
    id: 'proto-ethercat',
    label: 'EtherCAT 100BASE-TX',
    category: 'protocol',
    subLabel: 'Hard Real-Time Industrial Bus',
    description: 'Ethernet-based fieldbus system with on-the-fly frame processing achieving sub-millisecond cycle times with nanosecond jitter.',
    color: '#eab308',
    radius: 17,
    tags: ['EtherCAT', 'Real-Time', 'Ethernet'],
  },
  {
    id: 'proto-grpc',
    label: 'gRPC Protobuf Bridge (:50051)',
    category: 'protocol',
    subLabel: 'Rust / Python CAD Inter-Op',
    description: 'High-throughput binary RPC protocol connecting KiCad 8/9 Python scripting environments, FreeCAD kernels, and Rust agents.',
    color: '#eab308',
    radius: 18,
    tags: ['gRPC', 'Protobuf', 'CAD Bridge'],
  },

  // --- AI LORA ADAPTERS ---
  {
    id: 'lora-embassy',
    label: 'LoRA: Embassy HAL Specialist',
    category: 'lora',
    subLabel: 'Rank 16 · Weight 0.85',
    description: 'Fine-tuned QDoRA adapter on 1,400 embedded-hal & embassy-stm32 DMA peripheral drivers with verified 99.4% compilation pass rate.',
    color: '#ec4899',
    radius: 19,
    tags: ['LoRA', 'Embassy', 'Rust AST', 'QDoRA'],
  },
  {
    id: 'lora-control',
    label: 'LoRA: Control Theory & FOC',
    category: 'lora',
    subLabel: 'Rank 16 · Weight 0.90',
    description: 'Specialized model weights for Clarke/Park transform math, space vector PWM (SVPWM), and cascade PID loop tuning.',
    color: '#ec4899',
    radius: 19,
    tags: ['LoRA', 'FOC', 'PID', 'Motor Math'],
  },
  {
    id: 'lora-kicad',
    label: 'LoRA: KiCad DRC & Netlists',
    category: 'lora',
    subLabel: 'Rank 16 · Weight 0.92',
    description: 'Trained on 4,200 KiCad 8/9 S-expression schematics and layout rule netlists to generate zero-violation PCB layouts.',
    color: '#ec4899',
    radius: 20,
    tags: ['LoRA', 'KiCad', 'DRC', 'S-Expr'],
  },
  {
    id: 'lora-crypto',
    label: 'LoRA: Secure Elements & Crypto',
    category: 'lora',
    subLabel: 'Rank 16 · Weight 0.80',
    description: 'Expertise in ECDSA hardware signing, AES-128 Galois counter mode, secure bootloader verification, and anti-tamper nets.',
    color: '#ec4899',
    radius: 18,
    tags: ['LoRA', 'Crypto', 'Secure Boot'],
  },
  {
    id: 'lora-kinematics',
    label: 'LoRA: Kinematics & SIMD',
    category: 'lora',
    subLabel: 'Rank 16 · Weight 0.88',
    description: 'Fine-tuned for 6-DOF Denavit-Hartenberg parameter calculation, forward/inverse kinematic solvers, and Mojo SIMD kernels.',
    color: '#ec4899',
    radius: 19,
    tags: ['LoRA', 'Kinematics', 'Robotics', 'SIMD'],
  },
];

const INITIAL_LINKS: GraphLink[] = [
  // Proj: Drone Flight Controller
  { source: 'proj-drone', target: 'mcu-stm32h7', label: 'runs_on', type: 'implements' },
  { source: 'proj-drone', target: 'fw-embassy', label: 'coded_in', type: 'uses' },
  { source: 'proj-drone', target: 'hw-bmi088', label: 'samples_imu', type: 'integrates' },
  { source: 'proj-drone', target: 'proto-spidma', label: 'streams_via', type: 'communicates' },
  { source: 'proj-drone', target: 'lora-embassy', label: 'guided_by', type: 'assisted_by' },
  { source: 'proj-drone', target: 'lora-kicad', label: 'pcb_verified', type: 'verifies' },

  // Proj: RoboDrive ESC
  { source: 'proj-esc', target: 'mcu-stm32g4', label: 'runs_on', type: 'implements' },
  { source: 'proj-esc', target: 'fw-rtic', label: 'coded_in', type: 'uses' },
  { source: 'proj-esc', target: 'hw-drv8301', label: 'drives_mosfet', type: 'integrates' },
  { source: 'proj-esc', target: 'proto-canfd', label: 'telemetry_on', type: 'communicates' },
  { source: 'proj-esc', target: 'lora-control', label: 'foc_math_by', type: 'assisted_by' },
  { source: 'proj-esc', target: 'lora-kicad', label: 'drc_verified', type: 'verifies' },

  // Proj: MeshSense BLE Node
  { source: 'proj-ble', target: 'mcu-nrf52840', label: 'runs_on', type: 'implements' },
  { source: 'proj-ble', target: 'fw-zephyr', label: 'coded_in', type: 'uses' },
  { source: 'proj-ble', target: 'hw-bme688', label: 'reads_voc', type: 'integrates' },
  { source: 'proj-ble', target: 'proto-blemesh', label: 'broadcasts_on', type: 'communicates' },
  { source: 'proj-ble', target: 'fw-embedded-hal', label: 'uses_hal', type: 'uses' },

  // Proj: Oxide LoRaWAN Gateway
  { source: 'proj-lora', target: 'mcu-rp2040', label: 'runs_on', type: 'implements' },
  { source: 'proj-lora', target: 'hw-sx1262', label: 'radio_transceiver', type: 'integrates' },
  { source: 'proj-lora', target: 'hw-atecc608a', label: 'secures_keys', type: 'integrates' },
  { source: 'proj-lora', target: 'proto-lorawan', label: 'mesh_packets', type: 'communicates' },
  { source: 'proj-lora', target: 'fw-embedded-hal', label: 'uses_hal', type: 'uses' },
  { source: 'proj-lora', target: 'lora-crypto', label: 'crypto_logic', type: 'assisted_by' },

  // Proj: PowerForge Buck-Boost
  { source: 'proj-power', target: 'mcu-stm32c0', label: 'supervises', type: 'implements' },
  { source: 'proj-power', target: 'hw-lm5176', label: 'switch_controller', type: 'integrates' },
  { source: 'proj-power', target: 'fw-embedded-hal', label: 'i2c_pmbus_hal', type: 'uses' },
  { source: 'proj-power', target: 'lora-kicad', label: 'power_plane_drc', type: 'verifies' },

  // Proj: ArmCore-V2 Kinematics
  { source: 'proj-arm', target: 'mcu-stm32h745', label: 'dual_core_amp', type: 'implements' },
  { source: 'proj-arm', target: 'fw-embassy', label: 'async_executor', type: 'uses' },
  { source: 'proj-arm', target: 'fw-mojo-simd', label: 'jacobian_simd', type: 'uses' },
  { source: 'proj-arm', target: 'proto-ethercat', label: 'motion_bus', type: 'communicates' },
  { source: 'proj-arm', target: 'lora-kinematics', label: 'inverse_ik', type: 'assisted_by' },

  // Proj: SerDes PCIe Retimer
  { source: 'proj-retimer', target: 'lora-kicad', label: 'diff_pair_drc', type: 'verifies' },
  { source: 'proj-retimer', target: 'proto-grpc', label: 'cad_scripting', type: 'communicates' },

  // Proj: Oxide Studio Platform
  { source: 'proj-studio', target: 'proto-grpc', label: 'drc_bridge', type: 'communicates' },
  { source: 'proj-studio', target: 'fw-mojo-simd', label: 'rag_embeddings', type: 'uses' },
  { source: 'proj-studio', target: 'lora-embassy', label: 'serves_adapter', type: 'assisted_by' },
  { source: 'proj-studio', target: 'lora-control', label: 'serves_adapter', type: 'assisted_by' },
  { source: 'proj-studio', target: 'lora-kicad', label: 'serves_adapter', type: 'assisted_by' },
  { source: 'proj-studio', target: 'lora-crypto', label: 'serves_adapter', type: 'assisted_by' },
  { source: 'proj-studio', target: 'lora-kinematics', label: 'serves_adapter', type: 'assisted_by' },
];

export const KnowledgeGraphTab: React.FC = () => {
  const [nodes, setNodes] = useState<GraphNode[]>(INITIAL_NODES);
  const [links] = useState<GraphLink[]>(INITIAL_LINKS);
  const [selectedNodeId, setSelectedNodeId] = useState<string>('proj-drone');
  const [hoveredNodeId, setHoveredNodeId] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [categoryFilter, setCategoryFilter] = useState<string>('all');
  const [viewMode, setViewMode] = useState<'graph' | 'matrix' | 'tree' | 'insights'>('graph');
  const [zoomLevel, setZoomLevel] = useState<number>(1);
  const [panOffset, setPanOffset] = useState<{ x: number; y: number }>({ x: 0, y: 0 });
  const [isPhysicsActive, setIsPhysicsActive] = useState<boolean>(true);
  const [minDrcFilter, setMinDrcFilter] = useState<number>(90);

  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const containerRef = useRef<HTMLDivElement | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const draggingNodeRef = useRef<{ id: string; offsetX: number; offsetY: number } | null>(null);
  const isPanningRef = useRef<{ startX: number; startY: number; initialPanX: number; initialPanY: number } | null>(null);

  // Initialize node positions with radial cluster distribution
  useEffect(() => {
    const width = 800;
    const height = 550;
    const centerX = width / 2;
    const centerY = height / 2;

    const initialized = INITIAL_NODES.map((node, index) => {
      let angle = (index / INITIAL_NODES.length) * 2 * Math.PI;
      let dist = 180;

      if (node.category === 'project') {
        dist = 90 + (index % 4) * 35;
      } else if (node.category === 'mcu') {
        dist = 200;
        angle += 0.2;
      } else if (node.category === 'firmware') {
        dist = 230;
      } else if (node.category === 'hardware') {
        dist = 250;
      } else if (node.category === 'protocol') {
        dist = 210;
      } else if (node.category === 'lora') {
        dist = 260;
      }

      return {
        ...node,
        x: centerX + Math.cos(angle) * dist + (Math.random() - 0.5) * 40,
        y: centerY + Math.sin(angle) * dist + (Math.random() - 0.5) * 40,
        vx: 0,
        vy: 0,
      };
    });

    setNodes(initialized);
  }, []);

  // Simple Physics Force Engine (Springs + Coulomb Repulsion + Center Gravity)
  useEffect(() => {
    if (!isPhysicsActive) return;

    let frameCount = 0;
    const stepPhysics = () => {
      setNodes((prevNodes) => {
        const width = 800;
        const height = 550;
        const centerX = width / 2;
        const centerY = height / 2;
        const nextNodes = prevNodes.map((n) => ({ ...n }));
        const nodeMap = new Map<string, (typeof nextNodes)[0]>();
        nextNodes.forEach((n) => nodeMap.set(n.id, n));

        // 1. Center Gravity
        for (const n of nextNodes) {
          if (draggingNodeRef.current?.id === n.id) continue;
          const dx = centerX - (n.x || centerX);
          const dy = centerY - (n.y || centerY);
          n.vx = ((n.vx || 0) + dx * 0.0006) * 0.88;
          n.vy = ((n.vy || 0) + dy * 0.0006) * 0.88;
        }

        // 2. Repulsion between all nodes
        for (let i = 0; i < nextNodes.length; i++) {
          for (let j = i + 1; j < nextNodes.length; j++) {
            const a = nextNodes[i];
            const b = nextNodes[j];
            const dx = (b.x || 0) - (a.x || 0);
            const dy = (b.y || 0) - (a.y || 0);
            const distSq = dx * dx + dy * dy + 100;
            const dist = Math.sqrt(distSq);
            const minAllowedDist = (a.radius || 20) + (b.radius || 20) + 15;

            const force = 1800 / distSq + (dist < minAllowedDist ? (minAllowedDist - dist) * 0.08 : 0);
            const fx = (dx / dist) * force;
            const fy = (dy / dist) * force;

            if (draggingNodeRef.current?.id !== a.id) {
              a.vx = (a.vx || 0) - fx;
              a.vy = (a.vy || 0) - fy;
            }
            if (draggingNodeRef.current?.id !== b.id) {
              b.vx = (b.vx || 0) + fx;
              b.vy = (b.vy || 0) + fy;
            }
          }
        }

        // 3. Spring Attraction along Links
        for (const link of links) {
          const a = nodeMap.get(link.source);
          const b = nodeMap.get(link.target);
          if (!a || !b) continue;

          const dx = (b.x || 0) - (a.x || 0);
          const dy = (b.y || 0) - (a.y || 0);
          const dist = Math.sqrt(dx * dx + dy * dy) || 1;
          const targetDist = 110;
          const springForce = (dist - targetDist) * 0.008;

          const fx = (dx / dist) * springForce;
          const fy = (dy / dist) * springForce;

          if (draggingNodeRef.current?.id !== a.id) {
            a.vx = (a.vx || 0) + fx;
            a.vy = (a.vy || 0) + fy;
          }
          if (draggingNodeRef.current?.id !== b.id) {
            b.vx = (b.vx || 0) - fx;
            b.vy = (b.vy || 0) - fy;
          }
        }

        // 4. Position Update & Boundary Clamping
        for (const n of nextNodes) {
          if (draggingNodeRef.current?.id === n.id) continue;
          n.x = Math.max(30, Math.min(width - 30, (n.x || centerX) + (n.vx || 0)));
          n.y = Math.max(30, Math.min(height - 30, (n.y || centerY) + (n.vy || 0)));
        }

        return nextNodes;
      });

      frameCount++;
      // Auto stabilize after initial convergence
      if (frameCount < 240) {
        animationFrameRef.current = requestAnimationFrame(stepPhysics);
      }
    };

    animationFrameRef.current = requestAnimationFrame(stepPhysics);
    return () => {
      if (animationFrameRef.current) cancelAnimationFrame(animationFrameRef.current);
    };
  }, [isPhysicsActive, links]);

  // Selected Node Neighbors
  const connectedNodeIds = useMemo(() => {
    if (!selectedNodeId) return new Set<string>();
    const set = new Set<string>([selectedNodeId]);
    links.forEach((l) => {
      if (l.source === selectedNodeId) set.add(l.target);
      if (l.target === selectedNodeId) set.add(l.source);
    });
    return set;
  }, [selectedNodeId, links]);

  const selectedNode = useMemo(() => {
    return nodes.find((n) => n.id === selectedNodeId) || nodes[0];
  }, [nodes, selectedNodeId]);

  const selectedNodeLinks = useMemo(() => {
    return links.filter((l) => l.source === selectedNodeId || l.target === selectedNodeId);
  }, [links, selectedNodeId]);

  // Filtering
  const filteredNodes = useMemo(() => {
    return nodes.filter((n) => {
      const matchCat = categoryFilter === 'all' || n.category === categoryFilter;
      const matchSearch =
        searchQuery.trim() === '' ||
        n.label.toLowerCase().includes(searchQuery.toLowerCase()) ||
        n.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (n.tags && n.tags.some((t) => t.toLowerCase().includes(searchQuery.toLowerCase())));
      const matchDrc = !n.drcScore || n.drcScore >= minDrcFilter;
      return matchCat && matchSearch && matchDrc;
    });
  }, [nodes, categoryFilter, searchQuery, minDrcFilter]);

  // Canvas Mouse Interaction Handlers
  const handleCanvasMouseDown = (e: React.MouseEvent<HTMLDivElement>) => {
    const container = containerRef.current;
    if (!container) return;
    const rect = container.getBoundingClientRect();
    const clientX = e.clientX - rect.left;
    const clientY = e.clientY - rect.top;

    // Convert to graph coordinate space taking zoom and pan into account
    const graphX = (clientX - panOffset.x) / zoomLevel;
    const graphY = (clientY - panOffset.y) / zoomLevel;

    // Check if clicked a node
    let clickedNode: GraphNode | null = null;
    for (let i = nodes.length - 1; i >= 0; i--) {
      const n = nodes[i];
      const dx = graphX - (n.x || 0);
      const dy = graphY - (n.y || 0);
      const dist = Math.sqrt(dx * dx + dy * dy);
      if (dist <= (n.radius || 20)) {
        clickedNode = n;
        break;
      }
    }

    if (clickedNode) {
      setSelectedNodeId(clickedNode.id);
      draggingNodeRef.current = {
        id: clickedNode.id,
        offsetX: (clickedNode.x || 0) - graphX,
        offsetY: (clickedNode.y || 0) - graphY,
      };
    } else {
      // Start panning
      isPanningRef.current = {
        startX: e.clientX,
        startY: e.clientY,
        initialPanX: panOffset.x,
        initialPanY: panOffset.y,
      };
    }
  };

  const handleCanvasMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    const container = containerRef.current;
    if (!container) return;
    const rect = container.getBoundingClientRect();
    const clientX = e.clientX - rect.left;
    const clientY = e.clientY - rect.top;

    if (draggingNodeRef.current) {
      const graphX = (clientX - panOffset.x) / zoomLevel;
      const graphY = (clientY - panOffset.y) / zoomLevel;

      setNodes((prev) =>
        prev.map((n) =>
          n.id === draggingNodeRef.current?.id
            ? { ...n, x: graphX + draggingNodeRef.current.offsetX, y: graphY + draggingNodeRef.current.offsetY, vx: 0, vy: 0 }
            : n
        )
      );
    } else if (isPanningRef.current) {
      const dx = e.clientX - isPanningRef.current.startX;
      const dy = e.clientY - isPanningRef.current.startY;
      setPanOffset({
        x: isPanningRef.current.initialPanX + dx,
        y: isPanningRef.current.initialPanY + dy,
      });
    } else {
      // Hover detection
      const graphX = (clientX - panOffset.x) / zoomLevel;
      const graphY = (clientY - panOffset.y) / zoomLevel;
      let found: string | null = null;
      for (const n of nodes) {
        const dx = graphX - (n.x || 0);
        const dy = graphY - (n.y || 0);
        if (Math.sqrt(dx * dx + dy * dy) <= (n.radius || 20)) {
          found = n.id;
          break;
        }
      }
      setHoveredNodeId(found);
    }
  };

  const handleCanvasMouseUp = () => {
    draggingNodeRef.current = null;
    isPanningRef.current = null;
  };

  const handleResetView = () => {
    setZoomLevel(1);
    setPanOffset({ x: 0, y: 0 });
    setIsPhysicsActive(true);
  };

  const getCategoryBadge = (cat: string) => {
    switch (cat) {
      case 'project':
        return <span className="px-2 py-0.5 rounded text-[10px] mono font-bold bg-orange-500/20 text-orange-400 border border-orange-500/30">PROJECT</span>;
      case 'mcu':
        return <span className="px-2 py-0.5 rounded text-[10px] mono font-bold bg-blue-500/20 text-blue-400 border border-blue-500/30">MCU / SOC</span>;
      case 'firmware':
        return <span className="px-2 py-0.5 rounded text-[10px] mono font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">FIRMWARE</span>;
      case 'hardware':
        return <span className="px-2 py-0.5 rounded text-[10px] mono font-bold bg-purple-500/20 text-purple-400 border border-purple-500/30">HARDWARE IC</span>;
      case 'protocol':
        return <span className="px-2 py-0.5 rounded text-[10px] mono font-bold bg-amber-500/20 text-amber-400 border border-amber-500/30">PROTOCOL BUS</span>;
      case 'lora':
        return <span className="px-2 py-0.5 rounded text-[10px] mono font-bold bg-pink-500/20 text-pink-400 border border-pink-500/30">AI LORA ADAPTER</span>;
      default:
        return null;
    }
  };

  // Node Icon Helper
  const getNodeIcon = (cat: string) => {
    switch (cat) {
      case 'project':
        return <Boxes className="w-3.5 h-3.5 text-orange-400" />;
      case 'mcu':
        return <Cpu className="w-3.5 h-3.5 text-blue-400" />;
      case 'firmware':
        return <Code2 className="w-3.5 h-3.5 text-emerald-400" />;
      case 'hardware':
        return <Layers className="w-3.5 h-3.5 text-purple-400" />;
      case 'protocol':
        return <Radio className="w-3.5 h-3.5 text-amber-400" />;
      case 'lora':
        return <Zap className="w-3.5 h-3.5 text-pink-400" />;
      default:
        return <Activity className="w-3.5 h-3.5 text-gray-400" />;
    }
  };

  return (
    <div className="space-y-6 font-sans">
      {/* Header Bar */}
      <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4 bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl">
        <div>
          <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
            <Network className="w-4 h-4 text-orange-400" />
            <span>Workspace Knowledge Graph & Project Topology</span>
          </div>
          <div className="text-[10px] mono text-gray-400 mt-0.5">
            Relational hardware graph connecting embedded firmware, KiCad schematics, MCU targets, and fine-tuned AI adapters
          </div>
        </div>

        {/* View Mode & Statistics */}
        <div className="flex flex-wrap items-center gap-2.5">
          <div className="flex items-center gap-1 bg-[#14151e] border border-[#232530] rounded-lg p-0.5 text-[11px] mono">
            <button
              onClick={() => setViewMode('graph')}
              className={`px-3 py-1 rounded transition flex items-center gap-1.5 cursor-pointer ${
                viewMode === 'graph' ? 'bg-orange-500 text-gray-950 font-bold' : 'text-gray-400 hover:text-white'
              }`}
            >
              <Network className="w-3 h-3" />
              <span>Force Graph</span>
            </button>
            <button
              onClick={() => setViewMode('matrix')}
              className={`px-3 py-1 rounded transition flex items-center gap-1.5 cursor-pointer ${
                viewMode === 'matrix' ? 'bg-orange-500 text-gray-950 font-bold' : 'text-gray-400 hover:text-white'
              }`}
            >
              <Table className="w-3 h-3" />
              <span>Project Matrix</span>
            </button>
            <button
              onClick={() => setViewMode('tree')}
              className={`px-3 py-1 rounded transition flex items-center gap-1.5 cursor-pointer ${
                viewMode === 'tree' ? 'bg-orange-500 text-gray-950 font-bold' : 'text-gray-400 hover:text-white'
              }`}
            >
              <GitBranch className="w-3 h-3" />
              <span>Hierarchy Tree</span>
            </button>
            <button
              onClick={() => setViewMode('insights')}
              className={`px-3 py-1 rounded transition flex items-center gap-1.5 cursor-pointer ${
                viewMode === 'insights' ? 'bg-orange-500 text-gray-950 font-bold' : 'text-gray-400 hover:text-white'
              }`}
            >
              <Activity className="w-3 h-3" />
              <span>Reuse Insights</span>
            </button>
          </div>

          <span className="text-[10px] mono px-3 py-1 rounded-md bg-orange-500/10 text-orange-400 border border-orange-500/30 font-bold flex items-center gap-1.5">
            <span className="w-1.5 h-1.5 rounded-full bg-orange-400 animate-pulse" />
            {nodes.length} Nodes · {links.length} Relations
          </span>
        </div>
      </div>

      {/* Main Layout Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
        {/* Graph Canvas & Filtering Controls (Left 8 Cols) */}
        <div className="lg:col-span-8 space-y-4">
          {/* Filter & Search Toolbar */}
          <div className="bg-[#111217] border border-[#232530] rounded-xl p-3.5 flex flex-wrap items-center justify-between gap-3 shadow-md">
            {/* Search Input */}
            <div className="relative flex-1 min-w-[200px]">
              <Search className="w-3.5 h-3.5 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
              <input
                type="text"
                placeholder="Search projects, MCUs (STM32, nRF), crates, or buses..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full bg-[#14151e] border border-[#262838] focus:border-orange-500 rounded-lg pl-9 pr-3 py-1.5 text-xs text-gray-200 placeholder-gray-500 focus:outline-none transition"
              />
            </div>

            {/* Category Filter Pills */}
            <div className="flex flex-wrap items-center gap-1 text-[10px] mono">
              {[
                { id: 'all', label: 'All' },
                { id: 'project', label: 'Projects' },
                { id: 'mcu', label: 'MCUs' },
                { id: 'firmware', label: 'Firmware' },
                { id: 'hardware', label: 'Hardware' },
                { id: 'protocol', label: 'Protocols' },
                { id: 'lora', label: 'LoRAs' },
              ].map((cat) => (
                <button
                  key={cat.id}
                  onClick={() => setCategoryFilter(cat.id)}
                  className={`px-2 py-1 rounded transition cursor-pointer ${
                    categoryFilter === cat.id
                      ? 'bg-orange-500/20 text-orange-300 border border-orange-500/40 font-bold'
                      : 'text-gray-400 hover:text-white bg-[#14151e] border border-[#232530]'
                  }`}
                >
                  {cat.label}
                </button>
              ))}
            </div>

            {/* Canvas Zoom & Physics Controls */}
            {viewMode === 'graph' && (
              <div className="flex items-center gap-1.5 border-l border-[#232530] pl-3">
                <button
                  onClick={() => setZoomLevel((z) => Math.min(2.5, z + 0.15))}
                  className="p-1.5 rounded-lg bg-[#14151e] hover:bg-[#1f212e] text-gray-400 hover:text-white border border-[#232530] transition cursor-pointer"
                  title="Zoom In"
                >
                  <ZoomIn className="w-3.5 h-3.5" />
                </button>
                <button
                  onClick={() => setZoomLevel((z) => Math.max(0.4, z - 0.15))}
                  className="p-1.5 rounded-lg bg-[#14151e] hover:bg-[#1f212e] text-gray-400 hover:text-white border border-[#232530] transition cursor-pointer"
                  title="Zoom Out"
                >
                  <ZoomOut className="w-3.5 h-3.5" />
                </button>
                <button
                  onClick={handleResetView}
                  className="p-1.5 rounded-lg bg-[#14151e] hover:bg-[#1f212e] text-gray-400 hover:text-white border border-[#232530] transition cursor-pointer"
                  title="Reset Pan & Zoom"
                >
                  <RotateCcw className="w-3.5 h-3.5" />
                </button>
              </div>
            )}
          </div>

          {/* VIEW: 1. FORCE GRAPH CANVAS */}
          {viewMode === 'graph' && (
            <div
              ref={containerRef}
              onMouseDown={handleCanvasMouseDown}
              onMouseMove={handleCanvasMouseMove}
              onMouseUp={handleCanvasMouseUp}
              className="relative w-full h-[580px] bg-[#0c0d12] border border-[#232530] rounded-2xl overflow-hidden shadow-2xl cursor-grab active:cursor-grabbing select-none"
            >
              {/* Background Grid Pattern */}
              <div
                className="absolute inset-0 opacity-15 pointer-events-none"
                style={{
                  backgroundImage: `radial-gradient(#f97316 1px, transparent 1px)`,
                  backgroundSize: `${24 * zoomLevel}px ${24 * zoomLevel}px`,
                  backgroundPosition: `${panOffset.x}px ${panOffset.y}px`,
                }}
              />

              {/* Ambient Glow behind Selected Node */}
              {selectedNode && selectedNode.x && selectedNode.y && (
                <div
                  className="absolute pointer-events-none transition-all duration-300 rounded-full blur-3xl opacity-20"
                  style={{
                    left: `${selectedNode.x * zoomLevel + panOffset.x - 120}px`,
                    top: `${selectedNode.y * zoomLevel + panOffset.y - 120}px`,
                    width: '240px',
                    height: '240px',
                    backgroundColor: selectedNode.color || '#f97316',
                  }}
                />
              )}

              {/* SVG Layer for Links / Connections */}
              <svg className="absolute inset-0 w-full h-full pointer-events-none">
                <g transform={`translate(${panOffset.x}, ${panOffset.y}) scale(${zoomLevel})`}>
                  {links.map((link, idx) => {
                    const sourceNode = nodes.find((n) => n.id === link.source);
                    const targetNode = nodes.find((n) => n.id === link.target);
                    if (!sourceNode || !targetNode || !sourceNode.x || !targetNode.x) return null;

                    const isConnectedToSelected =
                      sourceNode.id === selectedNodeId || targetNode.id === selectedNodeId;
                    const isHovered =
                      sourceNode.id === hoveredNodeId || targetNode.id === hoveredNodeId;

                    let strokeColor = '#2e3245';
                    let strokeWidth = 1.2;
                    let strokeOpacity = 0.4;

                    if (isConnectedToSelected) {
                      strokeColor = '#f97316';
                      strokeWidth = 2.2;
                      strokeOpacity = 0.9;
                    } else if (isHovered) {
                      strokeColor = '#38bdf8';
                      strokeWidth = 1.8;
                      strokeOpacity = 0.8;
                    }

                    return (
                      <g key={`link-${idx}`}>
                        <line
                          x1={sourceNode.x}
                          y1={sourceNode.y}
                          x2={targetNode.x}
                          y2={targetNode.y}
                          stroke={strokeColor}
                          strokeWidth={strokeWidth}
                          strokeOpacity={strokeOpacity}
                          strokeDasharray={link.type === 'verifies' ? '4 3' : link.type === 'assisted_by' ? '3 3' : undefined}
                        />
                        {/* Link Label on Hover / Selection */}
                        {(isConnectedToSelected || isHovered) && (
                          <text
                            x={((sourceNode.x || 0) + (targetNode.x || 0)) / 2}
                            y={((sourceNode.y || 0) + (targetNode.y || 0)) / 2 - 4}
                            fill="#94a3b8"
                            fontSize="8"
                            fontFamily="monospace"
                            textAnchor="middle"
                            className="bg-[#0b0c10]"
                          >
                            {link.label}
                          </text>
                        )}
                      </g>
                    );
                  })}
                </g>
              </svg>

              {/* HTML Nodes Layer */}
              <div
                className="absolute inset-0 origin-top-left pointer-events-none"
                style={{
                  transform: `translate(${panOffset.x}px, ${panOffset.y}px) scale(${zoomLevel})`,
                }}
              >
                {filteredNodes.map((node) => {
                  const isSelected = node.id === selectedNodeId;
                  const isConnected = connectedNodeIds.has(node.id);
                  const isHovered = node.id === hoveredNodeId;
                  const radius = node.radius || 20;

                  return (
                    <div
                      key={node.id}
                      className={`absolute rounded-full flex flex-col items-center justify-center pointer-events-auto transition-transform duration-150 cursor-pointer shadow-lg ${
                        isSelected
                          ? 'ring-2 ring-orange-400 ring-offset-2 ring-offset-[#0b0c10] scale-110 z-30 shadow-[0_0_20px_rgba(249,115,22,0.6)]'
                          : isConnected
                          ? 'ring-1 ring-orange-500/60 z-20 opacity-100'
                          : isHovered
                          ? 'ring-2 ring-white/50 scale-105 z-25'
                          : 'opacity-85 hover:opacity-100 z-10'
                      }`}
                      style={{
                        left: `${(node.x || 0) - radius}px`,
                        top: `${(node.y || 0) - radius}px`,
                        width: `${radius * 2}px`,
                        height: `${radius * 2}px`,
                        backgroundColor: '#13141d',
                        borderColor: isSelected ? '#f97316' : node.color || '#3b82f6',
                        borderWidth: isSelected ? '2.5px' : '1.5px',
                      }}
                    >
                      {/* Node Center Icon */}
                      <div className="flex items-center justify-center">
                        {getNodeIcon(node.category)}
                      </div>

                      {/* Label Overlay */}
                      <div className="absolute top-full mt-1.5 px-1.5 py-0.5 rounded bg-[#0b0c10]/90 border border-[#232530] text-[9px] mono text-gray-200 whitespace-nowrap shadow-md pointer-events-none flex items-center gap-1">
                        <span className="truncate max-w-[120px]">{node.label}</span>
                        {node.drcScore && (
                          <span className="text-emerald-400 font-bold">{node.drcScore}%</span>
                        )}
                      </div>
                    </div>
                  );
                })}
              </div>

              {/* Canvas Overlay Floating Legend */}
              <div className="absolute bottom-3 left-3 bg-[#0e1017]/90 border border-[#232530] rounded-xl p-2.5 backdrop-blur-md text-[10px] mono flex flex-wrap items-center gap-3 text-gray-300 shadow-xl pointer-events-auto">
                <span className="text-gray-500 font-bold uppercase text-[9px]">Legend:</span>
                <div className="flex items-center gap-1.5">
                  <span className="w-2.5 h-2.5 rounded-full bg-orange-500" />
                  <span>Projects</span>
                </div>
                <div className="flex items-center gap-1.5">
                  <span className="w-2.5 h-2.5 rounded-full bg-blue-500" />
                  <span>MCUs</span>
                </div>
                <div className="flex items-center gap-1.5">
                  <span className="w-2.5 h-2.5 rounded-full bg-emerald-500" />
                  <span>Firmware</span>
                </div>
                <div className="flex items-center gap-1.5">
                  <span className="w-2.5 h-2.5 rounded-full bg-purple-500" />
                  <span>Hardware ICs</span>
                </div>
                <div className="flex items-center gap-1.5">
                  <span className="w-2.5 h-2.5 rounded-full bg-amber-500" />
                  <span>Protocols</span>
                </div>
                <div className="flex items-center gap-1.5">
                  <span className="w-2.5 h-2.5 rounded-full bg-pink-500" />
                  <span>LoRA Adapters</span>
                </div>
              </div>

              {/* Status Hint */}
              <div className="absolute top-3 right-3 bg-[#0e1017]/85 border border-[#232530] rounded-lg px-2.5 py-1 text-[10px] mono text-gray-400 flex items-center gap-2">
                <span>Drag nodes · Pan canvas · Scroll to zoom</span>
              </div>
            </div>
          )}

          {/* VIEW: 2. PROJECT DEPENDENCY MATRIX */}
          {viewMode === 'matrix' && (
            <div className="bg-[#111217] border border-[#232530] rounded-2xl p-4 shadow-xl overflow-x-auto">
              <table className="w-full text-left text-xs font-mono border-collapse">
                <thead>
                  <tr className="border-b border-[#232530] text-gray-400 text-[10px] uppercase">
                    <th className="p-3">Project</th>
                    <th className="p-3">MCU Target</th>
                    <th className="p-3">Firmware Stack</th>
                    <th className="p-3">Hardware ICs</th>
                    <th className="p-3">Communication Bus</th>
                    <th className="p-3">Trained LoRA</th>
                    <th className="p-3">DRC Status</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-[#1e202d] text-[11px]">
                  {nodes
                    .filter((n) => n.category === 'project')
                    .map((p) => {
                      const pLinks = links.filter((l) => l.source === p.id);
                      const targetIds = pLinks.map((l) => l.target);
                      const targets = nodes.filter((n) => targetIds.includes(n.id));

                      const mcus = targets.filter((n) => n.category === 'mcu');
                      const fws = targets.filter((n) => n.category === 'firmware');
                      const hws = targets.filter((n) => n.category === 'hardware');
                      const protos = targets.filter((n) => n.category === 'protocol');
                      const loras = targets.filter((n) => n.category === 'lora');

                      return (
                        <tr
                          key={p.id}
                          onClick={() => setSelectedNodeId(p.id)}
                          className={`hover:bg-[#181a26] transition cursor-pointer ${
                            selectedNodeId === p.id ? 'bg-orange-500/10 font-semibold' : ''
                          }`}
                        >
                          <td className="p-3">
                            <div className="font-bold text-white flex items-center gap-2">
                              <Boxes className="w-3.5 h-3.5 text-orange-400" />
                              <span>{p.label}</span>
                            </div>
                            <div className="text-[10px] text-gray-400">{p.subLabel}</div>
                          </td>
                          <td className="p-3 text-blue-400">
                            {mcus.map((m) => m.label.split(' ')[0]).join(', ') || '—'}
                          </td>
                          <td className="p-3 text-emerald-400">
                            {fws.map((f) => f.label.split(' ')[0]).join(', ') || '—'}
                          </td>
                          <td className="p-3 text-purple-400">
                            {hws.map((h) => h.label.split(' ')[1] || h.label).join(', ') || '—'}
                          </td>
                          <td className="p-3 text-amber-400">
                            {protos.map((pr) => pr.label.split(' ')[0]).join(', ') || '—'}
                          </td>
                          <td className="p-3 text-pink-400">
                            {loras.map((lo) => lo.label.replace('LoRA: ', '')).join(', ') || '—'}
                          </td>
                          <td className="p-3">
                            <span className="px-2 py-0.5 rounded bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 font-bold text-[10px]">
                              {p.drcScore}% PASS
                            </span>
                          </td>
                        </tr>
                      );
                    })}
                </tbody>
              </table>
            </div>
          )}

          {/* VIEW: 3. HIERARCHY TREE VIEW */}
          {viewMode === 'tree' && (
            <div className="bg-[#111217] border border-[#232530] rounded-2xl p-5 shadow-xl space-y-4 max-h-[580px] overflow-y-auto">
              <div className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2">
                <GitBranch className="w-4 h-4 text-orange-400" />
                <span>Hierarchical Project Decomposition</span>
              </div>

              <div className="space-y-3">
                {nodes
                  .filter((n) => n.category === 'project')
                  .map((p) => {
                    const pLinks = links.filter((l) => l.source === p.id);
                    const targets = nodes.filter((n) => pLinks.some((l) => l.target === n.id));

                    return (
                      <div
                        key={p.id}
                        onClick={() => setSelectedNodeId(p.id)}
                        className={`p-4 rounded-xl border transition cursor-pointer ${
                          selectedNodeId === p.id
                            ? 'bg-orange-500/10 border-orange-500/40 shadow-lg'
                            : 'bg-[#14151e] border-[#232530] hover:border-gray-600'
                        }`}
                      >
                        <div className="flex items-center justify-between pb-2 border-b border-[#232530]">
                          <div className="flex items-center gap-2.5">
                            <Boxes className="w-4 h-4 text-orange-400" />
                            <span className="text-sm font-bold text-white">{p.label}</span>
                            <span className="text-[10px] mono text-gray-400">{p.subLabel}</span>
                          </div>
                          <span className="text-[10px] mono px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 font-bold">
                            DRC {p.drcScore}%
                          </span>
                        </div>

                        <div className="grid grid-cols-2 sm:grid-cols-4 gap-2 mt-3 text-[10px] mono">
                          {targets.map((t) => (
                            <div
                              key={t.id}
                              className="p-2 rounded-lg bg-[#0b0c10] border border-[#232530] flex items-center gap-2 truncate"
                            >
                              {getNodeIcon(t.category)}
                              <div className="truncate">
                                <div className="text-gray-400 uppercase text-[8px]">{t.category}</div>
                                <div className="text-white font-semibold truncate">{t.label}</div>
                              </div>
                            </div>
                          ))}
                        </div>
                      </div>
                    );
                  })}
              </div>
            </div>
          )}

          {/* VIEW: 4. CROSS-PROJECT REUSE INSIGHTS */}
          {viewMode === 'insights' && (
            <div className="bg-[#111217] border border-[#232530] rounded-2xl p-5 shadow-xl space-y-4">
              <div className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2">
                <Activity className="w-4 h-4 text-orange-400" />
                <span>Architecture Component Reuse & Synergies</span>
              </div>

              <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
                <div className="p-4 rounded-xl bg-[#14151e] border border-[#232530]">
                  <div className="text-[10px] mono text-gray-400 uppercase">Shared HAL Crate Reuse</div>
                  <div className="text-xl font-bold mono text-emerald-400 mt-1">78.5%</div>
                  <div className="text-[10px] text-gray-400 mt-1">Across 8 embedded projects</div>
                </div>

                <div className="p-4 rounded-xl bg-[#14151e] border border-[#232530]">
                  <div className="text-[10px] mono text-gray-400 uppercase">Avg KiCad DRC Pass Rate</div>
                  <div className="text-xl font-bold mono text-orange-400 mt-1">99.1%</div>
                  <div className="text-[10px] text-gray-400 mt-1">0 fatal clearance errors</div>
                </div>

                <div className="p-4 rounded-xl bg-[#14151e] border border-[#232530]">
                  <div className="text-[10px] mono text-gray-400 uppercase">AST Compaction Savings</div>
                  <div className="text-xl font-bold mono text-amber-400 mt-1">76.2%</div>
                  <div className="text-[10px] text-gray-400 mt-1">Tree-Sitter prompt prune</div>
                </div>
              </div>

              <div className="p-4 rounded-xl bg-[#14151e] border border-[#232530] space-y-3">
                <div className="text-xs font-bold text-white uppercase tracking-wider">Top Shared Building Blocks</div>
                <div className="space-y-2 text-[11px] mono">
                  <div className="flex items-center justify-between p-2.5 rounded-lg bg-[#0b0c10] border border-[#232530]">
                    <div className="flex items-center gap-2">
                      <Code2 className="w-3.5 h-3.5 text-emerald-400" />
                      <span className="text-white font-bold">Embassy Async Executor (Rust)</span>
                    </div>
                    <span className="text-orange-400">Used by 4 Projects (50%)</span>
                  </div>

                  <div className="flex items-center justify-between p-2.5 rounded-lg bg-[#0b0c10] border border-[#232530]">
                    <div className="flex items-center gap-2">
                      <Zap className="w-3.5 h-3.5 text-pink-400" />
                      <span className="text-white font-bold">LoRA KiCad DRC Specialist</span>
                    </div>
                    <span className="text-orange-400">Assists 5 Schematics (62.5%)</span>
                  </div>

                  <div className="flex items-center justify-between p-2.5 rounded-lg bg-[#0b0c10] border border-[#232530]">
                    <div className="flex items-center gap-2">
                      <Radio className="w-3.5 h-3.5 text-amber-400" />
                      <span className="text-white font-bold">gRPC Protobuf CAD Bridge (:50051)</span>
                    </div>
                    <span className="text-orange-400">Integrated in Studio & Retimer</span>
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Deep Node Dossier / Inspector Sidebar (Right 4 Cols) */}
        <div className="lg:col-span-4 bg-[#111217] border border-[#232530] rounded-2xl p-5 shadow-xl flex flex-col justify-between space-y-4">
          <div>
            {/* Dossier Header */}
            <div className="flex items-center justify-between pb-3 border-b border-[#232530]">
              <div className="flex items-center gap-2">
                <div
                  className="w-7 h-7 rounded-lg flex items-center justify-center"
                  style={{ backgroundColor: `${selectedNode.color}20` }}
                >
                  {getNodeIcon(selectedNode.category)}
                </div>
                <div>
                  <div className="text-xs font-bold text-white truncate max-w-[180px]">
                    {selectedNode.label}
                  </div>
                  <div className="text-[10px] mono text-gray-400">ID: {selectedNode.id}</div>
                </div>
              </div>
              {getCategoryBadge(selectedNode.category)}
            </div>

            {/* Subtitle / Role */}
            {selectedNode.subLabel && (
              <div className="mt-3 text-[11px] mono text-orange-400 font-semibold flex items-center gap-1.5">
                <Sparkles className="w-3 h-3 shrink-0" />
                <span>{selectedNode.subLabel}</span>
              </div>
            )}

            {/* Description */}
            <p className="mt-2 text-xs text-gray-300 leading-relaxed bg-[#14151e] border border-[#262838] p-3 rounded-xl">
              {selectedNode.description}
            </p>

            {/* Key Technical Metrics */}
            {selectedNode.metrics && (
              <div className="mt-3 space-y-1.5 text-[11px] mono">
                <div className="text-[10px] uppercase font-bold text-gray-400 tracking-wider">Specifications</div>
                <div className="grid grid-cols-2 gap-2">
                  {selectedNode.metrics.frequency && (
                    <div className="p-2 rounded-lg bg-[#0b0c10] border border-[#232530]">
                      <span className="text-[9px] text-gray-400 block">Frequency:</span>
                      <span className="text-white font-bold">{selectedNode.metrics.frequency}</span>
                    </div>
                  )}
                  {selectedNode.metrics.voltage && (
                    <div className="p-2 rounded-lg bg-[#0b0c10] border border-[#232530]">
                      <span className="text-[9px] text-gray-400 block">Power / Voltage:</span>
                      <span className="text-emerald-400 font-bold">{selectedNode.metrics.voltage}</span>
                    </div>
                  )}
                  {selectedNode.metrics.busSpeed && (
                    <div className="p-2 rounded-lg bg-[#0b0c10] border border-[#232530]">
                      <span className="text-[9px] text-gray-400 block">Bus Throughput:</span>
                      <span className="text-amber-400 font-bold">{selectedNode.metrics.busSpeed}</span>
                    </div>
                  )}
                  {selectedNode.metrics.passRate && (
                    <div className="p-2 rounded-lg bg-[#0b0c10] border border-[#232530]">
                      <span className="text-[9px] text-gray-400 block">Pass Accuracy:</span>
                      <span className="text-orange-400 font-bold">{selectedNode.metrics.passRate}</span>
                    </div>
                  )}
                </div>
              </div>
            )}

            {/* Tags */}
            {selectedNode.tags && selectedNode.tags.length > 0 && (
              <div className="mt-3">
                <div className="text-[10px] uppercase font-bold text-gray-400 tracking-wider mb-1.5">Architecture Tags</div>
                <div className="flex flex-wrap gap-1.5">
                  {selectedNode.tags.map((tag, idx) => (
                    <span
                      key={idx}
                      className="px-2 py-0.5 rounded-md bg-[#181a26] text-gray-300 border border-[#282a3a] text-[10px] mono"
                    >
                      #{tag}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {/* Relational Neighborhood Links */}
            <div className="mt-4 pt-3 border-t border-[#232530]">
              <div className="text-[10px] uppercase font-bold text-gray-400 tracking-wider mb-2 flex items-center justify-between">
                <span>Direct Graph Connections ({selectedNodeLinks.length})</span>
                <span className="text-orange-400 mono">{connectedNodeIds.size - 1} Neighbors</span>
              </div>

              <div className="space-y-1.5 max-h-40 overflow-y-auto pr-1">
                {selectedNodeLinks.map((link, idx) => {
                  const targetId = link.source === selectedNode.id ? link.target : link.source;
                  const targetNode = nodes.find((n) => n.id === targetId);
                  if (!targetNode) return null;

                  return (
                    <button
                      key={idx}
                      onClick={() => setSelectedNodeId(targetNode.id)}
                      className="w-full p-2 rounded-lg bg-[#14151e] hover:bg-[#1f212e] border border-[#232530] hover:border-orange-500/40 text-left transition flex items-center justify-between text-[11px] mono cursor-pointer group"
                    >
                      <div className="flex items-center gap-2 min-w-0">
                        {getNodeIcon(targetNode.category)}
                        <span className="text-gray-200 group-hover:text-white font-medium truncate">
                          {targetNode.label}
                        </span>
                      </div>
                      <span className="text-[9px] text-gray-500 group-hover:text-orange-400 uppercase shrink-0">
                        {link.label}
                      </span>
                    </button>
                  );
                })}
              </div>
            </div>
          </div>

          {/* Quick Actions Footer */}
          <div className="pt-3 border-t border-[#232530] space-y-2">
            <div className="flex items-center justify-between text-[10px] mono text-gray-400">
              <span>Status:</span>
              <span className="text-emerald-400 font-bold uppercase flex items-center gap-1">
                <CheckCircle2 className="w-3 h-3" />
                {selectedNode.status || 'Active in Workspace'}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
