import React, { useState, useEffect } from 'react';
import {
  Wrench,
  Plus,
  Play,
  Save,
  Trash2,
  CheckCircle2,
  XCircle,
  Clock,
  Code2,
  FileCode,
  Layers,
  Sparkles,
  RefreshCw,
  Search,
} from 'lucide-react';
import { skillList, skillSave, skillTest, skillDelete } from '../lib/desktop';
import { SkillDto, SkillTestResultDto } from '../types';

export const SkillsTab: React.FC = () => {
  const [skills, setSkills] = useState<SkillDto[]>([]);
  const [selectedSkill, setSelectedSkill] = useState<SkillDto | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(false);
  const [testInput, setTestInput] = useState('{\n  "workspace_path": ".",\n  "strict_mode": true\n}');
  const [testResult, setTestResult] = useState<SkillTestResultDto | null>(null);
  const [testing, setTesting] = useState(false);
  const [statusMsg, setStatusMsg] = useState<string | null>(null);

  const fetchSkills = async () => {
    setLoading(true);
    try {
      const list = await skillList();
      setSkills(list);
      if (list.length > 0 && !selectedSkill) {
        setSelectedSkill(list[0]);
      }
    } catch (err) {
      console.error('Failed to load skills', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchSkills();
  }, []);

  const handleCreateNew = () => {
    const newSkill: SkillDto = {
      name: `custom_skill_${Date.now().toString().slice(-4)}`,
      title: 'New Custom Skill',
      description: 'Performs verified domain task in WASM sandbox',
      version: '0.1.0',
      runner_type: 'wasm',
      parameters: [
        {
          name: 'target_param',
          type: 'string',
          description: 'Primary input argument',
          required: true,
        },
      ],
      return_type: 'json',
      code_or_schema: JSON.stringify(
        {
          type: 'object',
          properties: {
            target_param: { type: 'string', description: 'Primary input argument' },
          },
          required: ['target_param'],
        },
        null,
        2
      ),
      is_builtin: false,
    };
    setSkills((prev) => [newSkill, ...prev]);
    setSelectedSkill(newSkill);
  };

  const handleSave = async () => {
    if (!selectedSkill) return;
    setStatusMsg('Saving skill...');
    try {
      await skillSave(selectedSkill);
      setStatusMsg('Skill saved successfully to .agents/skills/');
      await fetchSkills();
      setTimeout(() => setStatusMsg(null), 3000);
    } catch (err: unknown) {
      const error = err as Error;
      setStatusMsg(`Error saving skill: ${error.message || 'unknown'}`);
    }
  };

  const handleDelete = async (name: string) => {
    if (!confirm(`Delete skill '${name}'?`)) return;
    try {
      await skillDelete(name);
      await fetchSkills();
      if (selectedSkill?.name === name) {
        setSelectedSkill(null);
      }
    } catch (err) {
      console.error('Failed to delete skill', err);
    }
  };

  const handleRunTest = async () => {
    if (!selectedSkill) return;
    setTesting(true);
    setTestResult(null);
    try {
      const res = await skillTest(selectedSkill.name, testInput);
      setTestResult(res);
    } catch (err: unknown) {
      const error = err as Error;
      setTestResult({
        success: false,
        output: '',
        latency_ms: 0,
        schema_valid: false,
        error: error.message || 'Execution failed',
      });
    } finally {
      setTesting(false);
    }
  };

  const filteredSkills = skills.filter(
    (s) =>
      s.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      s.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      s.description.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="flex flex-col gap-6 max-w-7xl mx-auto pb-12">
      {/* Header */}
      <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 bg-[#111113] p-5 rounded-xl border border-[#27272A]">
        <div className="flex items-center gap-3.5">
          <div className="w-10 h-10 rounded-lg bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400">
            <Wrench className="w-5 h-5" />
          </div>
          <div>
            <h1 className="text-lg font-semibold text-zinc-100 flex items-center gap-2">
              Skills Studio & Tool Sandbox
              <span className="text-[11px] px-2 py-0.5 rounded bg-indigo-500/20 text-indigo-300 font-mono border border-indigo-500/30">
                Wasmtime 49 WASI 0.2
              </span>
            </h1>
            <p className="text-xs text-zinc-400">
              Create, test, and sandbox MCP skills with JSON-Schema verification and WASM isolation.
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2.5">
          <button
            onClick={fetchSkills}
            className="px-3 py-1.5 rounded-lg bg-[#18181B] border border-[#27272A] hover:bg-[#27272A] text-xs font-medium text-zinc-300 flex items-center gap-1.5 transition-colors"
          >
            <RefreshCw className={`w-3.5 h-3.5 ${loading ? 'animate-spin' : ''}`} />
            Reload
          </button>
          <button
            onClick={handleCreateNew}
            className="px-3.5 py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-xs font-semibold text-white flex items-center gap-1.5 transition-colors shadow-sm"
          >
            <Plus className="w-3.5 h-3.5" />
            New Skill
          </button>
        </div>
      </div>

      {statusMsg && (
        <div className="px-4 py-2.5 rounded-lg bg-indigo-500/10 border border-indigo-500/30 text-xs text-indigo-300 flex items-center gap-2">
          <Sparkles className="w-4 h-4 text-indigo-400" />
          {statusMsg}
        </div>
      )}

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
        {/* Left Sidebar: Skill List */}
        <div className="lg:col-span-4 flex flex-col gap-3">
          <div className="relative">
            <Search className="w-4 h-4 absolute left-3 top-2.5 text-zinc-500" />
            <input
              type="text"
              placeholder="Search skills..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-9 pr-3 py-2 bg-[#111113] border border-[#27272A] rounded-lg text-xs text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-indigo-500/50"
            />
          </div>

          <div className="bg-[#111113] border border-[#27272A] rounded-xl overflow-hidden flex flex-col divide-y divide-[#1F1F23]">
            {filteredSkills.length === 0 ? (
              <div className="p-8 text-center text-xs text-zinc-500">No skills matching search</div>
            ) : (
              filteredSkills.map((s) => {
                const isSelected = selectedSkill?.name === s.name;
                return (
                  <button
                    key={s.name}
                    onClick={() => {
                      setSelectedSkill(s);
                      setTestResult(null);
                    }}
                    className={`p-3.5 text-left transition-colors flex items-start justify-between gap-3 ${
                      isSelected ? 'bg-indigo-950/30 border-l-2 border-indigo-500' : 'hover:bg-[#18181B]'
                    }`}
                  >
                    <div className="flex flex-col gap-1 min-w-0">
                      <div className="flex items-center gap-2">
                        <span className="text-xs font-semibold text-zinc-200 truncate">{s.title || s.name}</span>
                        {s.is_builtin && (
                          <span className="text-[10px] px-1.5 py-0.2 bg-zinc-800 text-zinc-400 rounded">Built-in</span>
                        )}
                      </div>
                      <p className="text-[11px] text-zinc-400 line-clamp-1">{s.description}</p>
                      <div className="flex items-center gap-2 mt-1">
                        <span className="text-[10px] font-mono text-zinc-500 bg-zinc-900 px-1.5 py-0.5 rounded border border-zinc-800">
                          {s.runner_type}
                        </span>
                        <span className="text-[10px] text-zinc-500">v{s.version}</span>
                      </div>
                    </div>

                    {!s.is_builtin && (
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          handleDelete(s.name);
                        }}
                        className="p-1 hover:text-red-400 text-zinc-600 rounded transition-colors"
                        title="Delete skill"
                      >
                        <Trash2 className="w-3.5 h-3.5" />
                      </button>
                    )}
                  </button>
                );
              })
            )}
          </div>
        </div>

        {/* Right Panel: Skill Editor & Test Sandbox */}
        <div className="lg:col-span-8 flex flex-col gap-6">
          {selectedSkill ? (
            <div className="flex flex-col gap-6">
              {/* Metadata Form */}
              <div className="bg-[#111113] border border-[#27272A] rounded-xl p-5 flex flex-col gap-4">
                <div className="flex items-center justify-between border-b border-[#27272A] pb-3.5">
                  <div className="flex items-center gap-2">
                    <FileCode className="w-4 h-4 text-indigo-400" />
                    <h2 className="text-sm font-semibold text-zinc-200">Skill Definition & JSON Schema</h2>
                  </div>
                  <button
                    onClick={handleSave}
                    className="px-3.5 py-1.5 bg-indigo-600 hover:bg-indigo-500 text-xs font-medium text-white rounded-lg flex items-center gap-1.5 transition-colors"
                  >
                    <Save className="w-3.5 h-3.5" />
                    Save Changes
                  </button>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <label className="block text-[11px] font-medium text-zinc-400 mb-1">Skill Identifier (snake_case)</label>
                    <input
                      type="text"
                      value={selectedSkill.name}
                      disabled={selectedSkill.is_builtin}
                      onChange={(e) => setSelectedSkill({ ...selectedSkill, name: e.target.value })}
                      className="w-full px-3 py-1.5 bg-[#0A0A0A] border border-[#27272A] rounded-lg text-xs font-mono text-zinc-200 focus:outline-none focus:border-indigo-500"
                    />
                  </div>

                  <div>
                    <label className="block text-[11px] font-medium text-zinc-400 mb-1">Display Title</label>
                    <input
                      type="text"
                      value={selectedSkill.title}
                      onChange={(e) => setSelectedSkill({ ...selectedSkill, title: e.target.value })}
                      className="w-full px-3 py-1.5 bg-[#0A0A0A] border border-[#27272A] rounded-lg text-xs text-zinc-200 focus:outline-none focus:border-indigo-500"
                    />
                  </div>

                  <div className="md:col-span-2">
                    <label className="block text-[11px] font-medium text-zinc-400 mb-1">Description</label>
                    <input
                      type="text"
                      value={selectedSkill.description}
                      onChange={(e) => setSelectedSkill({ ...selectedSkill, description: e.target.value })}
                      className="w-full px-3 py-1.5 bg-[#0A0A0A] border border-[#27272A] rounded-lg text-xs text-zinc-200 focus:outline-none focus:border-indigo-500"
                    />
                  </div>

                  <div>
                    <label className="block text-[11px] font-medium text-zinc-400 mb-1">Runner Isolation</label>
                    <select
                      value={selectedSkill.runner_type}
                      onChange={(e) => setSelectedSkill({ ...selectedSkill, runner_type: e.target.value })}
                      className="w-full px-3 py-1.5 bg-[#0A0A0A] border border-[#27272A] rounded-lg text-xs text-zinc-200 focus:outline-none focus:border-indigo-500"
                    >
                      <option value="wasm">Wasmtime 49 (WASI 0.2 Sandbox)</option>
                      <option value="rust_crate">Native Rust Crate</option>
                      <option value="python_bridge">Python Bridge / sidecar</option>
                      <option value="script">Shell / Command</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-[11px] font-medium text-zinc-400 mb-1">Version</label>
                    <input
                      type="text"
                      value={selectedSkill.version}
                      onChange={(e) => setSelectedSkill({ ...selectedSkill, version: e.target.value })}
                      className="w-full px-3 py-1.5 bg-[#0A0A0A] border border-[#27272A] rounded-lg text-xs font-mono text-zinc-200 focus:outline-none focus:border-indigo-500"
                    />
                  </div>
                </div>

                <div>
                  <label className="block text-[11px] font-medium text-zinc-400 mb-1">Schema / Implementation Logic</label>
                  <textarea
                    rows={6}
                    value={selectedSkill.code_or_schema}
                    onChange={(e) => setSelectedSkill({ ...selectedSkill, code_or_schema: e.target.value })}
                    className="w-full p-3 bg-[#0A0A0A] border border-[#27272A] rounded-lg font-mono text-xs text-zinc-300 focus:outline-none focus:border-indigo-500 leading-relaxed"
                  />
                </div>
              </div>

              {/* Sandbox Test Runner */}
              <div className="bg-[#111113] border border-[#27272A] rounded-xl p-5 flex flex-col gap-4">
                <div className="flex items-center justify-between border-b border-[#27272A] pb-3.5">
                  <div className="flex items-center gap-2">
                    <Code2 className="w-4 h-4 text-emerald-400" />
                    <h2 className="text-sm font-semibold text-zinc-200">Interactive WASM Test Sandbox</h2>
                  </div>
                  <button
                    onClick={handleRunTest}
                    disabled={testing}
                    className="px-4 py-1.5 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-xs font-semibold text-white rounded-lg flex items-center gap-1.5 transition-colors shadow-sm"
                  >
                    <Play className={`w-3.5 h-3.5 ${testing ? 'animate-spin' : ''}`} />
                    {testing ? 'Executing...' : 'Run Test'}
                  </button>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <label className="block text-[11px] font-medium text-zinc-400 mb-1">Sample JSON Input</label>
                    <textarea
                      rows={6}
                      value={testInput}
                      onChange={(e) => setTestInput(e.target.value)}
                      className="w-full p-3 bg-[#0A0A0A] border border-[#27272A] rounded-lg font-mono text-xs text-zinc-300 focus:outline-none focus:border-emerald-500/50"
                    />
                  </div>

                  <div>
                    <label className="block text-[11px] font-medium text-zinc-400 mb-1">Live Sandbox Output</label>
                    <div className="h-[138px] p-3 bg-[#0A0A0A] border border-[#27272A] rounded-lg overflow-y-auto font-mono text-[11px] text-zinc-300 scrollbar-thin">
                      {testResult ? (
                        <div className="flex flex-col gap-2">
                          <div className="flex items-center gap-2">
                            {testResult.success ? (
                              <span className="text-emerald-400 flex items-center gap-1 text-[10px] font-bold">
                                <CheckCircle2 className="w-3.5 h-3.5" /> PASSED
                              </span>
                            ) : (
                              <span className="text-red-400 flex items-center gap-1 text-[10px] font-bold">
                                <XCircle className="w-3.5 h-3.5" /> FAILED
                              </span>
                            )}
                            <span className="text-zinc-500 text-[10px] flex items-center gap-1">
                              <Clock className="w-3 h-3" /> {testResult.latency_ms}ms
                            </span>
                          </div>
                          <pre className="text-zinc-300 whitespace-pre-wrap">{testResult.output}</pre>
                          {testResult.error && <p className="text-red-400">{testResult.error}</p>}
                        </div>
                      ) : (
                        <p className="text-zinc-500 italic">Click &apos;Run Test&apos; to execute in isolated sandbox.</p>
                      )}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          ) : (
            <div className="bg-[#111113] border border-[#27272A] rounded-xl p-12 text-center text-zinc-500 text-xs">
              Select a skill or create a new one to begin editing.
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
