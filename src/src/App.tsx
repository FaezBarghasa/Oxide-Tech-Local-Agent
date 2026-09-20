import React from 'react';
import { TabId } from './types';
import { Sidebar } from './components/Sidebar';
import { Header } from './components/Header';
import { ChatTab } from './components/ChatTab';
import { OverviewTab } from './components/OverviewTab';
import { InfraTab } from './components/InfraTab';
import { GrpcBridgeTab } from './components/GrpcBridgeTab';
import { RagPipelineTab } from './components/RagPipelineTab';
import { McpToolsTab } from './components/McpToolsTab';
import { CatalogTab } from './components/CatalogTab';
import { DatasetRecipeTab } from './components/DatasetRecipeTab';
import { TrainingTab } from './components/TrainingTab';
import { ModelSoupTab } from './components/ModelSoupTab';
import { SglangTab } from './components/SglangTab';
import { VerificationTab } from './components/VerificationTab';
import { KnowledgeGraphTab } from './components/KnowledgeGraphTab';
import { MemoryTab } from './components/MemoryTab';
import { GraphTopologyTab } from './components/GraphTopologyTab';
import { DoctorTab } from './components/DoctorTab';
import { ReForgeTab } from './components/ReForgeTab';
import { SettingsTab } from './components/SettingsTab';
import { StatusBar } from './components/StatusBar';
import { CommandPalette } from './components/CommandPalette';
import { DeploySlideOver } from './components/DeploySlideOver';
import { GatewayTab } from './components/GatewayTab';
import { UIProvider, useUI } from './store/uiStore';

function Shell() {
  const { tab, setTab, toasts, toast, setDeployModel, setPalette, toggleSidebar } = useUI();
  const renderTab = () => {
    switch (tab as TabId) {
      case 'chat': return <ChatTab />;
      case 'overview': return <OverviewTab onNavigateTab={(t) => setTab(t as TabId)} />;
      case 'infra': return <InfraTab />;
      case 'grpc': return <GrpcBridgeTab />;
      case 'rag': return <RagPipelineTab />;
      case 'mcp': return <McpToolsTab />;
      case 'catalog': return <CatalogTab />;
      case 'dataset': return <DatasetRecipeTab />;
      case 'training': return <TrainingTab />;
      case 'soup': return <ModelSoupTab />;
      case 'sglang': return <SglangTab />;
      case 'endpoints': return <GatewayTab notify={toast} />;
      case 'verify': return <VerificationTab />;
      case 'doctor': return <DoctorTab />;
      case 'reforge': return <ReForgeTab />;
      case 'settings': return <SettingsTab />;
      case 'graph': return <GraphTopologyTab />;
      case 'memory': return <MemoryTab />;
      default: return <ChatTab />;
    }
  };
  const onPalette = (id: string) => {
    if (id === 'deploy') setDeployModel('Qwen3-8B');
    else if (id === 'gateway') setTab('endpoints' as TabId);
    else if (id === 'playground') setTab('chat' as TabId);
    else if (id === 'dataset') setTab('dataset' as TabId);
    else if (id === 'theme') toggleSidebar();
    else toast(`Action: ${id}`);
  };
  return (
    <div className="h-screen bg-[#0A0A0A] text-[#FAFAFA] flex flex-row antialiased overflow-hidden font-sans">
      <Sidebar currentTab={tab} onSelectTab={setTab} />
      <div className="flex-1 flex flex-col min-w-0">
        <Header currentTab={tab} onSelectTab={setTab} onNewSession={() => { setTab('chat' as TabId); toast('Session initialized'); }} onQuickDeploy={() => setDeployModel('Qwen3-8B')} />
        <main className="flex-1 overflow-y-auto p-4 md:p-6 max-w-[1720px] w-full mx-auto">{renderTab()}</main>
        <StatusBar />
      </div>
      <CommandPalette onAction={onPalette} />
      <DeploySlideOver />
      <div className="fixed bottom-12 right-6 z-[70] flex flex-col gap-2 items-end" aria-live="polite">
        {toasts.map((t) => (
          <div key={t.id} className="px-4 py-2.5 rounded-lg bg-[#111113] border border-[#27272A] text-xs text-zinc-200 shadow-xl flex items-center gap-2">
            <span className="w-1.5 h-1.5 rounded-full bg-[#10B981]" />{t.msg}
          </div>
        ))}
      </div>
      <button onClick={() => setPalette(true)} aria-label="open command palette" className="sr-only">palette</button>
    </div>
  );
}

export default function App() {
  return <UIProvider initialTab={'chat' as TabId}><Shell /></UIProvider>;
}
