import React, { useState } from 'react';
import { SkeletonRows } from './Skeleton';
import {
  Key,
  Server,
  Globe,
  Copy,
  Check,
  ShieldCheck,
  Activity,
  Plus,
  Trash2,
  Lock,
  Eye,
  EyeOff,
  Radio,
  ExternalLink,
} from 'lucide-react';

interface ApiKeyItem {
  id: string;
  name: string;
  prefix: string;
  fullSecret: string;
  rateLimit: string;
  created: string;
  lastUsed: string;
}

interface ConnectedClient {
  ip: string;
  userAgent: string;
  tokensConsumed: number;
  activeSince: string;
  status: 'active' | 'idle';
}

export function GatewayTab({ notify }: { notify: (msg: string) => void }) {
  const [gatewayEnabled, setGatewayEnabled] = useState(true);
  const [keys, setKeys] = useState<ApiKeyItem[]>([
    {
      id: 'key-1',
      name: 'jan-local-app',
      prefix: 'oxk_9f2a',
      fullSecret: 'oxk_9f2ab71de0042a98f12c3e41b9d0',
      rateLimit: '10,000 req/min',
      created: '2026-09-18',
      lastUsed: '2 mins ago',
    },
    {
      id: 'key-2',
      name: 'lm-studio-desktop',
      prefix: 'oxk_3c8e',
      fullSecret: 'oxk_3c8e19bb445f1280a87d091e77aa',
      rateLimit: '5,000 req/min',
      created: '2026-09-19',
      lastUsed: 'Just now',
    },
    {
      id: 'key-3',
      name: 'goose-cli-agent',
      prefix: 'oxk_71d4',
      fullSecret: 'oxk_71d488e100fc921a998b3c1031d9',
      rateLimit: 'Unlimited',
      created: '2026-09-20',
      lastUsed: '15 mins ago',
    },
  ]);

  const [clients, setClients] = useState<ConnectedClient[]>([
    { ip: '127.0.0.1', userAgent: 'Jan-App / 0.5.1', tokensConsumed: 18450, activeSince: '42m ago', status: 'active' },
    { ip: '192.168.1.104', userAgent: 'LM-Studio / 0.3.6', tokensConsumed: 34120, activeSince: '1h 12m ago', status: 'active' },
    { ip: '192.168.1.188', userAgent: 'Goose-Agent / 1.0.0', tokensConsumed: 9280, activeSince: '20m ago', status: 'idle' },
  ]);

  const [isGenerating, setIsGenerating] = useState(false);
  const [tunnelUrl, setTunnelUrl] = useState<string | null>(null);
  const [isTunneling, setIsTunneling] = useState(false);
  const [revealedKeyId, setRevealedKeyId] = useState<string | null>(null);
  const [copiedKeyId, setCopiedKeyId] = useState<string | null>(null);

  const handleGenerateKey = () => {
    setIsGenerating(true);
    setTimeout(() => {
      const hex = Math.random().toString(16).slice(2, 6);
      const full = `oxk_${hex}${Math.random().toString(16).slice(2, 14)}`;
      const newKey: ApiKeyItem = {
        id: `key-${Date.now()}`,
        name: `agent-client-${keys.length + 1}`,
        prefix: `oxk_${hex}`,
        fullSecret: full,
        rateLimit: '10,000 req/min',
        created: new Date().toISOString().slice(0, 10),
        lastUsed: 'Never',
      };
      setKeys((prev) => [newKey, ...prev]);
      setIsGenerating(false);
      notify('New API key generated securely (Blake3 Hash)');
    }, 500);
  };

  const handleCopy = (id: string, text: string) => {
    navigator.clipboard?.writeText(text);
    setCopiedKeyId(id);
    notify('Copied to clipboard');
    setTimeout(() => setCopiedKeyId(null), 2000);
  };

  const handleToggleTunnel = () => {
    if (tunnelUrl) {
      setTunnelUrl(null);
      notify('Cloudflare Tunnel deactivated');
    } else {
      setIsTunneling(true);
      setTimeout(() => {
        setIsTunneling(false);
        const url = 'https://oxide-agent-9f4a.trycloudflare.com';
        setTunnelUrl(url);
        notify('Cloudflare Tunnel active');
      }, 700);
    }
  };

  return (
    <div className="space-y-6 max-w-5xl font-sans">
      {/* 1. Master Server Control */}
      <div className="bg-[#111113] border border-[#27272A] rounded-xl p-6 flex flex-col sm:flex-row sm:items-center justify-between gap-4 shadow-lg">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-[#10B981]/10 border border-[#10B981]/25 flex items-center justify-center shrink-0">
            <Server className="w-5 h-5 text-[#10B981]" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-base font-bold text-[#FAFAFA]">OpenAI-Compatible API Gateway</h2>
              <span
                className={`text-[10px] font-mono px-2 py-0.5 rounded-full font-semibold ${
                  gatewayEnabled
                    ? 'bg-[#10B981]/15 text-[#10B981] border border-[#10B981]/30'
                    : 'bg-zinc-800 text-zinc-400 border border-zinc-700'
                }`}
              >
                {gatewayEnabled ? 'ONLINE · Port 8080' : 'STOPPED'}
              </span>
            </div>
            <p className="text-xs text-[#A1A1AA] mt-1 font-mono">
              http://127.0.0.1:8080/v1 · Actix-Web SSE streaming engine
            </p>
          </div>
        </div>

        {/* Tactile Toggle Switch */}
        <div className="flex items-center gap-3">
          <span className="text-xs font-mono text-zinc-400">{gatewayEnabled ? 'Gateway Active' : 'Gateway Offline'}</span>
          <button
            role="switch"
            aria-checked={gatewayEnabled}
            onClick={() => {
              const next = !gatewayEnabled;
              setGatewayEnabled(next);
              notify(`API Gateway ${next ? 'Started' : 'Stopped'}`);
            }}
            className={`w-12 h-6 rounded-full transition-colors relative cursor-pointer ${
              gatewayEnabled ? 'bg-[#10B981]' : 'bg-[#27272A]'
            }`}
          >
            <span
              className={`absolute top-0.5 w-5 h-5 rounded-full bg-white transition-all shadow-md ${
                gatewayEnabled ? 'left-6.5' : 'left-0.5'
              }`}
            />
          </button>
        </div>
      </div>

      {/* 2. API Keys Management Table */}
      <div className="bg-[#111113] border border-[#27272A] rounded-xl p-6 shadow-lg space-y-4">
        <div className="flex items-center justify-between pb-3 border-b border-[#27272A]">
          <div>
            <h3 className="text-sm font-bold text-[#FAFAFA] flex items-center gap-2">
              <Key className="w-4 h-4 text-[#8B5CF6]" />
              API Keys & Security
            </h3>
            <p className="text-xs text-[#A1A1AA] mt-0.5">
              Secure Blake3-hashed tokens for local and network agents.
            </p>
          </div>
          <button
            onClick={handleGenerateKey}
            disabled={isGenerating}
            className="px-3.5 py-1.5 rounded-lg bg-[#FAFAFA] text-black text-xs font-bold hover:bg-white transition hover:-translate-y-px active:translate-y-0 disabled:opacity-50 cursor-pointer flex items-center gap-1.5"
          >
            <Plus className="w-3.5 h-3.5" />
            {isGenerating ? 'Generating…' : 'Generate Key'}
          </button>
        </div>

        {isGenerating && <SkeletonRows rows={1} />}

        <div className="overflow-x-auto">
          <table className="w-full text-xs font-mono">
            <thead>
              <tr className="text-zinc-500 border-b border-[#27272A] text-left">
                <th className="py-2.5 px-3">Client Name</th>
                <th className="py-2.5 px-3">Token Prefix</th>
                <th className="py-2.5 px-3">Rate Limit</th>
                <th className="py-2.5 px-3">Created</th>
                <th className="py-2.5 px-3">Last Used</th>
                <th className="py-2.5 px-3 text-right">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-[#27272A]">
              {keys.map((k) => {
                const isRevealed = revealedKeyId === k.id;
                const isCopied = copiedKeyId === k.id;
                return (
                  <tr key={k.id} className="hover:bg-[#18181b] text-zinc-300 transition-colors">
                    <td className="py-3 px-3 font-semibold text-[#FAFAFA]">{k.name}</td>
                    <td className="py-3 px-3">
                      <div
                        onMouseEnter={() => setRevealedKeyId(k.id)}
                        onMouseLeave={() => setRevealedKeyId(null)}
                        className="inline-flex items-center gap-1.5 px-2 py-1 rounded bg-[#18181b] border border-[#27272A] text-xs transition cursor-pointer"
                        title="Hover to reveal full secret"
                      >
                        <span className={isRevealed ? 'text-[#8B5CF6] font-bold' : 'blur-[3px] select-none text-zinc-400'}>
                          {isRevealed ? k.fullSecret : `${k.prefix}••••••••••••••••`}
                        </span>
                      </div>
                    </td>
                    <td className="py-3 px-3 text-zinc-400">{k.rateLimit}</td>
                    <td className="py-3 px-3 text-zinc-400">{k.created}</td>
                    <td className="py-3 px-3 text-zinc-400">{k.lastUsed}</td>
                    <td className="py-3 px-3 text-right">
                      <div className="flex items-center justify-end gap-1.5">
                        <button
                          onClick={() => handleCopy(k.id, k.fullSecret)}
                          title="Copy Full Secret"
                          className="p-1.5 rounded-md hover:bg-[#27272A] text-zinc-300 hover:text-white transition cursor-pointer"
                        >
                          {isCopied ? <Check className="w-3.5 h-3.5 text-[#10B981]" /> : <Copy className="w-3.5 h-3.5" />}
                        </button>
                        <button
                          onClick={() => {
                            setKeys((prev) => prev.filter((item) => item.id !== k.id));
                            notify(`Revoked API key: ${k.name}`);
                          }}
                          title="Revoke Key"
                          className="p-1.5 rounded-md hover:bg-red-500/10 text-zinc-500 hover:text-red-400 transition cursor-pointer"
                        >
                          <Trash2 className="w-3.5 h-3.5" />
                        </button>
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>

      {/* 3. Connected Clients Live Table */}
      <div className="bg-[#111113] border border-[#27272A] rounded-xl p-6 shadow-lg space-y-3">
        <div className="flex items-center justify-between pb-3 border-b border-[#27272A]">
          <div>
            <h3 className="text-sm font-bold text-[#FAFAFA] flex items-center gap-2">
              <Activity className="w-4 h-4 text-[#10B981]" />
              Active Agent Connections
            </h3>
            <p className="text-xs text-[#A1A1AA] mt-0.5">
              Live external clients consuming inference through this instance.
            </p>
          </div>
          <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-[#10B981]/10 text-[#10B981] border border-[#10B981]/25">
            {clients.length} Clients Attached
          </span>
        </div>

        <table className="w-full text-xs font-mono">
          <thead>
            <tr className="text-zinc-500 border-b border-[#27272A] text-left">
              <th className="py-2.5 px-3">IP Address</th>
              <th className="py-2.5 px-3">User Agent / Client</th>
              <th className="py-2.5 px-3">Session Age</th>
              <th className="py-2.5 px-3 text-right">Tokens Consumed</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-[#27272A]">
            {clients.map((c, i) => (
              <tr key={i} className="hover:bg-[#18181b] text-zinc-300 transition-colors">
                <td className="py-2.5 px-3 flex items-center gap-2">
                  <span
                    className={`w-1.5 h-1.5 rounded-full ${c.status === 'active' ? 'bg-[#10B981]' : 'bg-zinc-500'}`}
                  />
                  {c.ip}
                </td>
                <td className="py-2.5 px-3 font-semibold text-[#FAFAFA]">{c.userAgent}</td>
                <td className="py-2.5 px-3 text-zinc-400">{c.activeSince}</td>
                <td className="py-2.5 px-3 text-right text-[#10B981] font-bold">
                  {c.tokensConsumed.toLocaleString()} tok
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* 4. Cloudflare Tunnel Network Sharing Card */}
      <div className="bg-[#111113] border border-[#27272A] rounded-xl p-6 shadow-lg flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <div className="flex items-center gap-2">
            <Globe className="w-4 h-4 text-cyan-400" />
            <h3 className="text-sm font-bold text-[#FAFAFA]">Cloudflare Edge Tunnel</h3>
            {tunnelUrl && (
              <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-[#8B5CF6]/15 text-[#8B5CF6] border border-[#8B5CF6]/30">
                HTTPS Online
              </span>
            )}
          </div>
          <p className="text-xs text-[#A1A1AA] mt-1 font-mono">
            {tunnelUrl ? (
              <span className="text-zinc-200">{tunnelUrl}</span>
            ) : (
              'Expose local models to remote Claude / Jan clients without port forwarding'
            )}
          </p>
        </div>

        <div className="flex items-center gap-2.5">
          {tunnelUrl && (
            <button
              onClick={() => handleCopy('tunnel-url', tunnelUrl)}
              className="px-3 py-1.5 rounded-lg bg-[#18181b] border border-[#27272A] text-xs font-mono text-zinc-200 hover:text-white hover:border-zinc-500 transition cursor-pointer flex items-center gap-1.5"
            >
              <Copy className="w-3.5 h-3.5" />
              Copy URL
            </button>
          )}
          <button
            onClick={handleToggleTunnel}
            disabled={isTunneling}
            className={`px-4 py-1.5 rounded-lg text-xs font-bold transition cursor-pointer ${
              tunnelUrl
                ? 'bg-red-500/10 border border-red-500/30 text-red-400 hover:bg-red-500/20'
                : 'bg-[#FAFAFA] text-black hover:bg-white'
            }`}
          >
            {isTunneling ? 'Connecting…' : tunnelUrl ? 'Disconnect' : 'Expose to Internet'}
          </button>
        </div>
      </div>
    </div>
  );
}
