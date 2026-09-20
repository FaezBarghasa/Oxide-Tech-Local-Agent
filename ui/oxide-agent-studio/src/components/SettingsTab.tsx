import React, { useState, useEffect } from 'react';
import { Settings, Save, RefreshCw, Sliders, CheckCircle2, AlertCircle, FileText } from 'lucide-react';
import { desktop } from '../lib/desktop';

export const SettingsTab: React.FC = () => {
  const [profile, setProfile] = useState<'lite' | 'standard' | 'pro' | 'airgapped' | 'enterprise'>('standard');
  const [configContent, setConfigContent] = useState('');
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [statusMsg, setStatusMsg] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const loadConfig = async () => {
    setLoading(true);
    setError(null);
    try {
      if (desktop.isDesktop) {
        const file = await desktop.configLoad();
        setConfigContent(typeof file === 'string' ? file : (file as any).content || '');
      } else {
        setConfigContent(`# Oxide-Tech Local Agent OS Configuration (Web Preview Mode)
[gateway]
latency_threshold_ms = 2000
quality_threshold = 0.85
max_retries = 5

[thinker]
provider = "ollama"
model = "qwen2.5-coder:32b-instruct-q4_K_M"
base_url = "http://localhost:11434"

[coder.local]
provider = "ollama"
model = "qwen2.5-coder:7b-instruct-q4_K_M"
base_url = "http://localhost:11434"

[rag]
collection = "rust_rag"
top_k = 8
auto_update_on_startup = true

[mcp]
transport = "stdio"
tcp_port = 9090
`);
      }
    } catch (err: any) {
      setError(err?.message || String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    setSaving(true);
    setError(null);
    setStatusMsg(null);
    try {
      if (desktop.isDesktop) {
        await desktop.configSave(configContent);
        setStatusMsg('Configuration successfully validated and saved to disk.');
      } else {
        setStatusMsg('Configuration updated (in-memory web mode).');
      }
      setTimeout(() => setStatusMsg(null), 3000);
    } catch (err: any) {
      setError(err?.message || String(err));
    } finally {
      setSaving(false);
    }
  };

  useEffect(() => {
    loadConfig();
  }, []);

  return (
    <div className="space-y-6 font-sans">
      {/* Header & Profile Switcher */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Settings className="w-4 h-4 text-orange-400" />
              <span>Settings & Operating Profile</span>
            </div>
            <div className="text-[11px] mono text-gray-400 mt-1">
              Deterministic runtime profiles, hardware limits & hot-reloadable config.toml
            </div>
          </div>
          <div className="flex items-center gap-3">
            <button
              onClick={handleSave}
              disabled={saving || loading}
              className="px-4 py-2 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-slate-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-2 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(249,115,22,0.35)]"
            >
              <Save className="w-3.5 h-3.5" />
              <span>{saving ? 'Validating...' : 'Save Config'}</span>
            </button>
          </div>
        </div>

        {/* Profile Preset Selectors */}
        <div className="pt-4">
          <label className="text-[10px] uppercase tracking-widest text-gray-400 mb-2 block flex items-center gap-1.5">
            <Sliders className="w-3 h-3 text-orange-400" /> Active Operating Profile
          </label>
          <div className="grid grid-cols-2 sm:grid-cols-5 gap-2.5">
            {(['lite', 'standard', 'pro', 'airgapped', 'enterprise'] as const).map((p) => (
              <button
                key={p}
                onClick={() => setProfile(p)}
                className={`p-3 rounded-xl border text-left transition cursor-pointer ${
                  profile === p
                    ? 'bg-orange-500/10 border-orange-500/50 text-white shadow-[0_0_10px_rgba(249,115,22,0.2)]'
                    : 'bg-[#161822] border-[#242738] text-gray-400 hover:text-gray-200'
                }`}
              >
                <div className="text-xs font-bold uppercase tracking-wider">{p}</div>
                <div className="text-[9px] mono text-gray-400 mt-0.5">
                  {p === 'lite' && 'CPU / Low RAM'}
                  {p === 'standard' && 'Single GPU'}
                  {p === 'pro' && 'Dual GPU + LoRA'}
                  {p === 'airgapped' && 'Strict Offline'}
                  {p === 'enterprise' && 'RBAC + Audit'}
                </div>
              </button>
            ))}
          </div>
        </div>
      </div>

      {statusMsg && (
        <div className="p-4 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-300 text-xs mono flex items-center gap-2">
          <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
          <span>{statusMsg}</span>
        </div>
      )}

      {error && (
        <div className="p-4 rounded-xl bg-rose-500/10 border border-rose-500/30 text-rose-300 text-xs mono flex items-center gap-2">
          <AlertCircle className="w-4 h-4 text-rose-400 shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* config.toml Editor */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl space-y-3">
        <div className="flex items-center justify-between pb-3 border-b border-[#232530]">
          <h3 className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2">
            <FileText className="w-4 h-4 text-orange-400" />
            <span>config.toml (Interactive Editor)</span>
          </h3>
          <button
            onClick={loadConfig}
            disabled={loading}
            className="px-2.5 py-1 rounded bg-[#181a24] hover:bg-[#202330] border border-[#2c2f3d] text-[10px] mono text-gray-300 flex items-center gap-1.5 transition cursor-pointer"
          >
            <RefreshCw className={`w-3 h-3 ${loading ? 'animate-spin' : ''}`} />
            <span>Reload</span>
          </button>
        </div>

        <textarea
          value={configContent}
          onChange={(e) => setConfigContent(e.target.value)}
          spellCheck={false}
          className="w-full bg-[#0c0d12] border border-[#232530] rounded-xl p-4 text-[11px] mono text-orange-100/90 leading-relaxed font-mono min-h-[420px] focus:outline-none focus:border-orange-500/60 resize-y"
        />
      </div>
    </div>
  );
};
