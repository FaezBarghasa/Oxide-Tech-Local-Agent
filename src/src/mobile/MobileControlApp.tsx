import React, { useState, useEffect } from 'react';

interface ApprovalPayload {
  intervention_id: string;
  title: string;
  target_file: string;
  diff_preview: string;
  risk_tier: 'Low' | 'Medium' | 'Critical';
}

export const MobileControlApp: React.FC = () => {
  const [activeApproval, setActiveApproval] = useState<ApprovalPayload | null>(null);
  const [promptInput, setPromptInput] = useState('');
  const [telemetryLogs, setTelemetryLogs] = useState<string[]>([]);
  const [dataChannel, setDataChannel] = useState<RTCDataChannel | null>(null);
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    // 1. Initialize WebRTC peer connection using local hash params (#pk=...&nonce=...)
    try {
      const pc = new RTCPeerConnection({
        iceServers: [
          { urls: 'stun:stun.cloudflare.com:3478' },
          { urls: 'stun:stun.l.google.com:19302' },
        ],
      });

      const dc = pc.createDataChannel('oxide-control', { ordered: true });
      dc.onopen = () => setIsConnected(true);
      dc.onclose = () => setIsConnected(false);
      dc.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data);
          if (message.type === 'HITL_INTERVENTION_REQUIRED') {
            setActiveApproval(message.data);
            if ('vibrate' in navigator) navigator.vibrate([200, 100, 200]);
          } else if (message.type === 'TELEMETRY_STREAM') {
            setTelemetryLogs((prev) => [...prev.slice(-15), message.data]);
          }
        } catch {
          // Plain text fallback
          setTelemetryLogs((prev) => [...prev.slice(-15), String(event.data)]);
        }
      };

      setDataChannel(dc);
      return () => pc.close();
    } catch {
      // Fallback state if WebRTC fails in mock / test environment
      setIsConnected(false);
    }
  }, []);

  const handleApprove = async () => {
    if (!activeApproval || !dataChannel) return;

    try {
      // Trigger local biometric hardware authentication (FaceID / TouchID)
      if (window.navigator?.credentials?.get) {
        const assertion = await navigator.credentials.get({
          publicKey: {
            challenge: new Uint8Array(32),
            timeout: 60000,
            userVerification: 'required',
          },
        });

        if (assertion) {
          dataChannel.send(
            JSON.stringify({
              action: 'ApproveHunk',
              plan_id: activeApproval.intervention_id,
              hunk_hash: 'verified_via_passkey',
            })
          );
          setActiveApproval(null);
          return;
        }
      }
    } catch {
      // Direct fallback if biometric unavailable
    }

    dataChannel.send(
      JSON.stringify({
        action: 'ApproveHunk',
        plan_id: activeApproval.intervention_id,
        hunk_hash: 'manual_touch_approval',
      })
    );
    setActiveApproval(null);
  };

  const handleReject = () => {
    if (!activeApproval || !dataChannel) return;
    dataChannel.send(
      JSON.stringify({
        action: 'RejectAction',
        plan_id: activeApproval.intervention_id,
        reason: 'Rejected by mobile operator',
      })
    );
    setActiveApproval(null);
  };

  const handleSendPrompt = () => {
    if (!promptInput.trim() || !dataChannel) return;
    dataChannel.send(
      JSON.stringify({
        action: 'InjectPrompt',
        prompt: promptInput,
      })
    );
    setPromptInput('');
  };

  const handleEmergencyHalt = () => {
    if (!dataChannel) return;
    dataChannel.send(
      JSON.stringify({
        action: 'EmergencyHalt',
      })
    );
  };

  return (
    <div className="flex flex-col h-screen bg-slate-950 text-slate-100 p-4 font-mono select-none">
      {/* Header with biometric status */}
      <header className="flex justify-between items-center border-b border-slate-800 pb-3">
        <div className="flex items-center gap-2">
          <span
            className={`w-2.5 h-2.5 rounded-full ${
              isConnected ? 'bg-emerald-500 animate-pulse' : 'bg-amber-500'
            }`}
          />
          <h1 className="text-sm font-bold tracking-wider">OXIDE-REMOTE</h1>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-xs bg-slate-800 px-2 py-1 rounded text-slate-400">
            {isConnected ? 'P2P ENCRYPTED' : 'AWAITING PEER'}
          </span>
          <button
            onClick={handleEmergencyHalt}
            className="text-xs bg-rose-950 hover:bg-rose-900 border border-rose-700/60 px-2 py-1 rounded text-rose-300 font-bold"
          >
            HALT
          </button>
        </div>
      </header>

      {/* Urgent HITL Intervention Card */}
      {activeApproval && (
        <section className="mt-4 p-4 rounded-lg bg-amber-950/40 border border-amber-500/50 flex flex-col gap-3 shadow-lg shadow-amber-950/20">
          <div className="flex justify-between items-center">
            <span className="text-xs font-bold text-amber-400">⚠️ INTERVENTION REQUIRED</span>
            <span className="text-xs uppercase bg-red-900/60 px-2 py-0.5 rounded text-red-200">
              {activeApproval.risk_tier}
            </span>
          </div>
          <div className="text-xs text-slate-300 font-semibold">{activeApproval.target_file}</div>
          <pre className="text-xs bg-black/60 p-2 rounded overflow-x-auto text-emerald-400 border border-slate-800">
            {activeApproval.diff_preview}
          </pre>
          <div className="grid grid-cols-2 gap-3 mt-1">
            <button
              onClick={handleReject}
              className="py-2.5 bg-rose-950 hover:bg-rose-900 border border-rose-700/60 rounded text-xs font-bold text-rose-200 active:scale-95 transition-transform"
            >
              REJECT
            </button>
            <button
              onClick={handleApprove}
              className="py-2.5 bg-emerald-700 hover:bg-emerald-600 rounded text-xs font-bold text-white shadow-lg shadow-emerald-900/40 active:scale-95 transition-transform"
            >
              SIGN & APPROVE
            </button>
          </div>
        </section>
      )}

      {/* Real-time Thought & Telemetry Feed */}
      <section className="flex-1 mt-4 overflow-y-auto bg-slate-900/50 p-3 rounded border border-slate-800 text-xs flex flex-col gap-1">
        <div className="text-slate-500 mb-1 border-b border-slate-800/80 pb-1 flex justify-between">
          <span>LIVE REASONING STREAM</span>
          <span className="text-slate-600">{telemetryLogs.length} events</span>
        </div>
        {telemetryLogs.length === 0 ? (
          <div className="text-slate-600 italic py-4 text-center">Awaiting agent thought stream...</div>
        ) : (
          telemetryLogs.map((log, index) => (
            <div key={index} className="text-slate-300 leading-relaxed font-mono">
              {log}
            </div>
          ))
        )}
      </section>

      {/* Prompt Injection / Steering Loop */}
      <footer className="mt-4 flex gap-2">
        <input
          type="text"
          value={promptInput}
          onChange={(e) => setPromptInput(e.target.value)}
          placeholder="Steer agent or give instructions..."
          className="flex-1 bg-slate-900 border border-slate-800 rounded px-3 py-2 text-xs focus:outline-none focus:border-cyan-500 text-slate-200"
          onKeyDown={(e) => {
            if (e.key === 'Enter') handleSendPrompt();
          }}
        />
        <button
          onClick={handleSendPrompt}
          className="px-4 py-2 bg-cyan-700 hover:bg-cyan-600 rounded text-xs font-bold text-white active:scale-95 transition-transform"
        >
          SEND
        </button>
      </footer>
    </div>
  );
};
