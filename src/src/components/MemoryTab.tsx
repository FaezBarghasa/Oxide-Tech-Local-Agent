import React, { useCallback, useEffect, useState } from 'react';
import { Brain, RefreshCw, Search, BookmarkPlus, MessagesSquare, ScanSearch, Database, FolderGit2 } from 'lucide-react';
import { desktop, embedText, MemoryEnv, EmbedResult } from '../lib/desktop';

type RunState = { running: boolean; label: string | null };

export const MemoryTab: React.FC = () => {
  const [env, setEnv] = useState<MemoryEnv | null>(null);
  const [envError, setEnvError] = useState<string | null>(null);
  const [projectDir, setProjectDir] = useState('');
  const [run, setRun] = useState<RunState>({ running: false, label: null });
  const [output, setOutput] = useState('// oxide-embed output appears here');

  // remember form
  const [rememberContent, setRememberContent] = useState('');
  const [rememberKind, setRememberKind] = useState('fact');
  const [rememberTags, setRememberTags] = useState('');
  const [rememberSymbol, setRememberSymbol] = useState('');
  const [autoResolve, setAutoResolve] = useState(true);

  // search / recall form
  const [query, setQuery] = useState('');
  const [stair, setStair] = useState(true);
  const [limit, setLimit] = useState('8');

  // context form
  const [task, setTask] = useState('');
  const [budget, setBudget] = useState('1500');

  const cwd = projectDir.trim() || undefined;

  const refreshEnv = useCallback(async () => {
    setEnvError(null);
    try {
      const e = await desktop.memoryEnv(cwd);
      setEnv(e);
      if (!projectDir && e.cwd && !e.cwd.startsWith('(')) setProjectDir(e.cwd);
    } catch (err) {
      setEnvError(err instanceof Error ? err.message : String(err));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    void refreshEnv();
  }, [refreshEnv]);

  const exec = async (label: string, fn: () => Promise<EmbedResult | string>) => {
    setRun({ running: true, label });
    try {
      const res = await fn();
      setOutput(typeof res === 'string' ? res : embedText(res));
    } catch (err) {
      setOutput(`error: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      setRun({ running: false, label: null });
    }
  };

  const btn =
    'px-4 py-2 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-gray-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-1.5 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(249,115,22,0.35)]';
  const ghost =
    'px-4 py-2 rounded-lg bg-[#1a1c26] hover:bg-[#22242f] border border-[#2c2f3d] text-gray-200 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-1.5 cursor-pointer disabled:opacity-50';
  const input =
    'w-full bg-[#0c0d12] border border-[#2c2f3d] rounded-lg px-3 py-2 text-xs font-mono text-gray-200 placeholder:text-gray-600 focus:outline-none focus:border-orange-500/60';

  return (
    <div className="space-y-6 font-sans">
      {/* Status header */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-[#232530]">
          <div>
            <h2 className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Brain className="w-4 h-4 text-orange-400" />
              <span>Project Memory · oxide-embed</span>
            </h2>
            <p className="text-[11px] font-mono text-gray-400 mt-1">
              {env ? (
                <>
                  {env.bin} · {env.version || 'version unknown'} · manifest{' '}
                  {env.manifest_present ? 'present' : 'missing'}
                  {!desktop.isDesktop && ' · browser mode (read-only gateway fallback)'}
                </>
              ) : (
                envError ?? 'probing memory backend…'
              )}
            </p>
          </div>
          <button className={ghost} onClick={() => void refreshEnv()} disabled={run.running}>
            <RefreshCw className="w-3.5 h-3.5" />
            <span>Refresh</span>
          </button>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-[1fr_auto_auto_auto] gap-3 pt-4 items-end">
          <label className="block">
            <span className="text-[10px] uppercase tracking-widest text-gray-500 flex items-center gap-1 mb-1">
              <FolderGit2 className="w-3 h-3" /> Project directory (contains .oxide/)
            </span>
            <input
              className={input}
              value={projectDir}
              onChange={(e) => setProjectDir(e.target.value)}
              placeholder="/path/to/project"
              spellCheck={false}
            />
          </label>
          <button className={ghost} disabled={run.running || !desktop.isDesktop} onClick={() => void exec('status', () => desktop.memoryStatus(cwd))}>
            <Database className="w-3.5 h-3.5" /><span>Status</span>
          </button>
          <button className={ghost} disabled={run.running || !desktop.isDesktop} onClick={() => void exec('init', () => desktop.memoryInit(cwd))}>
            <span>Init</span>
          </button>
          <button className={btn} disabled={run.running || !desktop.isDesktop} onClick={() => void exec('index', () => desktop.memoryIndex(cwd, true))}>
            <span>{run.label === 'index' ? 'Indexing…' : 'Re-index'}</span>
          </button>
        </div>
      </div>

      {/* Search / recall */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl">
        <h3 className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2 pb-3 border-b border-[#232530]">
          <Search className="w-4 h-4 text-orange-400" /> Semantic search & recall
        </h3>
        <div className="grid grid-cols-1 lg:grid-cols-[1fr_120px_auto] gap-3 pt-4 items-end">
          <input className={input} value={query} onChange={(e) => setQuery(e.target.value)} placeholder="symbol, topic, or question…" spellCheck={false} />
          <label className="block">
            <span className="text-[10px] uppercase tracking-widest text-gray-500 mb-1 block">Limit</span>
            <input className={input} value={limit} onChange={(e) => setLimit(e.target.value)} inputMode="numeric" />
          </label>
          <div className="flex gap-2">
            <button className={btn} disabled={run.running || !query.trim() || !desktop.isDesktop}
              onClick={() => void exec('search', () => desktop.memorySearch(query, { cwd, stair, limit: Number(limit) || 8 }))}>
              <ScanSearch className="w-3.5 h-3.5" /><span>{run.label === 'search' ? '…' : 'Search'}</span>
            </button>
            <button className={ghost} disabled={run.running || !query.trim() || !desktop.isDesktop}
              onClick={() => void exec('recall', () => desktop.memoryRecall(query, { cwd, limit: Number(limit) || 5 }))}>
              <span>{run.label === 'recall' ? '…' : 'Recall'}</span>
            </button>
          </div>
        </div>
        <label className="flex items-center gap-2 pt-3 text-[11px] font-mono text-gray-400 cursor-pointer">
          <input type="checkbox" checked={stair} onChange={(e) => setStair(e.target.checked)} className="accent-orange-500" />
          hierarchical (--stair) search
        </label>
      </div>

      {/* Remember + context */}
      <div className="grid grid-cols-1 xl:grid-cols-2 gap-6">
        <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl">
          <h3 className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2 pb-3 border-b border-[#232530]">
            <BookmarkPlus className="w-4 h-4 text-orange-400" /> Remember assertion
          </h3>
          <div className="space-y-3 pt-4">
            <textarea className={`${input} min-h-[90px] resize-y`} value={rememberContent} onChange={(e) => setRememberContent(e.target.value)} placeholder="Decision, fact, preference, or goal to store…" />
            <div className="grid grid-cols-3 gap-3">
              <input className={input} value={rememberKind} onChange={(e) => setRememberKind(e.target.value)} placeholder="kind" />
              <input className={input} value={rememberTags} onChange={(e) => setRememberTags(e.target.value)} placeholder="tags a,b" />
              <input className={input} value={rememberSymbol} onChange={(e) => setRememberSymbol(e.target.value)} placeholder="symbol" />
            </div>
            <div className="flex items-center justify-between">
              <label className="flex items-center gap-2 text-[11px] font-mono text-gray-400 cursor-pointer">
                <input type="checkbox" checked={autoResolve} onChange={(e) => setAutoResolve(e.target.checked)} className="accent-orange-500" />
                auto-resolve conflicts
              </label>
              <button className={btn} disabled={run.running || !rememberContent.trim() || !desktop.isDesktop}
                onClick={() => void exec('remember', () => desktop.memoryRemember(rememberContent, { cwd, kind: rememberKind, tags: rememberTags, symbol: rememberSymbol, autoResolve }))}>
                <span>{run.label === 'remember' ? '…' : 'Remember'}</span>
              </button>
            </div>
          </div>
        </div>

        <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl">
          <h3 className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2 pb-3 border-b border-[#232530]">
            <MessagesSquare className="w-4 h-4 text-orange-400" /> Task context pack
          </h3>
          <div className="space-y-3 pt-4">
            <textarea className={`${input} min-h-[90px] resize-y`} value={task} onChange={(e) => setTask(e.target.value)} placeholder="Describe the engineering task to pack context for…" />
            <div className="flex items-center justify-between gap-3">
              <label className="flex items-center gap-2 text-[11px] font-mono text-gray-400">
                budget
                <input className={`${input} !w-24`} value={budget} onChange={(e) => setBudget(e.target.value)} inputMode="numeric" />
              </label>
              <button className={btn} disabled={run.running || !task.trim() || !desktop.isDesktop}
                onClick={() => void exec('context', () => desktop.memoryContext(task, { cwd, budget: Number(budget) || 1500 }))}>
                <span>{run.label === 'context' ? '…' : 'Pack context'}</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Console */}
      <div className="bg-[#0c0d12] border border-[#232530] rounded-2xl p-4 shadow-xl">
        <div className="text-[10px] uppercase tracking-widest text-gray-500 pb-2">oxide-embed console{run.label ? ` · ${run.label} running…` : ''}</div>
        <pre className="text-[11px] font-mono text-gray-300 whitespace-pre-wrap break-words max-h-[420px] overflow-auto">{output}</pre>
      </div>
    </div>
  );
};
