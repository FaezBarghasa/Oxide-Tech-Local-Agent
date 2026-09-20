import React, { useEffect, useState } from 'react';
import { useUI } from '../store/uiStore';
const ACTIONS=[{id:'deploy',label:'Deploy model…'},{id:'gateway',label:'Go to Gateway / API keys'},{id:'playground',label:'Open Playground'},{id:'dataset',label:'Convert dataset…'},{id:'tunnel',label:'Start Cloudflare tunnel'},{id:'theme',label:'Toggle sidebar'}];
export function CommandPalette({onAction}:{onAction:(id:string)=>void}) {
  const {paletteOpen,setPalette}=useUI(); const [q,setQ]=useState('');
  useEffect(()=>{ const h=(e:KeyboardEvent)=>{ if((e.metaKey||e.ctrlKey)&&e.key.toLowerCase()==='k'){e.preventDefault();setPalette(!paletteOpen);} if(e.key==='Escape')setPalette(false); }; window.addEventListener('keydown',h); return ()=>window.removeEventListener('keydown',h); },[paletteOpen,setPalette]);
  useEffect(()=>{ if(!paletteOpen) setQ(''); },[paletteOpen]);
  if(!paletteOpen) return null;
  const list=ACTIONS.filter(a=>a.label.toLowerCase().includes(q.toLowerCase()));
  return <div className="fixed inset-0 z-[80] bg-black/60 backdrop-blur-sm flex justify-center pt-[15vh]" onClick={()=>setPalette(false)} role="dialog" aria-modal="true" aria-label="command palette">
    <div className="w-[480px] max-w-[90vw] h-fit rounded-xl surface-popover border-white/10 shadow-2xl overflow-hidden" onClick={e=>e.stopPropagation()}>
      <input autoFocus value={q} onChange={e=>setQ(e.target.value)} placeholder="Type a command or search… (Esc to close)" aria-label="command search" className="w-full bg-transparent px-4 py-3 text-sm text-zinc-100 outline-none border-b border-white/10 font-mono"/>
      <ul>{list.map(a=><li key={a.id}><button onClick={()=>{setPalette(false);onAction(a.id);}} className="w-full text-left px-4 py-2.5 text-sm text-zinc-300 hover:bg-white/[0.06] focus:bg-white/[0.06] focus:outline-none">{a.label}</button></li>)}
      {list.length===0&&<li className="px-4 py-3 text-xs text-zinc-500 font-mono">No matches</li>}</ul>
    </div></div>;
}
