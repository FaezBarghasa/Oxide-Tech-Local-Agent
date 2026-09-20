import React, { useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { useUI } from '../store/uiStore';
const ENGINES=['Candle','llama.cpp','Mistral.rs','vLLM (Docker)','SGLang'] as const;
const QUANTS=['Q4_K_M','Q5_K_M','Q8_0','FP16'] as const;
const VRAM:Record<string,number>={'Q4_K_M':6.2,'Q5_K_M':7.4,'Q8_0':9.8,'FP16':14.5};
export function DeploySlideOver() {
  const {deployModel,setDeployModel,toast}=useUI();
  const [engine,setEngine]=useState<typeof ENGINES[number]>('Candle'); const [quant,setQuant]=useState<typeof QUANTS[number]>('Q4_K_M');
  return <AnimatePresence>{deployModel&&<>
    <motion.div initial={{opacity:0}} animate={{opacity:1}} exit={{opacity:0}} className="fixed inset-0 z-[60] bg-black/50" onClick={()=>setDeployModel(null)}/>
    <motion.aside initial={{x:400}} animate={{x:0}} exit={{x:400}} transition={{type:'spring',damping:30,stiffness:300}} role="dialog" aria-modal="true" aria-label={`Deploy ${deployModel}`} className="fixed right-0 top-0 bottom-0 z-[61] w-[380px] max-w-[92vw] bg-[#121215]/95 backdrop-blur-xl border-l border-white/10 surface-modal p-6 flex flex-col gap-5">
      <div><h2 className="text-base font-semibold text-[#FAFAFA] font-display">Deploy {deployModel}</h2><p className="text-xs text-[#A1A1AA] mt-1">Engine + quantization. Progressive disclosure — defaults are safe.</p></div>
      <div><div className="text-[11px] font-mono uppercase tracking-widest text-zinc-500 mb-2">Engine</div>
        <div className="grid grid-cols-2 gap-2" role="radiogroup" aria-label="engine">{ENGINES.map(e=><button key={e} role="radio" aria-checked={engine===e} onClick={()=>setEngine(e)} className={`px-3 py-2 rounded-md border text-xs font-medium transition hover:-translate-y-px ${engine===e?'bg-[#FAFAFA] text-black border-transparent':'bg-white/[0.03] text-zinc-300 border-white/10 hover:border-zinc-500'}`}>{e}</button>)}</div></div>
      <div><div className="text-[11px] font-mono uppercase tracking-widest text-zinc-500 mb-2">Quantization</div>
        <div className="flex flex-col gap-1.5" role="radiogroup" aria-label="quantization">{QUANTS.map(q=><label key={q} className={`flex items-center gap-2.5 px-3 py-2 rounded-md border text-xs cursor-pointer transition ${quant===q?'border-[#10B981]/60 bg-[#10B981]/10':'border-white/10 hover:border-zinc-500'}`}><input type="radio" name="quant" checked={quant===q} onChange={()=>setQuant(q)} className="accent-[#10B981]"/>{q}</label>)}</div></div>
      <p className="text-xs font-mono text-zinc-400" aria-live="polite">Estimated VRAM: <b className="text-zinc-100">{VRAM[quant].toFixed(1)}GB</b> · {engine}</p>
      <button onClick={()=>{toast(`Deploy initiated: ${deployModel} · ${engine} · ${quant}`);setDeployModel(null);}} className="mt-auto w-full py-2.5 rounded-md bg-[#FAFAFA] text-black text-sm font-semibold hover:-translate-y-px transition focus:outline-none focus-visible:ring-2 focus-visible:ring-[#8B5CF6]">Deploy Model</button>
      <button onClick={()=>setDeployModel(null)} className="text-xs text-zinc-500 hover:text-zinc-300">Cancel (Esc)</button>
    </motion.aside></>}</AnimatePresence>;
}
