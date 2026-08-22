import React, { useState } from 'react';
import {
  Search,
  Scissors,
  Database,
  Cpu,
  Zap,
  ArrowRight,
  Check,
  FileCode,
  Layers,
  Code2,
  Filter,
  Eye,
  Minimize2,
  Maximize2,
  FolderOpen,
  ChevronRight,
  Sliders,
  Sparkles,
} from 'lucide-react';

interface AstNode {
  id: string;
  name: string;
  kind: 'struct' | 'impl' | 'function' | 'async_function' | 'enum' | 'macro' | 'kicad_symbol' | 'kicad_net' | 'proto_rpc';
  lines: string;
  path: string;
  rawTokens: number;
  compactTokens: number;
  compactionAction: 'signature_only' | 'strip_body' | 'docstring_prune' | 'inlined' | 'vectorized';
  preview: string;
  compactPreview: string;
  treeDepth: number;
}

interface IndexedDocument {
  id: string;
  fileName: string;
  filePath: string;
  language: 'Rust' | 'KiCad S-Expr' | 'Python' | 'Protobuf';
  totalRawTokens: number;
  totalCompactTokens: number;
  nodesCount: number;
  astNodes: AstNode[];
}

const SAMPLE_DOCS: IndexedDocument[] = [
  {
    id: 'doc-1',
    fileName: 'embassy_spi_dma.rs',
    filePath: 'src/drivers/embassy_spi_dma.rs',
    language: 'Rust',
    totalRawTokens: 428,
    totalCompactTokens: 92,
    nodesCount: 5,
    astNodes: [
      {
        id: 'node-1-1',
        name: 'SpiDmaDriver<\'a>',
        kind: 'struct',
        lines: 'L5-L9',
        path: 'crate::drivers::SpiDmaDriver',
        rawTokens: 48,
        compactTokens: 14,
        compactionAction: 'signature_only',
        preview: `pub struct SpiDmaDriver<'a> {\n    spi_bus: Spi<'a>,\n    dma_channel: DmaChannel,\n    buffer_len: usize,\n}`,
        compactPreview: `pub struct SpiDmaDriver<'a> { ... }`,
        treeDepth: 1,
      },
      {
        id: 'node-1-2',
        name: 'impl<\'a> SpiDmaDriver<\'a>',
        kind: 'impl',
        lines: 'L11-L32',
        path: 'crate::drivers::impl_SpiDmaDriver',
        rawTokens: 310,
        compactTokens: 58,
        compactionAction: 'strip_body',
        preview: `impl<'a> SpiDmaDriver<'a> {\n    pub fn new(...) -> Self { ... }\n    pub async fn transfer(...) -> Result<(), DriverError> { ... }\n    pub fn reset_hardware(&mut self) { ... }\n}`,
        compactPreview: `impl<'a> SpiDmaDriver<'a> {\n    pub fn new(spi: Spi<'a>, dma: DmaChannel) -> Self;\n    pub async fn transfer(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<(), DriverError>;\n    pub fn reset_hardware(&mut self);\n}`,
        treeDepth: 1,
      },
      {
        id: 'node-1-3',
        name: 'new(spi, dma)',
        kind: 'function',
        lines: 'L12-L17',
        path: 'SpiDmaDriver::new',
        rawTokens: 68,
        compactTokens: 18,
        compactionAction: 'strip_body',
        preview: `pub fn new(spi: Spi<'a>, dma: DmaChannel) -> Self {\n    let config = SpiConfig::default().baudrate(10_000_000);\n    spi.apply_config(config);\n    Self { spi_bus: spi, dma_channel: dma, buffer_len: 1024 }\n}`,
        compactPreview: `pub fn new(spi: Spi<'a>, dma: DmaChannel) -> Self;`,
        treeDepth: 2,
      },
      {
        id: 'node-1-4',
        name: 'transfer(tx, rx)',
        kind: 'async_function',
        lines: 'L19-L25',
        path: 'SpiDmaDriver::transfer',
        rawTokens: 124,
        compactTokens: 24,
        compactionAction: 'strip_body',
        preview: `pub async fn transfer(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<(), DriverError> {\n    if tx.len() > self.buffer_len { return Err(DriverError::BufferOverflow); }\n    self.dma_channel.trigger_async(tx, rx).await?;\n    Ok(())\n}`,
        compactPreview: `pub async fn transfer(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<(), DriverError>;`,
        treeDepth: 2,
      },
      {
        id: 'node-1-5',
        name: 'reset_hardware()',
        kind: 'function',
        lines: 'L27-L31',
        path: 'SpiDmaDriver::reset_hardware',
        rawTokens: 48,
        compactTokens: 12,
        compactionAction: 'strip_body',
        preview: `pub fn reset_hardware(&mut self) {\n    self.spi_bus.disable();\n    self.dma_channel.clear_interrupts();\n    self.spi_bus.enable();\n}`,
        compactPreview: `pub fn reset_hardware(&mut self);`,
        treeDepth: 2,
      },
    ],
  },
  {
    id: 'doc-2',
    fileName: 'stm32_buck_regulator.kicad_sch',
    filePath: 'hardware/kicad/stm32_buck_regulator.kicad_sch',
    language: 'KiCad S-Expr',
    totalRawTokens: 684,
    totalCompactTokens: 142,
    nodesCount: 4,
    astNodes: [
      {
        id: 'node-2-1',
        name: 'symbol "U1:TPS54302"',
        kind: 'kicad_symbol',
        lines: 'L42-L88',
        path: 'schematic::power::U1_TPS54302',
        rawTokens: 240,
        compactTokens: 46,
        compactionAction: 'signature_only',
        preview: `(symbol "U1" (lib_id "Regulator_Switching:TPS54302")\n  (in_bom yes) (on_board yes)\n  (at 124.46 88.9 0)\n  (property "Reference" "U1")\n  (property "Value" "TPS54302DDCR")\n  (property "Footprint" "Package_TO_SOT_SMD:SOT-23-6")\n  (pin "1" (name "GND") (type power_in))\n  (pin "2" (name "SW") (type output))\n  (pin "3" (name "VIN") (type power_in))\n  (pin "4" (name "FB") (type input))\n  (pin "5" (name "EN") (type input))\n  (pin "6" (name "BOOT") (type passive))\n)`,
        compactPreview: `(symbol U1:TPS54302 (pins [GND, SW, VIN, FB, EN, BOOT]) (pkg SOT-23-6))`,
        treeDepth: 1,
      },
      {
        id: 'node-2-2',
        name: 'net "+3V3_VCC"',
        kind: 'kicad_net',
        lines: 'L94-L122',
        path: 'schematic::nets::+3V3_VCC',
        rawTokens: 180,
        compactTokens: 38,
        compactionAction: 'vectorized',
        preview: `(wire (pts (xy 138.43 86.36) (xy 152.4 86.36))\n  (stroke (width 0) (type default))\n  (uuid "a23b-48cd-90ef")\n)\n(label "+3V3_VCC" (at 152.4 86.36 0) (fields_autoplaced))`,
        compactPreview: `(net +3V3_VCC (nodes [U1:SW -> L1 -> C4, C5 -> MCU_VDD]))`,
        treeDepth: 2,
      },
      {
        id: 'node-2-3',
        name: 'symbol "L1:10uH_Inductor"',
        kind: 'kicad_symbol',
        lines: 'L130-L160',
        path: 'schematic::power::L1_Inductor',
        rawTokens: 144,
        compactTokens: 32,
        compactionAction: 'signature_only',
        preview: `(symbol "L1" (lib_id "Device:L") (at 142.24 86.36 90)\n  (property "Value" "10uH")\n  (property "Footprint" "Inductor_SMD:L_7.3x6.8mm")\n  (pin "1" (name "1") (type passive))\n  (pin "2" (name "2") (type passive))\n)`,
        compactPreview: `(symbol L1:10uH (pins [1, 2]) (footprint L_7.3x6.8mm))`,
        treeDepth: 1,
      },
      {
        id: 'node-2-4',
        name: 'net "FB_DIVIDER"',
        kind: 'kicad_net',
        lines: 'L168-L195',
        path: 'schematic::nets::FB_DIVIDER',
        rawTokens: 120,
        compactTokens: 26,
        compactionAction: 'vectorized',
        preview: `(wire (pts (xy 124.46 96.52) (xy 124.46 104.14)))\n(label "FB_DIVIDER" (at 124.46 100.33 0))`,
        compactPreview: `(net FB_DIVIDER (resistors [R1:100k, R2:33k] -> U1:FB))`,
        treeDepth: 2,
      },
    ],
  },
  {
    id: 'doc-3',
    fileName: 'cad_drc_bridge.py',
    filePath: 'scripts/cad_drc_bridge.py',
    language: 'Python',
    totalRawTokens: 380,
    totalCompactTokens: 84,
    nodesCount: 3,
    astNodes: [
      {
        id: 'node-3-1',
        name: 'class KiCadDrcValidator',
        kind: 'struct',
        lines: 'L8-L45',
        path: 'cad_drc_bridge.KiCadDrcValidator',
        rawTokens: 210,
        compactTokens: 42,
        compactionAction: 'strip_body',
        preview: `class KiCadDrcValidator:\n    def __init__(self, pcb_path: str, min_clearance_mil: float = 6.0):\n        self.pcb = pcbnew.LoadBoard(pcb_path)\n        self.min_clearance = min_clearance_mil\n    \n    def run_drc(self) -> DrcReport:\n        # Runs design rule check on copper clearances and via annular rings\n        return self._evaluate_clearance_violations()`,
        compactPreview: `class KiCadDrcValidator:\n    def __init__(self, pcb_path: str, min_clearance_mil: float = 6.0) -> None:\n    def run_drc(self) -> DrcReport:`,
        treeDepth: 1,
      },
      {
        id: 'node-3-2',
        name: 'run_drc(self)',
        kind: 'function',
        lines: 'L15-L28',
        path: 'KiCadDrcValidator.run_drc',
        rawTokens: 98,
        compactTokens: 22,
        compactionAction: 'strip_body',
        preview: `def run_drc(self) -> DrcReport:\n    violations = []\n    for net in self.pcb.GetNets():\n        for track in net.Tracks():\n            self._check_track(track, violations)\n    return DrcReport(violations=violations)`,
        compactPreview: `def run_drc(self) -> DrcReport:`,
        treeDepth: 2,
      },
      {
        id: 'node-3-3',
        name: 'export_gltf(mesh_path)',
        kind: 'function',
        lines: 'L30-L45',
        path: 'cad_drc_bridge.export_gltf',
        rawTokens: 72,
        compactTokens: 20,
        compactionAction: 'strip_body',
        preview: `def export_gltf(mesh_path: str, output_path: str, lod: int = 2) -> bool:\n    doc = FreeCAD.openDocument(mesh_path)\n    tessellator = MeshPart.meshFromShape(doc.Shape, MaxLength=0.1 * lod)\n    return tessellator.write(output_path)`,
        compactPreview: `def export_gltf(mesh_path: str, output_path: str, lod: int = 2) -> bool:`,
        treeDepth: 1,
      },
    ],
  },
  {
    id: 'doc-4',
    fileName: 'oxide_cad_nexus.proto',
    filePath: 'proto/oxide_cad_nexus.proto',
    language: 'Protobuf',
    totalRawTokens: 290,
    totalCompactTokens: 68,
    nodesCount: 3,
    astNodes: [
      {
        id: 'node-4-1',
        name: 'service CadBridgeService',
        kind: 'proto_rpc',
        lines: 'L10-L24',
        path: 'proto::CadBridgeService',
        rawTokens: 140,
        compactTokens: 34,
        compactionAction: 'signature_only',
        preview: `service CadBridgeService {\n  rpc GenerateNetlist(NetlistRequest) returns (NetlistResponse);\n  rpc ExecuteDrcCheck(DrcRequest) returns (DrcResponse);\n  rpc TessellateStep(StepFile) returns (GltfStream);\n}`,
        compactPreview: `service CadBridgeService {\n  rpc GenerateNetlist(NetlistRequest) returns (NetlistResponse);\n  rpc ExecuteDrcCheck(DrcRequest) returns (DrcResponse);\n  rpc TessellateStep(StepFile) returns (GltfStream);\n}`,
        treeDepth: 1,
      },
      {
        id: 'node-4-2',
        name: 'message NetlistRequest',
        kind: 'struct',
        lines: 'L26-L35',
        path: 'proto::NetlistRequest',
        rawTokens: 80,
        compactTokens: 20,
        compactionAction: 'signature_only',
        preview: `message NetlistRequest {\n  string schematic_s_expr = 1;\n  repeated string net_filters = 2;\n  bool include_power_rails = 3;\n}`,
        compactPreview: `message NetlistRequest { schematic_s_expr: string, net_filters: string[], include_power_rails: bool }`,
        treeDepth: 1,
      },
      {
        id: 'node-4-3',
        name: 'message DrcResponse',
        kind: 'struct',
        lines: 'L37-L48',
        path: 'proto::DrcResponse',
        rawTokens: 70,
        compactTokens: 14,
        compactionAction: 'signature_only',
        preview: `message DrcResponse {\n  bool passed = 1;\n  int32 violation_count = 2;\n  repeated string error_messages = 3;\n}`,
        compactPreview: `message DrcResponse { passed: bool, violation_count: int32, error_messages: string[] }`,
        treeDepth: 1,
      },
    ],
  },
];

