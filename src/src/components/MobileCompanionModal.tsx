import React, { useState } from 'react';

interface MobileCompanionModalProps {
  isOpen: boolean;
  onClose: () => void;
  hostIp?: string;
  port?: number;
  hostPk?: string;
}

export const MobileCompanionModal: React.FC<MobileCompanionModalProps> = ({
  isOpen,
  onClose,
  hostIp = '127.0.0.1',
  port = 8080,
  hostPk = 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855',
}) => {
  const [copied, setCopied] = useState(false);

  if (!isOpen) return null;

  const pairUrl = `https://${hostIp}:${port}/mobile/#pk=${hostPk}&nonce=${Date.now()}`;

  const handleCopy = () => {
    navigator.clipboard.writeText(pairUrl);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/75 backdrop-blur-sm">
      <div className="bg-slate-900 border border-slate-700 rounded-xl p-6 w-full max-w-md shadow-2xl flex flex-col gap-4 font-mono text-slate-100">
        <div className="flex justify-between items-center border-b border-slate-800 pb-3">
          <div className="flex items-center gap-2">
            <span className="w-2.5 h-2.5 rounded-full bg-cyan-400 animate-pulse" />
            <h2 className="text-sm font-bold tracking-wider">PAIR MOBILE COMPANION</h2>
          </div>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-slate-200 text-sm font-bold"
          >
            ✕
          </button>
        </div>

        <div className="text-xs text-slate-400 leading-relaxed">
          Scan with your smartphone camera to establish a zero-trust, sovereign P2P WebRTC connection. No cloud relay or VPN required.
        </div>

        {/* Optical QR Pairing Visualizer */}
        <div className="flex flex-col items-center justify-center p-6 bg-white rounded-lg border-2 border-cyan-500/40">
          <div className="w-48 h-48 bg-slate-950 rounded flex flex-col items-center justify-center p-2 text-center text-white text-[10px] select-none">
            <div className="w-40 h-40 border border-cyan-400/40 flex flex-col items-center justify-center gap-1 p-2 bg-slate-900">
              <span className="text-cyan-400 font-bold text-xs">OXIDE P2P PAIR</span>
              <span className="text-slate-400 text-[9px] break-all">PK: {hostPk.slice(0, 16)}...</span>
              <div className="w-16 h-16 border-2 border-dashed border-cyan-500 flex items-center justify-center text-xs">
                [ QR ]
              </div>
              <span className="text-emerald-400 text-[9px]">X25519 E2EE</span>
            </div>
          </div>
        </div>

        <div className="flex flex-col gap-1.5">
          <label className="text-[11px] text-slate-400">Direct Pairing Link</label>
          <div className="flex gap-2">
            <input
              type="text"
              readOnly
              value={pairUrl}
              className="flex-1 bg-slate-950 border border-slate-800 rounded px-2.5 py-1.5 text-xs text-cyan-300 font-mono focus:outline-none"
            />
            <button
              onClick={handleCopy}
              className="px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded text-xs font-semibold"
            >
              {copied ? 'COPIED' : 'COPY'}
            </button>
          </div>
        </div>

        <div className="flex justify-between items-center pt-2 border-t border-slate-800 text-[11px] text-slate-500">
          <span>Security: DTLS 1.3 / ChaCha20</span>
          <button
            onClick={onClose}
            className="px-4 py-1.5 bg-cyan-700 hover:bg-cyan-600 text-white rounded text-xs font-bold"
          >
            DONE
          </button>
        </div>
      </div>
    </div>
  );
};
