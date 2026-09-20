/**
 * Desktop bridge: Oxide Agent Studio ↔ Tauri backend (`src-tauri`).
 *
 * Inside the .deb-installed desktop app every memory operation is a Tauri
 * command that shells out to the bundled `oxide-embed` sidecar, so the UI
 * never needs the gateway for memory. When running in a plain browser
 * (vite dev / express server), calls fall back to the gateway HTTP API.
 */

export interface EmbedResult {
  code: number;
  stdout: string;
  stderr: string;
}

export interface MemoryEnv {
  bin: string;
  version: string;
  cwd: string;
  manifest_present: boolean;
}

export function isTauriRuntime(): boolean {
  return (
    typeof window !== 'undefined' &&
    ('__TAURI_INTERNALS__' in window || '__TAURI__' in window)
  );
}

async function tauriInvoke<T>(cmd: string, args: Record<string, unknown>): Promise<T> {
  const mod = await import('@tauri-apps/api/core');
  return mod.invoke<T>(cmd, args);
}

const GATEWAY = (import.meta as unknown as { env?: Record<string, string> }).env
  ?.VITE_GATEWAY_URL ?? 'http://127.0.0.1:8080';

async function gatewayGet(path: string): Promise<string> {
  const res = await fetch(`${GATEWAY}${path}`);
  if (!res.ok) throw new Error(`gateway ${path} → HTTP ${res.status}`);
  return res.text();
}

/** Combined output helper: prefer stdout, fall back to stderr. */
export function embedText(r: EmbedResult): string {
  if (r.stdout.trim()) return r.stdout;
  if (r.stderr.trim()) return r.stderr;
  return `(exit ${r.code}, no output)`;
}