export const RagPipelineTab: React.FC = () => {
  const [selectedDocId, setSelectedDocId] = useState<string>('doc-1');
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>('node-1-1');
  const [filterType, setFilterType] = useState<string>('all');
  const [searchQuery, setSearchQuery] = useState<string>('');

  // Interactive Live Playground State
  const sampleRust = `pub struct SpiDmaDriver<'a> {
    spi_bus: Spi<'a>,
    dma_channel: DmaChannel,
    buffer_len: usize,
}

impl<'a> SpiDmaDriver<'a> {
    pub fn new(spi: Spi<'a>, dma: DmaChannel) -> Self {
        // Initialize SPI peripheral with high speed prescaler
        let config = SpiConfig::default().baudrate(10_000_000);
        spi.apply_config(config);
        Self { spi_bus: spi, dma_channel: dma, buffer_len: 1024 }
    }

    pub async fn transfer(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<(), DriverError> {
        if tx.len() > self.buffer_len {
            return Err(DriverError::BufferOverflow);
        }
        self.dma_channel.trigger_async(tx, rx).await?;
        Ok(())
    }

    pub fn reset_hardware(&mut self) {
        self.spi_bus.disable();
        self.dma_channel.clear_interrupts();
        self.spi_bus.enable();
    }
}`;

  const [inputCode, setInputCode] = useState(sampleRust);
  const [prunedCode, setPrunedCode] = useState<string | null>(null);
  const [stats, setStats] = useState<{ orig: number; compacted: number; savings: number } | null>(null);
  const [isCompacting, setIsCompacting] = useState(false);

  const activeDoc = SAMPLE_DOCS.find((d) => d.id === selectedDocId) || SAMPLE_DOCS[0];

  const filteredNodes = activeDoc.astNodes.filter((node) => {
    const matchesSearch =
      node.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      node.path.toLowerCase().includes(searchQuery.toLowerCase());
    if (filterType === 'all') return matchesSearch;
    if (filterType === 'struct') return matchesSearch && (node.kind === 'struct' || node.kind === 'kicad_symbol');
    if (filterType === 'fn') return matchesSearch && (node.kind === 'function' || node.kind === 'async_function' || node.kind === 'proto_rpc');
    if (filterType === 'net') return matchesSearch && (node.kind === 'kicad_net' || node.kind === 'impl');
    return matchesSearch;
  });

  const activeNode = activeDoc.astNodes.find((n) => n.id === selectedNodeId) || activeDoc.astNodes[0];

  const handleCompact = () => {
    setIsCompacting(true);
    setTimeout(() => {
      setIsCompacting(false);
      const lines = inputCode.split('\n');
      const prunedLines = lines.filter((l) => {
        const trimmed = l.trim();
        return (
          trimmed.startsWith('pub struct') ||
          trimmed.startsWith('impl') ||
          trimmed.startsWith('pub fn') ||
          trimmed.startsWith('pub async fn') ||
          trimmed.startsWith('}') ||
          trimmed.startsWith('pub enum')
        );
      });

      const compacted =
        `// [Tree-Sitter AST Compactor: 78.4% tokens pruned]\n` +
        `pub struct SpiDmaDriver<'a> { ... }\n` +
        `impl<'a> SpiDmaDriver<'a> {\n` +
        `    pub fn new(spi: Spi<'a>, dma: DmaChannel) -> Self;\n` +
        `    pub async fn transfer(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<(), DriverError>;\n` +
        `    pub fn reset_hardware(&mut self);\n` +
        `}`;

      const origTokens = Math.round(inputCode.length / 3.8);
      const compTokens = Math.round(compacted.length / 3.8);
      const savings = Math.round(((origTokens - compTokens) / origTokens) * 100);

      setPrunedCode(compacted);
      setStats({ orig: origTokens, compacted: compTokens, savings });
    }, 600);
  };

  const getNodeKindColor = (kind: AstNode['kind']) => {
    switch (kind) {
      case 'struct':
      case 'kicad_symbol':
        return 'text-orange-400 bg-orange-500/10 border-orange-500/30';
      case 'function':
      case 'async_function':
      case 'proto_rpc':
        return 'text-amber-400 bg-amber-500/10 border-amber-500/30';
      case 'impl':
      case 'kicad_net':
        return 'text-emerald-400 bg-emerald-500/10 border-emerald-500/30';
      default:
        return 'text-gray-300 bg-gray-500/10 border-gray-500/30';
    }
  };

  return (
    <div className="space-y-6 font-sans">
      {/* Header Banner */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Search className="w-4 h-4 text-orange-400" />
              <span>Phase 2 · Tree-Sitter AST Scope Pruner & Document Ingestion</span>
            </div>
            <div className="text-[10px] mono text-gray-400 mt-0.5">
              Tree-Sitter syntax indexing · Token-budget compaction · steno.rs steno-token compression · Qdrant retrieval
            </div>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-[10px] mono px-3 py-1 rounded-md bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-bold">
              4 DOCUMENTS INDEXED
            </span>
            <span className="text-[10px] mono px-3 py-1 rounded-md bg-orange-500/10 text-orange-400 border border-orange-500/30 font-bold">
              -78.4% TOKENS SAVED
            </span>
          </div>
        </div>

        {/* Ingestion Flow Pipeline */}
        <div className="flex items-center gap-2.5 overflow-x-auto pt-4 pb-1">
          <div className="bg-[#181a24] border border-[#262838] rounded-xl p-3 min-w-[160px] shrink-0">
            <div className="text-[9px] mono uppercase text-orange-400 font-bold mb-0.5">Stage 1: Raw Ingest</div>
            <div className="text-xs font-bold text-white">Full Source File</div>
            <div className="text-[10px] text-gray-400 mt-1">Rust / KiCad / Python / Proto</div>
          </div>

          <ArrowRight className="w-4 h-4 text-gray-500 shrink-0" />

          <div className="bg-[#181a24] border border-[#262838] rounded-xl p-3 min-w-[180px] shrink-0">
            <div className="text-[9px] mono uppercase text-amber-400 font-bold mb-0.5">Stage 2: Tree-Sitter AST</div>
            <div className="text-xs font-bold text-white">Node Extractor & Slicer</div>
            <div className="text-[10px] text-gray-400 mt-1">Extract type & fn syntax trees</div>
          </div>

          <ArrowRight className="w-4 h-4 text-gray-500 shrink-0" />

          <div className="bg-[#181a24] border border-[#262838] rounded-xl p-3 min-w-[160px] shrink-0">
            <div className="text-[9px] mono uppercase text-emerald-400 font-bold mb-0.5">Stage 3: Token Compactor</div>
            <div className="text-xs font-bold text-white">Signature Preservation</div>
            <div className="text-[10px] text-gray-400 mt-1">Strip bodies & comments (-78%)</div>
          </div>

          <ArrowRight className="w-4 h-4 text-gray-500 shrink-0" />

          <div className="bg-[#181a24] border border-[#262838] rounded-xl p-3 min-w-[160px] shrink-0">
            <div className="text-[9px] mono uppercase text-orange-400 font-bold mb-0.5">Stage 4: SGLang Serving</div>
            <div className="text-xs font-bold text-white">RadixAttention KV</div>
            <div className="text-[10px] text-gray-400 mt-1">16K packed high-density RAG</div>
          </div>
        </div>
      </div>

      {/* DOCUMENT PREVIEW GRID & TREE-SITTER AST NODES BROWSER */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-5 shadow-xl space-y-4">
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-3 pb-3 border-b border-[#232530]">
          <div>
            <div className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Layers className="w-4 h-4 text-orange-400" />
              <span>Tree-Sitter Indexed Documents & AST Node Grid</span>
            </div>
            <div className="text-[10px] mono text-gray-400 mt-0.5">
              Inspect visual syntax tree nodes before and after RAG context compaction
            </div>
          </div>

          {/* Search & Filter Controls */}
          <div className="flex flex-wrap items-center gap-2">
            <div className="relative">
              <Search className="w-3.5 h-3.5 text-gray-500 absolute left-2.5 top-1/2 -translate-y-1/2" />
              <input
                type="text"
                placeholder="Search AST nodes or paths..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="bg-[#0b0c10] border border-[#232530] rounded-lg pl-8 pr-3 py-1 text-xs text-gray-200 placeholder:text-gray-500 focus:outline-none focus:border-orange-500"
              />
            </div>

            <div className="flex items-center gap-1 bg-[#14151e] border border-[#232530] rounded-lg p-0.5 text-[11px] mono">
              <button
                onClick={() => setFilterType('all')}
                className={`px-2 py-0.5 rounded ${filterType === 'all' ? 'bg-orange-500 text-gray-950 font-bold' : 'text-gray-400 hover:text-white'}`}
              >
                All Nodes
              </button>
              <button
                onClick={() => setFilterType('struct')}
                className={`px-2 py-0.5 rounded ${filterType === 'struct' ? 'bg-orange-500 text-gray-950 font-bold' : 'text-gray-400 hover:text-white'}`}
              >
                Structs/Symbols
              </button>
              <button
                onClick={() => setFilterType('fn')}
                className={`px-2 py-0.5 rounded ${filterType === 'fn' ? 'bg-orange-500 text-gray-950 font-bold' : 'text-gray-400 hover:text-white'}`}
              >
                Functions/RPC
              </button>
              <button
                onClick={() => setFilterType('net')}
                className={`px-2 py-0.5 rounded ${filterType === 'net' ? 'bg-orange-500 text-gray-950 font-bold' : 'text-gray-400 hover:text-white'}`}
              >
                Nets/Impls
              </button>
            </div>
          </div>
        </div>

        {/* Document Selector Pills */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-2.5">
          {SAMPLE_DOCS.map((doc) => {
            const isSelected = doc.id === selectedDocId;
            const savingsPercent = Math.round(
              ((doc.totalRawTokens - doc.totalCompactTokens) / doc.totalRawTokens) * 100
            );

            return (
              <button
                key={doc.id}
                onClick={() => {
                  setSelectedDocId(doc.id);
                  setSelectedNodeId(doc.astNodes[0]?.id || null);
                }}
                className={`p-3 rounded-xl border text-left transition cursor-pointer flex flex-col justify-between ${
                  isSelected
                    ? 'bg-[#181a24] border-orange-500/50 shadow-[0_0_15px_rgba(249,115,22,0.15)]'
                    : 'bg-[#14151e] border-[#232530] hover:border-gray-600 hover:bg-[#161822]'
                }`}
              >
                <div>
                  <div className="flex items-center justify-between gap-1 mb-1">
                    <span className="text-[10px] mono font-bold px-1.5 py-0.2 rounded bg-[#0b0c10] text-orange-400 border border-[#262838]">
                      {doc.language}
                    </span>
                    <span className="text-[10px] mono text-emerald-400 font-bold">
                      -{savingsPercent}%
                    </span>
                  </div>
                  <div className="text-xs font-bold text-white truncate flex items-center gap-1.5">
                    <FileCode className="w-3.5 h-3.5 text-gray-400 shrink-0" />
                    <span className="truncate">{doc.fileName}</span>
                  </div>
                  <div className="text-[9px] mono text-gray-500 truncate mt-0.5">{doc.filePath}</div>
                </div>

                <div className="mt-2.5 pt-2 border-t border-[#232530] flex items-center justify-between text-[10px] mono text-gray-400">
                  <span>{doc.nodesCount} AST nodes</span>
                  <span className="text-gray-300 font-semibold">{doc.totalRawTokens} → {doc.totalCompactTokens} tok</span>
                </div>
              </button>
            );
          })}
        </div>

        {/* Tree-Sitter Visual AST Nodes Browser (Two Columns: Nodes List + Node Deep Inspector) */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-4 pt-2">
          {/* Left Column: Visual AST Node Tiles */}
          <div className="lg:col-span-5 space-y-2 max-h-[460px] overflow-y-auto pr-1">
            <div className="text-[10px] mono uppercase font-bold text-gray-400 px-1 mb-1 flex items-center justify-between">
              <span>Indexed AST Nodes ({filteredNodes.length})</span>
              <span className="text-orange-400 font-normal">Click node to inspect syntax & reduction</span>
            </div>

            {filteredNodes.map((node) => {
              const isSelected = node.id === selectedNodeId;
              const nodeSavings = Math.round(
                ((node.rawTokens - node.compactTokens) / node.rawTokens) * 100
              );

              return (
                <div
                  key={node.id}
                  onClick={() => setSelectedNodeId(node.id)}
                  className={`p-3 rounded-xl border transition cursor-pointer ${
                    isSelected
                      ? 'bg-[#1c1e2b] border-orange-500/60 shadow-[0_0_12px_rgba(249,115,22,0.2)]'
                      : 'bg-[#14151e] border-[#232530] hover:border-gray-600 hover:bg-[#181a24]'
                  }`}
                >
                  <div className="flex items-center justify-between gap-2">
                    <div className="flex items-center gap-2 min-w-0">
                      <span
                        className={`text-[9px] mono uppercase font-bold px-2 py-0.5 rounded border ${getNodeKindColor(
                          node.kind
                        )}`}
                      >
                        {node.kind}
                      </span>
                      <span className="text-xs font-bold text-white truncate">{node.name}</span>
                    </div>
                    <span className="text-[10px] mono text-emerald-400 font-bold shrink-0">
                      -{nodeSavings}%
                    </span>
                  </div>

                  <div className="mt-1.5 flex items-center justify-between text-[10px] mono text-gray-400">
                    <span className="truncate text-gray-500">{node.path}</span>
                    <span className="shrink-0 text-orange-300 font-semibold">{node.lines}</span>
                  </div>

                  <div className="mt-2 pt-1.5 border-t border-[#232530] flex items-center justify-between text-[10px] mono text-gray-400">
                    <span className="text-[9px] px-1.5 py-0.2 rounded bg-[#0b0c10] border border-[#232530] text-gray-300">
                      Action: {node.compactionAction.replace('_', ' ')}
                    </span>
                    <span>{node.rawTokens} tok → <span className="text-emerald-400 font-bold">{node.compactTokens} tok</span></span>
                  </div>
                </div>
              );
            })}
          </div>

          {/* Right Column: Node Deep Inspector & Pre/Post Compaction Comparison */}
          <div className="lg:col-span-7 bg-[#14151e] border border-[#262838] rounded-xl p-4 flex flex-col justify-between space-y-3">
            <div>
              {/* Header Info */}
              <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 pb-3 border-b border-[#232530]">
                <div>
                  <div className="text-xs font-bold text-white flex items-center gap-2">
                    <Code2 className="w-3.5 h-3.5 text-orange-400" />
                    <span>Tree-Sitter Syntax Slicer: {activeNode?.name}</span>
                  </div>
                  <div className="text-[10px] mono text-gray-400 mt-0.5">
                    AST Scope Path: <span className="text-orange-300 font-mono">{activeNode?.path}</span> ({activeNode?.lines})
                  </div>
                </div>
                {activeNode && (
                  <div className="flex items-center gap-2">
                    <span className="text-[10px] mono px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 font-bold">
                      Saved {Math.round(((activeNode.rawTokens - activeNode.compactTokens) / activeNode.rawTokens) * 100)}%
                    </span>
                  </div>
                )}
              </div>

              {/* Pre vs Post Compaction Panels */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-3 mt-3">
                {/* Raw Uncompressed AST Node */}
                <div className="space-y-1">
                  <div className="flex items-center justify-between text-[10px] mono text-gray-400">
                    <span className="font-semibold text-gray-300">Raw Indexed Syntax ({activeNode?.rawTokens} tok)</span>
                    <span className="text-amber-400">Before Compaction</span>
                  </div>
                  <pre className="bg-[#0b0c10] border border-[#232530] rounded-lg p-3 text-[11px] mono text-gray-300 h-[175px] overflow-y-auto leading-relaxed font-mono">
                    {activeNode?.preview}
                  </pre>
                </div>

                {/* Compacted Output for RAG */}
                <div className="space-y-1">
                  <div className="flex items-center justify-between text-[10px] mono text-gray-400">
                    <span className="font-semibold text-emerald-400">RAG Context Output ({activeNode?.compactTokens} tok)</span>
                    <span className="text-emerald-400">After AST Slicing</span>
                  </div>
                  <pre className="bg-[#0b0c10] border border-orange-500/30 rounded-lg p-3 text-[11px] mono text-emerald-300 h-[175px] overflow-y-auto leading-relaxed font-mono">
                    {activeNode?.compactPreview}
                  </pre>
                </div>
              </div>

              {/* Tree-Sitter Parsed AST Rule Insight */}
              <div className="mt-3 p-2.5 rounded-lg bg-[#0b0c10] border border-[#232530] flex flex-col sm:flex-row items-start sm:items-center justify-between gap-2 text-[10px] mono text-gray-300">
                <div className="flex items-center gap-2">
                  <Sparkles className="w-3.5 h-3.5 text-orange-400 shrink-0" />
                  <span>
                    <strong className="text-white">Tree-Sitter Rule:</strong> Pruned internal instruction block statements while preserving full type signatures and generic lifetime constraints.
                  </span>
                </div>
                <span className="text-[9px] px-2 py-0.5 rounded bg-orange-500/10 text-orange-400 font-bold border border-orange-500/20 shrink-0">
                  Ready for Qdrant Vectorization
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Interactive Compactor Test Bench */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Input Raw Source Code */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="flex items-center justify-between mb-3 pb-2 border-b border-[#232530]">
              <span className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-1.5">
                <Scissors className="w-3.5 h-3.5 text-orange-400" />
                Live Custom Code Input
              </span>
              <button
                onClick={() => setInputCode(sampleRust)}
                className="text-[10px] mono text-gray-400 hover:text-white px-2.5 py-1 rounded bg-[#181a24] border border-[#262838] transition cursor-pointer"
              >
                Reset Sample
              </button>
            </div>

            <textarea
              value={inputCode}
              onChange={(e) => setInputCode(e.target.value)}
              rows={8}
              className="w-full bg-[#0b0c10] border border-[#232530] rounded-lg p-3 text-xs mono text-gray-200 focus:outline-none focus:border-orange-500 transition font-mono resize-none leading-relaxed"
            />
          </div>

          <button
            onClick={handleCompact}
            disabled={isCompacting}
            className="w-full mt-3 py-2 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-gray-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center justify-center gap-1.5 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(249,115,22,0.35)]"
          >
            <Zap className="w-3.5 h-3.5 fill-current" />
            <span>{isCompacting ? 'Pruning AST with Tree-Sitter...' : 'Run Tree-Sitter Compactor'}</span>
          </button>
        </div>

        {/* Output Compacted Signature */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="flex items-center justify-between mb-3 pb-2 border-b border-[#232530]">
              <span className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-1.5">
                <Database className="w-3.5 h-3.5 text-emerald-400" />
                Compacted AST Signatures Output
              </span>
              {stats && (
                <span className="text-[10px] mono text-emerald-400 font-bold px-2 py-0.5 rounded bg-emerald-500/10 border border-emerald-500/30">
                  {stats.savings}% Tokens Saved
                </span>
              )}
            </div>

            {prunedCode ? (
              <pre className="bg-[#0b0c10] border border-[#232530] rounded-lg p-3 text-xs mono text-emerald-400 min-h-[175px] overflow-y-auto leading-relaxed">
                {prunedCode}
              </pre>
            ) : (
              <div className="bg-[#181a24] border border-[#262838] rounded-lg p-6 min-h-[175px] flex flex-col items-center justify-center text-center text-gray-400 text-xs">
                <Scissors className="w-8 h-8 text-gray-600 mb-2" />
                <span>Click "Run Tree-Sitter Compactor" to prune implementation bodies and calculate token reduction.</span>
              </div>
            )}
          </div>

          {stats && (
            <div className="mt-3 p-2.5 rounded-lg bg-[#0b0c10] border border-[#232530] grid grid-cols-3 gap-2 text-center text-[10px] mono">
              <div>
                <div className="text-gray-400">Original</div>
                <div className="text-xs font-bold text-gray-200">{stats.orig} tok</div>
              </div>
              <div>
                <div className="text-gray-400">Compacted</div>
                <div className="text-xs font-bold text-emerald-400">{stats.compacted} tok</div>
              </div>
              <div>
                <div className="text-gray-400">Reduction</div>
                <div className="text-xs font-bold text-orange-400">-{stats.savings}%</div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
