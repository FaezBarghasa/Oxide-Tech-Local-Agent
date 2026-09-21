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
      case 'chat':
        return <ChatTab />;
      case 'overview':
        return <OverviewTab onNavigateTab={(t) => setTab(t as TabId)} />;
      case 'infra':
        return <InfraTab />;
      case 'grpc':
        return <GrpcBridgeTab />;
      case 'rag':
        return <RagPipelineTab />;
      case 'mcp':
        return <McpToolsTab />;
      case 'catalog':
        return <CatalogTab />;
      case 'dataset':
        return <DatasetRecipeTab />;
      case 'training':
        return <TrainingTab />;
      case 'soup':
        return <ModelSoupTab />;
      case 'sglang':
        return <SglangTab />;
      case 'endpoints':
        return <GatewayTab notify={toast} />;
      case 'verify':
        return <VerificationTab />;
      case 'doctor':
        return <DoctorTab />;
      case 'reforge':
        return <ReForgeTab />;
      case 'settings':
        return <SettingsTab />;
      case 'graph':
        return <GraphTopologyTab />;
      case 'memory':
        return <MemoryTab />;
      default:
        return <ChatTab />;
    }
  };

  const onPalette = (id: string) => {
    switch (id) {
      case 'deploy':
        setDeployModel('Qwen3-8B');
        break;
      case 'gen-key':
      case 'tunnel':
      case 'nav-gateway':
        setTab('endpoints');
        toast('Navigated to API Gateway');
        break;
      case 'dataset':
        setTab('dataset');
        toast('Opened Dataset Formatter');
        break;
      case 'export-gguf':
        setTab('dataset');
        toast('Opened GGUF Exporter');
        break;
      case 'compact-memory':
        setTab('dataset');
        toast('Triggered STAIR Context Compaction');
        break;
      case 'nav-overview':
        setTab('overview');
        break;
      case 'nav-chat':
        setTab('chat');
        break;
      case 'nav-catalog':
        setTab('catalog');
        break;
      case 'nav-engines':
        setTab('sglang');
        break;
      case 'nav-mcp':
        setTab('mcp');
        break;
      case 'nav-doctor':
        setTab('doctor');
        break;
      case 'nav-settings':
        setTab('settings');
        break;
      default:
        toast(`Executed: ${id}`);
    }
  };

  return (
    <div className="h-screen bg-[#0A0A0A] text-[#FAFAFA] flex flex-row antialiased overflow-hidden font-sans select-none">
      <Sidebar currentTab={tab} onSelectTab={setTab} />
      <div className="flex-1 flex flex-col min-w-0">
        <Header
          currentTab={tab}
          onSelectTab={setTab}
          onNewSession={() => {
            setTab('chat' as TabId);
            toast('Session initialized');
          }}
          onQuickDeploy={() => setDeployModel('Qwen3-8B')}
        />
        <main className="flex-1 overflow-y-auto p-4 md:p-6 max-w-[1720px] w-full mx-auto scrollbar-thin">
          {renderTab()}
        </main>
        <StatusBar />
      </div>
      <CommandPalette onAction={onPalette} />
      <DeploySlideOver />

      {/* Floating Toast Notification Stack */}
      <div className="fixed bottom-12 right-6 z-[70] flex flex-col gap-2 items-end pointer-events-none" aria-live="polite">
        {toasts.map((t) => (
          <div
            key={t.id}
            className="px-4 py-2.5 rounded-lg bg-[#111113] border border-[#27272A] text-xs text-zinc-200 shadow-xl flex items-center gap-2 pointer-events-auto animate-slide-up"
          >
            <span className="w-1.5 h-1.5 rounded-full bg-[#10B981]" />
            {t.msg}
          </div>
        ))}
      </div>
    </div>
  );
}

export default function App() {
  return (
    <UIProvider initialTab={'overview' as TabId}>
      <Shell />
    </UIProvider>
  );
}