export const desktop = {
  isDesktop: isTauriRuntime(),

  async memoryEnv(cwd?: string): Promise<MemoryEnv> {
    if (!isTauriRuntime()) {
      return {
        bin: 'oxide-embed (via gateway host PATH)',
        version: 'browser mode — use desktop app for direct memory access',
        cwd: cwd ?? '(gateway working directory)',
        manifest_present: false,
      };
    }
    return tauriInvoke<MemoryEnv>('memory_env', { cwd: cwd ?? null });
  },

  async gatewayStatus(baseUrl?: string): Promise<number> {
    if (!isTauriRuntime()) {
      const res = await fetch(`${baseUrl ?? GATEWAY}/health/live`);
      return res.status;
    }
    return tauriInvoke<number>('gateway_status', { baseUrl: baseUrl ?? null });
  },

  async memoryStatus(cwd?: string): Promise<EmbedResult | string> {
    if (!isTauriRuntime()) return gatewayGet('/api/status');
    return tauriInvoke<EmbedResult>('memory_status', { cwd: cwd ?? null });
  },

  async memoryInit(cwd?: string, name?: string): Promise<EmbedResult> {
    if (!isTauriRuntime()) throw new Error('memory init requires the desktop app');
    return tauriInvoke<EmbedResult>('memory_init', { cwd: cwd ?? null, name: name ?? null });
  },

  async memoryIndex(cwd?: string, force?: boolean): Promise<EmbedResult> {
    if (!isTauriRuntime()) throw new Error('memory index requires the desktop app');
    return tauriInvoke<EmbedResult>('memory_index', { cwd: cwd ?? null, force: force ?? false });
  },

  async memorySearch(
    query: string,
    opts: { cwd?: string; stair?: boolean; limit?: number; budget?: number; withGraph?: boolean } = {},
  ): Promise<EmbedResult> {
    if (!isTauriRuntime()) throw new Error('memory search requires the desktop app');
    return tauriInvoke<EmbedResult>('memory_search', {
      cwd: opts.cwd ?? null,
      query,
      stair: opts.stair ?? false,
      limit: opts.limit ?? null,
      budget: opts.budget ?? null,
      withGraph: opts.withGraph ?? false,
    });
  },

  async memoryRecall(
    query: string,
    opts: { cwd?: string; kind?: string; tags?: string; budget?: number; limit?: number } = {},
  ): Promise<EmbedResult> {
    if (!isTauriRuntime()) throw new Error('memory recall requires the desktop app');
    return tauriInvoke<EmbedResult>('memory_recall', {
      cwd: opts.cwd ?? null,
      query,
      kind: opts.kind ?? null,
      tags: opts.tags ?? null,
      budget: opts.budget ?? null,
      limit: opts.limit ?? null,
    });
  },

  async memoryRemember(
    content: string,
    opts: { cwd?: string; kind?: string; tags?: string; symbol?: string; autoResolve?: boolean } = {},
  ): Promise<EmbedResult> {
    if (!isTauriRuntime()) throw new Error('memory remember requires the desktop app');
    return tauriInvoke<EmbedResult>('memory_remember', {
      cwd: opts.cwd ?? null,
      content,
      kind: opts.kind ?? null,
      tags: opts.tags ?? null,
      symbol: opts.symbol ?? null,
      autoResolve: opts.autoResolve ?? false,
    });
  },

  async memoryContext(task: string, opts: { cwd?: string; budget?: number } = {}): Promise<EmbedResult> {
    if (!isTauriRuntime()) throw new Error('memory context requires the desktop app');
    return tauriInvoke<EmbedResult>('memory_context', {
      cwd: opts.cwd ?? null,
      task,
      budget: opts.budget ?? null,
    });
  },

  async memoryExplain(symbol: string, opts: { cwd?: string; hops?: number } = {}): Promise<EmbedResult> {
    if (!isTauriRuntime()) throw new Error('memory explain requires the desktop app');
    return tauriInvoke<EmbedResult>('memory_explain', {
      cwd: opts.cwd ?? null,
      symbol,
      hops: opts.hops ?? null,
    });
  },

  async memoryConflicts(cwd?: string): Promise<EmbedResult> {
    if (!isTauriRuntime()) throw new Error('memory conflicts requires the desktop app');
    return tauriInvoke<EmbedResult>('memory_conflicts', { cwd: cwd ?? null });
  },

  /** STAIR Code-ToC search + pack results as chat context prefix. Returns injected prefix (empty when disabled/unavailable). */
  async injectStairContext(prompt: string, opts: { cwd?: string; budget?: number; limit?: number } = {}): Promise<{ prefix: string; breadcrumbs: string[]; tokens: number }> {
    if (!isTauriRuntime()) return { prefix: '', breadcrumbs: [], tokens: 0 };
    try {
      const res = await this.memorySearch(prompt, { cwd: opts.cwd, stair: true, limit: opts.limit ?? 5, budget: opts.budget ?? 1500, withGraph: true });
      const text = embedText(res);
      const lines = text.split('\n').filter((l) => l.trim()).slice(0, opts.limit ?? 5);
      const prefix = lines.length
        ? `[STAIR Code-ToC context, budget ${opts.budget ?? 1500} tokens]\n${lines.join('\n')}\n---\n`
        : '';
      return { prefix, breadcrumbs: lines, tokens: prefix.length >> 2 };
    } catch {
      return { prefix: '', breadcrumbs: [], tokens: 0 };
    }
  },

  // Doctor Diagnostics
  async doctorRunDiagnostics(): Promise<any> {
    if (!isTauriRuntime()) throw new Error('doctor diagnostics requires the desktop app');
    return tauriInvoke<any>('doctor_run_diagnostics', {});
  },

  async doctorInstallUdevRules(): Promise<any> {
    if (!isTauriRuntime()) throw new Error('udev install requires the desktop app');
    return tauriInvoke<any>('doctor_install_udev_rules', {});
  },

  // RE-Forge Binary/PTX Analysis
  async reforgeAnalyzeFile(request: any): Promise<any> {
    if (!isTauriRuntime()) throw new Error('reforge analysis requires the desktop app');
    return tauriInvoke<any>('reforge_analyze_file', request);
  },

  // Verifier Suite
  async verifierRunSuite(request: any): Promise<any> {
    if (!isTauriRuntime()) throw new Error('verifier suite requires the desktop app');
    return tauriInvoke<any>('verifier_run_suite', request);
  },

  async verifierExportEvidence(exportPath: string): Promise<any> {
    if (!isTauriRuntime()) throw new Error('verifier export requires the desktop app');
    return tauriInvoke<any>('verifier_export_evidence', { exportPath });
  },

  // Hardware & probe-rs
  async hardwareListProbes(): Promise<any> {
    if (!isTauriRuntime()) {
      return {
        devices: [
          { identifier: 'ST-Link V2 (SWD)', vendorId: 0x0483, productId: 0x3748, productName: 'ST-Link Debug Probe', manufacturer: 'STMicroelectronics' },
        ],
        error: null,
      };
    }
    return tauriInvoke<any>('hardware_list_probes', {});
  },

  async hardwareGetChipInfo(deviceIdentifier: string): Promise<any> {
    if (!isTauriRuntime()) {
      return {
        name: 'STM32F401RE',
        part: 'ARM Cortex-M4F',
        cores: [{ name: 'main', coreType: 'Cortex-M4' }],
        memoryRegions: [
          { name: 'FLASH', rangeStart: 0x08000000, rangeEnd: 0x08080000, isFlash: true, isRam: false },
          { name: 'SRAM', rangeStart: 0x20000000, rangeEnd: 0x20018000, isFlash: false, isRam: true },
        ],
      };
    }
    return tauriInvoke<any>('hardware_get_chip_info', { deviceIdentifier });
  },

  async hardwareFlashFirmware(request: any): Promise<any> {
    if (!isTauriRuntime()) {
      return {
        success: true,
        message: 'Simulated flash complete (running in browser mode)',
        bytesWritten: 32768,
        durationMs: 150,
      };
    }
    return tauriInvoke<any>('hardware_flash_firmware', { request });
  },

  // Aliases for compatibility
  async probeRsListDevices(): Promise<any> {
    return this.hardwareListProbes();
  },

  async probeRsGetChipInfo(deviceIdentifier: string): Promise<any> {
    return this.hardwareGetChipInfo(deviceIdentifier);
  },

  async probeRsFlashFirmware(request: any): Promise<any> {
    return this.hardwareFlashFirmware(request);
  },

  // Gateway Daemon Control
  async gatewayDaemonStart(config?: string): Promise<any> {
    if (!isTauriRuntime()) return { status: 'online' };
    return tauriInvoke<any>('gateway_status', {});
  },

  async gatewayDaemonStop(): Promise<any> {
    if (!isTauriRuntime()) return { status: 'stopped' };
    return { status: 'ok' };
  },

  async gatewayDaemonRestart(config?: string): Promise<any> {
    if (!isTauriRuntime()) return { status: 'restarted' };
    return tauriInvoke<any>('gateway_status', {});
  },

  async gatewayDaemonLogs(): Promise<any> {
    return [];
  },

  // Config
  async configRead(): Promise<string> {
    if (!isTauriRuntime()) {
      return '# Oxide-Tech Local Agent Configuration (Default Profile: standard)\n\n[gateway]\nhost = "127.0.0.1"\nport = 8080\n';
    }
    return tauriInvoke<string>('config_read', {});
  },

  async configLoad(): Promise<string> {
    return this.configRead();
  },

  async configSave(content: string): Promise<any> {
    if (!isTauriRuntime()) return { success: true };
    return tauriInvoke<any>('config_save', { content });
  },
};
