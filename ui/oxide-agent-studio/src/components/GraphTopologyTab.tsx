import React, { useState } from 'react';
import {
  Network,
  GitBranch,
  Layers,
  Activity,
  AlertTriangle,
  Scissors,
  Share2,
  Database,
  Terminal,
  Filter,
  CheckCircle2,
  ChevronRight,
  Sparkles,
  Zap,
} from 'lucide-react';

interface CodeNodeItem {
  id: string;
  name: string;
  type: 'Function' | 'Struct' | 'Trait' | 'Module' | 'Variable';
  file: string;
  span: string;
  signature: string;
  vectorId: string;
  callers: string[];
  callees: string[];
  downstreamImpact: number;
}

const SAMPLE_GRAPH_NODES: CodeNodeItem[] = [
  {
    id: 'fn_handle_login',
    name: 'handle_login',
    type: 'Function',
    file: 'workspace/gateway/src/routes.rs',
    span: 'L45-L78',
    signature: 'pub async fn handle_login(req: Json<LoginReq>) -> HttpResponse',
    vectorId: 'vec-auth-091',
    callers: ['api_router'],
    callees: ['verify_password', 'fetch_user_by_id'],
    downstreamImpact: 4,
  },
  {
    id: 'fn_verify_password',
    name: 'verify_password',
    type: 'Function',
    file: 'crates/auth/src/service.rs',
    span: 'L112-L138',
    signature: 'pub fn verify_password(hash: &str, raw: &str) -> bool',
    vectorId: 'vec-auth-092',
    callers: ['handle_login', 'cli_auth_helper'],
    callees: ['argon2_verify'],
    downstreamImpact: 2,
  },
  {
    id: 'struct_surreal_client',
    name: 'SurrealClient',
    type: 'Struct',
    file: 'workspace/memory/src/lib.rs',
    span: 'L18-L62',
    signature: 'pub struct SurrealClient { db: Arc<Surreal<Db>> }',
    vectorId: 'vec-db-001',
    callers: ['fetch_user_by_id', 'save_chat_turn', 'query_ast_graph'],
    callees: [],
    downstreamImpact: 14,
  },
  {
    id: 'fn_qemu_verify',
    name: 'verify_firmware_qemu',
    type: 'Function',
    file: 'workspace/verifier/src/qemu_firmware.rs',
    span: 'L34-L89',
    signature: 'pub async fn verify_firmware_qemu(elf: &Path, arch: &str) -> Result<bool>',
    vectorId: 'vec-verif-412',
    callers: ['autonomous_supervisor', 'test_rklipper_firmware'],
    callees: ['execute_in_sandbox'],
    downstreamImpact: 3,
  },
  {
    id: 'struct_task_dag',
    name: 'TaskDag',
    type: 'Struct',
    file: 'workspace/router/src/supervisor.rs',
    span: 'L22-L75',
    signature: 'pub struct TaskDag { pub nodes: HashMap<String, TaskNode> }',
    vectorId: 'vec-dag-108',
    callers: ['SupervisorAgent', 'plan_goal'],
    callees: ['is_ready', 'detect_oscillation'],
    downstreamImpact: 6,
  },
];

