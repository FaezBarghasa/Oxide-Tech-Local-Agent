import React, { useState, useEffect } from 'react';
import {
  Sparkles,
  CheckCircle2,
  FolderDown,
  RefreshCw,
  X,
  FileCheck,
  Cpu,
  Layers,
  Check,
} from 'lucide-react';
import { scanUnslothData, importUnslothItems } from '../lib/desktop';
import { DiscoveredUnslothItemDto } from '../types';

interface UnslothMigrateModalProps {
  isOpen: boolean;
  onClose: () => void;
  onImportComplete?: (count: number) => void;
}

export const UnslothMigrateModal: React.FC<UnslothMigrateModalProps> = ({
  isOpen,
  onClose,
  onImportComplete,
}) => {
  const [items, setItems] = useState<DiscoveredUnslothItemDto[]>([]);
  const [selectedPaths, setSelectedPaths] = useState<Set<string>>(new Set());
  const [scanning, setScanning] = useState(false);
  const [importing, setImporting] = useState(false);
  const [resultMsg, setResultMsg] = useState<string | null>(null);

  const handleScan = async () => {
    setScanning(true);
    try {
      const res = await scanUnslothData();
      setItems(res.items);
      const allPaths = new Set(res.items.map((i) => i.source_path));
      setSelectedPaths(allPaths);
    } catch (err) {
      console.error('Scan failed', err);
    } finally {
      setScanning(false);
    }
  };

  useEffect(() => {
    if (isOpen) {
      handleScan();
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const toggleSelect = (path: string) => {
    setSelectedPaths((prev) => {
      const next = new Set(prev);
      if (next.has(path)) {
        next.delete(path);
      } else {
        next.add(path);
      }
      return next;
    });
  };

  const handleImport = async () => {
    if (selectedPaths.size === 0) return;
    setImporting(true);
    setResultMsg(null);

    try {
      const res = await importUnslothItems(Array.from(selectedPaths));
      setResultMsg(res.message);
      if (onImportComplete) {
        onImportComplete(res.imported_models + res.imported_sessions);
      }
      setTimeout(() => {
        onClose();
      }, 2500);
    } catch (err: unknown) {
      const error = err as Error;
      setResultMsg(`Import failed: ${error.message || 'unknown'}`);
    } finally {
      setImporting(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4 select-none animate-fade-in">
      <div className="bg-[#111113] border border-[#27272A] rounded-xl max-w-2xl w-full p-6 flex flex-col gap-5 shadow-2xl">
        {/* Header */}
        <div className="flex items-center justify-between border-b border-[#27272A] pb-4">
          <div className="flex items-center gap-3">
            <div className="w-9 h-9 rounded-lg bg-orange-500/10 border border-orange-500/30 flex items-center justify-center text-orange-400">
              <FolderDown className="w-5 h-5" />
            </div>
            <div>
              <h2 className="text-base font-semibold text-zinc-100 flex items-center gap-2">
                Unsloth Desktop Migration Wizard
                <span className="text-[10px] px-2 py-0.5 rounded bg-orange-500/20 text-orange-300 font-mono">
                  v0.1.900-beta Bridge
                </span>
              </h2>
              <p className="text-xs text-zinc-400">
                Import your downloaded GGUF models, fine-tuned LoRA weights, and sessions from Unsloth.
              </p>
            </div>
          </div>
          <button onClick={onClose} className="p-1 hover:bg-[#18181B] text-zinc-400 rounded-lg">
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Scan Results List */}
        <div className="flex flex-col gap-2">
          <div className="flex items-center justify-between text-xs text-zinc-400">
            <span>Discovered Unsloth Assets ({items.length})</span>
            <button onClick={handleScan} className="hover:text-zinc-200 flex items-center gap-1">
              <RefreshCw className={`w-3 h-3 ${scanning ? 'animate-spin' : ''}`} /> Rescan
            </button>
          </div>

          <div className="max-h-64 overflow-y-auto bg-[#0A0A0A] border border-[#27272A] rounded-lg divide-y divide-[#1F1F23]">
            {scanning ? (
              <div className="p-8 text-center text-xs text-zinc-500 flex items-center justify-center gap-2">
                <RefreshCw className="w-4 h-4 animate-spin text-orange-400" /> Scanning ~/.unsloth and local caches...
              </div>
            ) : items.length === 0 ? (
              <div className="p-8 text-center text-xs text-zinc-500">No Unsloth models or sessions detected.</div>
            ) : (
              items.map((item) => {
                const isChecked = selectedPaths.has(item.source_path);
                return (
                  <div
                    key={item.source_path}
                    onClick={() => toggleSelect(item.source_path)}
                    className="p-3 flex items-center justify-between gap-3 hover:bg-[#141416] cursor-pointer"
                  >
                    <div className="flex items-center gap-3 min-w-0">
                      <div
                        className={`w-4 h-4 rounded border flex items-center justify-center ${
                          isChecked ? 'bg-orange-600 border-orange-500 text-white' : 'border-zinc-600'
                        }`}
                      >
                        {isChecked && <Check className="w-3 h-3" />}
                      </div>
                      <div className="flex flex-col min-w-0">
                        <span className="text-xs font-semibold text-zinc-200 truncate">{item.name}</span>
                        <span className="text-[10px] text-zinc-500">
                          {item.details} • {item.size_formatted}
                        </span>
                      </div>
                    </div>
                    <span className="text-[10px] font-mono text-zinc-500 uppercase bg-zinc-900 px-2 py-0.5 rounded">
                      {item.item_type}
                    </span>
                  </div>
                );
              })
            )}
          </div>
        </div>

        {resultMsg && (
          <div className="p-3 rounded-lg bg-emerald-500/10 border border-emerald-500/30 text-xs text-emerald-300 flex items-center gap-2">
            <CheckCircle2 className="w-4 h-4 text-emerald-400" />
            {resultMsg}
          </div>
        )}

        {/* Footer */}
        <div className="flex items-center justify-between pt-2 border-t border-[#27272A]">
          <span className="text-xs text-zinc-500">{selectedPaths.size} items selected for import</span>
          <div className="flex items-center gap-2">
            <button
              onClick={onClose}
              className="px-3.5 py-1.5 rounded-lg bg-[#18181B] hover:bg-[#27272A] text-xs font-medium text-zinc-300"
            >
              Cancel
            </button>
            <button
              onClick={handleImport}
              disabled={importing || selectedPaths.size === 0}
              className="px-4 py-1.5 rounded-lg bg-orange-600 hover:bg-orange-500 disabled:opacity-50 text-xs font-semibold text-white flex items-center gap-1.5 shadow-sm"
            >
              <Sparkles className="w-3.5 h-3.5" />
              {importing ? 'Importing...' : 'Import Selected Assets'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
