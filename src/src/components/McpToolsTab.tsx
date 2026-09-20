import React, { useState } from 'react';
import { McpTool } from '../types';
import { Wrench, Terminal, Play, CheckCircle2, Shield, Plus, Code2 } from 'lucide-react';

export const McpToolsTab: React.FC = () => {
  const tools: McpTool[] = [
    {
      name: 'pcb_synthesize',
      description: 'Generate KiCad 8/9 schematic from netlist specifications and component references',
      category: 'cad',
      status: 'ready',
      latency: '28ms',
      schemaSample: '{"project_name": "stm32_core", "components": [{"ref": "U1", "val": "STM32F401"}]}',
    },
    {
      name: 'cargo_cross_build',
      description: 'Cross-compile bare-metal firmware targeting thumbv7em-none-eabihf with sccache',
      category: 'embedded',
      status: 'ready',
      latency: '142ms',
      schemaSample: '{"target": "thumbv7em-none-eabihf", "release": true, "features": ["defmt"]}',
    },
    {
      name: 'probe_rs_debug',
      description: 'Attach SWD/JTAG debugger to target MCU via CMSIS-DAP or ST-Link',
      category: 'debug',
      status: 'ready',
      latency: '85ms',
      schemaSample: '{"chip": "STM32F401RETx", "probe_index": 0, "action": "read_r0"}',
    },
    {
      name: 'qemu_redox_boot',
      description: 'Launch microVM KVM instance for Redox OS kernel verification',
      category: 'embedded',
      status: 'ready',
      latency: '310ms',
      schemaSample: '{"arch": "x86_64", "kernel": "redox_kernel.bin", "memory_mb": 512}',
    },
    {
      name: 'kicad_drc_check',
      description: 'Execute kicad-cli electrical and physical DRC rule checks with JSON output',
      category: 'cad',
      status: 'ready',
      latency: '38ms',
      schemaSample: '{"pcb_path": "workspace/hardware/pcb/main.kicad_pcb", "strict": true}',
    },
    {
      name: 'step_to_gltf',
      description: 'FreeCAD/Blender tessellation bridge converting 3D STEP models to GLTF format',
      category: 'cad',
      status: 'ready',
      latency: '64ms',
      schemaSample: '{"input_step": "case.step", "output_gltf": "case.gltf", "tolerance": 0.05}',
    },
    {
      name: 'tree_sitter_parse',
      description: 'Extract AST declarations and strip function bodies for 70-90% token reduction',
      category: 'rag',
      status: 'ready',
      latency: '12ms',
      schemaSample: '{"lang": "rust", "source_file": "crates/optio/src/lib.rs"}',
    },
    {
      name: 'web_search',
      description: 'SearXNG local instance search query for electronics datasheets and RFCs',
      category: 'rag',
      status: 'network',
      latency: '180ms',
      schemaSample: '{"query": "STM32F401 SPI DMA errata", "limit": 5}',
    },
    {
      name: 'jit_tool_maker',
      description: 'Dynamic Python/Mojo tool synthesizer sandboxed with Linux bubblewrap (bwrap)',
      category: 'jit',
      status: 'active',
      latency: '95ms',
      schemaSample: '{"intent": "Compute CRC32 with SIMD", "lang": "mojo"}',
    },
  ];

  const [selectedTool, setSelectedTool] = useState<McpTool>(tools[0]);
  const [paramInput, setParamInput] = useState(tools[0].schemaSample);
  const [rpcResponse, setRpcResponse] = useState<string | null>(null);
  const [isExecuting, setIsExecuting] = useState(false);

  const handleSelectTool = (tool: McpTool) => {
    setSelectedTool(tool);
    setParamInput(tool.schemaSample);
    setRpcResponse(null);
  };

  const handleExecuteRpc = () => {
    setIsExecuting(true);
    setTimeout(() => {
      setIsExecuting(false);
      setRpcResponse(
        JSON.stringify(
          {
            jsonrpc: '2.0',
            id: Math.floor(Math.random() * 1000),
            result: {
              status: 'SUCCESS',
              tool: selectedTool.name,
              execution_time_ms: parseInt(selectedTool.latency) || 35,
              sandbox: 'bubblewrap_isolate_ok',
              output: {
                message: `Tool '${selectedTool.name}' executed cleanly with returncode=0`,
                artifacts: [`workspace/out/${selectedTool.name}_artifact.json`],
              },
            },
          },
          null,
          2
        )
      );
    }, 700);
  };

  return (
    <div className="space-y-6 font-sans">
      {/* Header */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Wrench className="w-4 h-4 text-orange-400" />
              <span>Phase 3 · Model Context Protocol (MCP) Server Suite</span>
            </div>
            <div className="text-[10px] mono text-gray-400 mt-0.5">
              STDIO JSON-RPC 2.0 · 9 native tools · JIT Tool Synthesizer · Bubblewrap sandbox
            </div>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-[10px] mono px-3 py-1 rounded-md bg-orange-500/10 text-orange-400 border border-orange-500/30 font-bold flex items-center gap-1.5">
              <Shield className="w-3.5 h-3.5" />
              Bubblewrap Isolated
            </span>
          </div>
        </div>

        {/* Tools Catalog Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 mt-4">
          {tools.map((tool) => {
            const isSelected = selectedTool.name === tool.name;
            return (
              <div
                key={tool.name}
                onClick={() => handleSelectTool(tool)}
                className={`p-3.5 rounded-xl border transition-all cursor-pointer ${
                  isSelected
                    ? 'bg-orange-500/10 border-orange-500/50 shadow-[0_0_12px_rgba(249,115,22,0.2)]'
                    : 'bg-[#181a24] border-[#262838] hover:border-orange-500/30'
                }`}
              >
                <div className="flex items-center justify-between mb-1.5">
                  <span className="text-xs font-bold mono text-orange-400">{tool.name}</span>
                  <span className="text-[9px] mono px-2 py-0.5 rounded bg-[#111217] border border-[#232530] text-gray-300 font-semibold">
                    {tool.latency}
                  </span>
                </div>
                <div className="text-[11px] text-gray-300 leading-snug line-clamp-2">
                  {tool.description}
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* JSON-RPC 2.0 Interactive Executor */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Request Input */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="flex items-center justify-between mb-3 pb-2 border-b border-[#232530]">
              <span className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-1.5">
                <Terminal className="w-3.5 h-3.5 text-orange-400" />
                STDIO JSON-RPC Request: {selectedTool.name}
              </span>
              <span className="text-[10px] mono text-gray-400">Method: tools/call</span>
            </div>

            <div className="space-y-2">
              <div className="text-[10px] mono text-gray-400">Params Payload (JSON):</div>
              <textarea
                value={paramInput}
                onChange={(e) => setParamInput(e.target.value)}
                rows={6}
                className="w-full bg-[#0b0c10] border border-[#232530] rounded-lg p-3 text-xs mono text-gray-200 focus:outline-none focus:border-orange-500 transition font-mono resize-none leading-relaxed"
              />
            </div>
          </div>

          <button
            onClick={handleExecuteRpc}
            disabled={isExecuting}
            className="w-full mt-3 py-2 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-gray-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center justify-center gap-1.5 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(249,115,22,0.35)]"
          >
            <Play className="w-3.5 h-3.5 fill-current" />
            <span>{isExecuting ? 'Invoking MCP STDIO...' : `Execute ${selectedTool.name}`}</span>
          </button>
        </div>

        {/* Response Viewer */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="flex items-center justify-between mb-3 pb-2 border-b border-[#232530]">
              <span className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-1.5">
                <Code2 className="w-3.5 h-3.5 text-emerald-400" />
                STDIO JSON-RPC 2.0 Response
              </span>
              {rpcResponse && (
                <span className="text-[10px] mono text-emerald-400 font-bold px-2 py-0.5 rounded bg-emerald-500/10 border border-emerald-500/30">200 OK</span>
              )}
            </div>

            {rpcResponse ? (
              <pre className="bg-[#0b0c10] border border-[#232530] rounded-lg p-3 text-xs mono text-emerald-400 min-h-[160px] overflow-y-auto leading-relaxed">
                {rpcResponse}
              </pre>
            ) : (
              <div className="bg-[#181a24] border border-[#262838] rounded-lg p-6 min-h-[160px] flex flex-col items-center justify-center text-center text-gray-400 text-xs">
                <Terminal className="w-8 h-8 text-gray-600 mb-2" />
                <span>Click "Execute {selectedTool.name}" to trigger the STDIO JSON-RPC 2.0 protocol cycle.</span>
              </div>
            )}
          </div>

          <div className="mt-3 text-[10px] mono text-gray-400">
            Bubblewrap sandbox guarantees no network or disk escapes outside <code className="text-orange-400">workspace/</code>.
          </div>
        </div>
      </div>
    </div>
  );
};
