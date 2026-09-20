import React, { useState } from 'react';
import { SkeletonRows } from './Skeleton';
export function GatewayTab({notify}:{notify:(m:string)=>void}) {
  const [on,setOn]=useState(true); const [keys,setKeys]=useState([{name:'jan-local',prefix:'oxk_9f2a',limit:'1000/h',created:'2026-09-18'}]);
  const [loading,setLoading]=useState(false); const [tunnel,setTunnel]=useState<string|null>(null); const [reveal,setReveal]=useState(false);
  const gen=()=>{ setLoading(true); setTimeout(()=>{ setKeys(k=>[{name:`agent-${k.length+1}`,prefix:'oxk_'+Math.random().toString(16).slice(2,6),limit:'1000/h',created:'2026-09-20'},...k]); setLoading(false); notify('API key generated'); },600); };
  return <div className="flex flex-col gap-5 max-w-4xl">
    <div className="rounded-xl surface-card backdrop-blur-md border border-white/10 p-6 flex items-center justify-between">
      <div><h3 className="text-sm font-semibold text-[#FAFAFA] font-display">API Gateway</h3><p className="text-xs text-[#A1A1AA] font-mono mt-1">OpenAI-compatible · localhost:8080</p></div>
      <button role="switch" aria-checked={on} onClick={()=>{setOn(v=>!v);notify(`Gateway ${!on?'enabled':'disabled'}`);}} className={`w-12 h-7 rounded-full transition relative ${on?'bg-[#10B981]':'bg-zinc-700'}`}><span className={`absolute top-1 w-5 h-5 rounded-full bg-white transition-all ${on?'left-6':'left-1'}`}/></button>
    </div>
    <div className="rounded-xl surface-card backdrop-blur-md border border-white/10 p-6">
      <div className="flex items-center justify-between mb-4"><h3 className="text-sm font-semibold text-[#FAFAFA] font-display">API Keys</h3><button onClick={gen} className="px-3 py-1.5 rounded-md bg-[#FAFAFA] text-black text-xs font-semibold hover:-translate-y-px transition">Generate Key</button></div>
      {loading&&<SkeletonRows rows={1}/>}
      <table className="w-full text-xs font-mono"><thead><tr className="text-zinc-500 text-left"><th className="py-2">Name</th><th>Prefix</th><th>Limit</th><th>Created</th></tr></thead>
      <tbody>{keys.map(k=><tr key={k.prefix} className="border-t border-white/10 text-zinc-300"><td className="py-2.5">{k.name}</td><td>{k.prefix}…</td><td>{k.limit}</td><td>{k.created}</td></tr>)}</tbody></table>
      <div className="mt-4 rounded-md border border-[#a855f7]/40 bg-[#a855f7]/10 p-3 text-xs font-mono text-zinc-200 flex items-center justify-between">
        <span className={reveal?'':'blur-sm select-none'} onMouseEnter={()=>setReveal(true)}>{reveal?'oxk_9f2ab71de0042 secret — hover to reveal':'oxk_•••••••• hover to reveal'}</span>
        <button onClick={()=>{navigator.clipboard?.writeText('oxk_9f2ab71de0042');notify('API key copied');}} className="px-2 py-1 rounded bg-white/10 hover:bg-white/20">Copy</button></div>
    </div>
    <div className="rounded-xl surface-card backdrop-blur-md border border-white/10 p-6">
      <h3 className="text-sm font-semibold text-[#FAFAFA] font-display">Connected Clients</h3>
      <table className="w-full text-xs font-mono mt-3"><tbody>
        {[['192.168.1.20','Jan-App','12,408'],['192.168.1.34','Goose-Agent','3,891']].map(([ip,ua,tok])=><tr key={ip} className="border-t border-white/10 text-zinc-300"><td className="py-2">{ip}</td><td>{ua}</td><td className="text-right">{tok} tok</td></tr>)}
      </tbody></table>
    </div>
    <div className="rounded-xl surface-card backdrop-blur-md border border-white/10 p-6 flex items-center justify-between">
      <div><h3 className="text-sm font-semibold text-[#FAFAFA] font-display">Cloudflare Tunnel</h3><p className="text-xs font-mono text-zinc-400 mt-1">{tunnel??'Expose gateway to the internet'}</p></div>
      <button onClick={()=>{ if(tunnel){setTunnel(null);} else {setTunnel('https://oxide-7f2a.trycloudflare.com'); notify('Tunnel started');} }} className="px-3 py-1.5 rounded-md border border-white/10 text-xs hover:border-zinc-400 transition">{tunnel?'Stop':'Expose to Internet'}</button>
    </div>
    {tunnel&&<button onClick={()=>{navigator.clipboard?.writeText(tunnel);notify('Tunnel URL copied');}} className="font-mono text-xs text-[#a855f7] hover:underline text-left">{tunnel} — Copy</button>}
  </div>;
}