export const GraphTopologyTab: React.FC = () => {
  const [selectedNodeId, setSelectedNodeId] = useState<string>('struct_surreal_client');
  const [modalityFilter, setModalityFilter] = useState<'all' | 'ast' | 'calls' | 'deps' | 'data'>('all');
  const [activeHopDepth, setActiveHopDepth] = useState<number>(2);

  const selectedNode = SAMPLE_GRAPH_NODES.find((n) => n.id === selectedNodeId) || SAMPLE_GRAPH_NODES[0];

  return (
    <div className="space-y-6">
      {/* Top Banner: Graph Engineering Layer Status */}
      <div className="bg-gradient-to-r from-orange-950/40 via-[#141721] to-[#0f1117] border border-orange-500/20 rounded-xl p-6 shadow-xl relative overflow-hidden backdrop-blur-md">
        <div className="absolute top-0 right-0 w-96 h-96 bg-orange-500/5 rounded-full blur-3xl pointer-events-none" />
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4 relative z-10">
          <div>
            <div className="flex items-center gap-3">
              <div className="p-2.5 bg-orange-500/10 border border-orange-500/30 rounded-lg text-orange-400">
                <Network className="w-6 h-6 animate-pulse" />
              </div>
              <div>
                <h1 className="text-2xl font-bold text-white tracking-tight flex items-center gap-2">
                  Graph Engineering Layer
                  <span className="text-xs px-2.5 py-0.5 rounded-full bg-orange-500/20 text-orange-300 border border-orange-500/30">
                    Structural Skeleton Active
                  </span>
                </h1>
                <p className="text-sm text-gray-400 mt-1">
                  SurrealDB v3 multi-modal code graph, semantic-structural hybrid indexing, impact propagation, and subgraph context pruning.
                </p>
              </div>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <div className="px-3.5 py-2 rounded-lg bg-black/40 border border-gray-800 text-center">
              <div className="text-xs text-gray-500 font-mono">AST NODES</div>
              <div className="text-lg font-bold text-orange-400 font-mono">1,482</div>
            </div>
            <div className="px-3.5 py-2 rounded-lg bg-black/40 border border-gray-800 text-center">
              <div className="text-xs text-gray-500 font-mono">DIRECTED EDGES</div>
              <div className="text-lg font-bold text-emerald-400 font-mono">3,940</div>
            </div>
            <div className="px-3.5 py-2 rounded-lg bg-black/40 border border-gray-800 text-center">
              <div className="text-xs text-gray-500 font-mono">TOKEN SAVINGS</div>
              <div className="text-lg font-bold text-cyan-400 font-mono">76.4%</div>
            </div>
          </div>
        </div>
      </div>

      {/* Main 2-Column Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
        {/* Left Column: Multi-Modal Code Graph & Symbol Explorer */}
        <div className="lg:col-span-7 space-y-6">
          <div className="bg-[#12141c] border border-gray-800 rounded-xl p-5 shadow-lg">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center gap-2">
                <Layers className="w-5 h-5 text-orange-400" />
                <h2 className="text-base font-semibold text-white">Multi-Modal Code Graph</h2>
              </div>
              <div className="flex items-center gap-1 bg-black/40 p-1 rounded-lg border border-gray-800 text-xs">
                {(['all', 'ast', 'calls', 'deps', 'data'] as const).map((mode) => (
                  <button
                    key={mode}
                    onClick={() => setModalityFilter(mode)}
                    className={`px-2.5 py-1 rounded capitalize font-medium transition-all ${
                      modalityFilter === mode
                        ? 'bg-orange-500 text-white shadow'
                        : 'text-gray-400 hover:text-white'
                    }`}
                  >
                    {mode === 'all' ? 'All Modalities' : mode}
                  </button>
                ))}
              </div>
            </div>

            {/* Interactive Graph Node List */}
            <div className="space-y-2.5">
              {SAMPLE_GRAPH_NODES.map((node) => {
                const isSelected = node.id === selectedNodeId;
                return (
                  <div
                    key={node.id}
                    onClick={() => setSelectedNodeId(node.id)}
                    className={`p-3.5 rounded-lg border transition-all cursor-pointer ${
                      isSelected
                        ? 'bg-orange-500/10 border-orange-500/50 shadow-md ring-1 ring-orange-500/30'
                        : 'bg-[#181a24] border-gray-800/80 hover:border-gray-700'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2.5">
                        <span
                          className={`text-xs px-2 py-0.5 rounded font-mono font-semibold ${
                            node.type === 'Function'
                              ? 'bg-blue-500/20 text-blue-300 border border-blue-500/30'
                              : 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
                          }`}
                        >
                          {node.type}
                        </span>
                        <span className="text-sm font-semibold text-white font-mono">{node.name}</span>
                      </div>
                      <div className="flex items-center gap-3">
                        <span className="text-xs text-gray-500 font-mono">{node.span}</span>
                        <span
                          className={`text-xs px-2 py-0.5 rounded-full font-mono ${
                            node.downstreamImpact > 5
                              ? 'bg-red-500/20 text-red-300 border border-red-500/30'
                              : 'bg-yellow-500/20 text-yellow-300 border border-yellow-500/30'
                          }`}
                        >
                          Blast Radius: {node.downstreamImpact}
                        </span>
                      </div>
                    </div>

                    <div className="text-xs text-gray-400 font-mono mt-2 truncate bg-black/30 px-2 py-1 rounded">
                      {node.signature}
                    </div>

                    <div className="flex items-center justify-between text-xs text-gray-500 mt-2">
                      <span className="truncate">{node.file}</span>
                      <span className="font-mono text-cyan-400">Qdrant: {node.vectorId}</span>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>

          {/* Subgraph Pruner & Context Reduction Box */}
          <div className="bg-[#12141c] border border-gray-800 rounded-xl p-5 shadow-lg">
            <div className="flex items-center justify-between mb-3">
              <div className="flex items-center gap-2">
                <Scissors className="w-5 h-5 text-cyan-400" />
                <h3 className="text-sm font-semibold text-white">Graph-Based Context Pruning</h3>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-xs text-gray-400 font-mono">Hop Depth:</span>
                {[1, 2, 3].map((depth) => (
                  <button
                    key={depth}
                    onClick={() => setActiveHopDepth(depth)}
                    className={`px-2 py-0.5 rounded text-xs font-mono font-semibold transition ${
                      activeHopDepth === depth
                        ? 'bg-cyan-500 text-black'
                        : 'bg-black/40 text-gray-400 hover:text-white border border-gray-800'
                    }`}
                  >
                    {depth}-Hop
                  </button>
                ))}
              </div>
            </div>

            <div className="grid grid-cols-2 gap-3 p-3 bg-black/40 rounded-lg border border-gray-800/80 mb-3 font-mono text-xs">
              <div>
                <div className="text-gray-500">RAW FILE TOKEN DUMP</div>
                <div className="text-base font-bold text-red-400 mt-0.5">3,850 tokens</div>
                <div className="text-[11px] text-gray-500">Full multi-file contents</div>
              </div>
              <div>
                <div className="text-gray-500">PRUNED SUBGRAPH</div>
                <div className="text-base font-bold text-cyan-400 mt-0.5">420 tokens (-89.1%)</div>
                <div className="text-[11px] text-emerald-400">100% typing & caller fidelity</div>
              </div>
            </div>

            <div className="text-xs text-gray-400 bg-[#161822] p-3 rounded border border-gray-800 font-mono whitespace-pre-wrap leading-relaxed max-h-36 overflow-y-auto">
              {`### Topology-Aware Subgraph Context\n**Target**: \`${selectedNode.name}\` (${selectedNode.type})\n- Callers: ${selectedNode.callers.join(', ') || 'none'}\n- Callees: ${selectedNode.callees.join(', ') || 'none'}\n- File: ${selectedNode.file}`}
            </div>
          </div>
        </div>

        {/* Right Column: Impact Analysis & Change Propagation */}
        <div className="lg:col-span-5 space-y-6">
          <div className="bg-[#12141c] border border-gray-800 rounded-xl p-5 shadow-lg">
            <div className="flex items-center gap-2 mb-4">
              <AlertTriangle className="w-5 h-5 text-amber-400" />
              <h2 className="text-base font-semibold text-white">Impact Analysis & Propagation</h2>
            </div>

            <div className="p-4 rounded-lg bg-[#181a24] border border-gray-800 space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-xs text-gray-400 uppercase tracking-wider font-mono">Target Symbol</span>
                <span className="text-xs font-mono font-bold text-orange-400">{selectedNode.name}</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-xs text-gray-400 uppercase tracking-wider font-mono">Risk Level</span>
                <span
                  className={`text-xs px-2.5 py-0.5 rounded-full font-mono font-bold ${
                    selectedNode.downstreamImpact > 10
                      ? 'bg-red-500/20 text-red-400 border border-red-500/40'
                      : selectedNode.downstreamImpact > 3
                      ? 'bg-amber-500/20 text-amber-400 border border-amber-500/40'
                      : 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/40'
                  }`}
                >
                  {selectedNode.downstreamImpact > 10 ? 'HIGH RISK' : selectedNode.downstreamImpact > 3 ? 'MEDIUM RISK' : 'LOW RISK'}
                </span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-xs text-gray-400 uppercase tracking-wider font-mono">Blast Radius</span>
                <span className="text-sm font-mono font-bold text-white">{selectedNode.downstreamImpact} downstream nodes</span>
              </div>
            </div>

            {/* Predictive Test Selection */}
            <div className="mt-5">
              <h3 className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2 font-mono flex items-center gap-1.5">
                <Zap className="w-3.5 h-3.5 text-orange-400" />
                Predictive Test Selection
              </h3>
              <div className="space-y-2">
                <div className="flex items-center justify-between p-2.5 bg-black/40 rounded border border-gray-800 text-xs font-mono">
                  <span className="text-gray-300 truncate">cargo test -p memory --test surreal_test</span>
                  <span className="text-emerald-400 text-[11px] font-semibold">Affected</span>
                </div>
                <div className="flex items-center justify-between p-2.5 bg-black/40 rounded border border-gray-800 text-xs font-mono">
                  <span className="text-gray-300 truncate">cargo test -p knowledge --test code_graph_test</span>
                  <span className="text-emerald-400 text-[11px] font-semibold">Affected</span>
                </div>
                <div className="flex items-center justify-between p-2.5 bg-black/40 rounded border border-gray-800 text-xs font-mono">
                  <span className="text-gray-300 truncate">cargo test -p gateway --test routes_test</span>
                  <span className="text-gray-500 text-[11px]">Clean</span>
                </div>
              </div>
            </div>

            {/* Topological Execution Path */}
            <div className="mt-5">
              <h3 className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2 font-mono flex items-center gap-1.5">
                <GitBranch className="w-3.5 h-3.5 text-cyan-400" />
                Call Path Traversal
              </h3>
              <div className="p-3 bg-black/40 rounded border border-gray-800 text-xs font-mono space-y-1.5 text-gray-300">
                <div className="text-blue-400">1. API Entry: api_router (routes.rs)</div>
                <div className="pl-3 text-cyan-300">↳ 2. Calls: handle_login</div>
                <div className="pl-6 text-orange-300">↳ 3. Calls: verify_password & fetch_user</div>
                <div className="pl-9 text-emerald-400 font-bold">↳ 4. Target: SurrealClient::query</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
