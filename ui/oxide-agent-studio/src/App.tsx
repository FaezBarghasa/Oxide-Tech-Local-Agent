import React, { useState, useCallback } from 'react';
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
import { EndpointsTab } from './components/EndpointsTab';
import { VerificationTab } from './components/VerificationTab';
import { KnowledgeGraphTab } from './components/KnowledgeGraphTab';
import { MemoryTab } from './components/MemoryTab';
import { GraphTopologyTab } from './components/GraphTopologyTab';
import { HardwareClusterStatus } from './components/HardwareClusterStatus';
import { DoctorTab } from './components/DoctorTab';
import { ReForgeTab } from './components/ReForgeTab';
import { SettingsTab } from './components/SettingsTab';

export default function App() {
  const [currentTab, setCurrentTab] = useState<TabId>('chat');
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const showToast = useCallback((msg: string) => {
    setToastMessage(msg);
    const timer = setTimeout(() => setToastMessage(null), 3000);
    return () => clearTimeout(timer);
  }, []);

  const handleNewSession = useCallback(() => {
    setCurrentTab('chat');
    showToast('Session initialized');
  }, [showToast]);

  const handleQuickDeploy = useCallback(() => {
    showToast('Deploy initiated');
  }, [showToast]);

  const renderTab = () => {
    switch (currentTab) {
      case 'chat': return <ChatTab />;
      case 'overview': return <OverviewTab onNavigateTab={(t) => setCurrentTab(t as TabId)} />;
      case 'infra': return <InfraTab />;
      case 'grpc': return <GrpcBridgeTab />;
      case 'rag': return <RagPipelineTab />;
      case 'mcp': return <McpToolsTab />;
      case 'catalog': return <CatalogTab />;
      case 'dataset': return <DatasetRecipeTab />;
      case 'training': return <TrainingTab />;
      case 'soup': return <ModelSoupTab />;
      case 'sglang': return <SglangTab />;
      case 'endpoints': return <EndpointsTab />;
      case 'verify': return <VerificationTab />;
      case 'doctor': return <DoctorTab />;
      case 'reforge': return <ReForgeTab />;
      case 'settings': return <SettingsTab />;
      case 'graph': return <GraphTopologyTab />;
      case 'memory': return <MemoryTab />;
      default: return <ChatTab />;
    }
  };

  return (
    <div className="min-h-screen bg-[#09090b] text-gray-100 flex flex-row antialiased selection:bg-amber-500/20 selection:text-amber-200 relative overflow-x-hidden font-sans">
      <Sidebar currentTab={currentTab} onSelectTab={setCurrentTab} />

      <div className="flex-1 flex flex-col min-w-0 z-10 relative">
        <Header
          currentTab={currentTab}
          onSelectTab={setCurrentTab}
          onNewSession={handleNewSession}
          onQuickDeploy={handleQuickDeploy}
        />

        <main className="flex-1 p-4 md:p-6 max-w-[1720px] w-full mx-auto">
          {renderTab()}
        </main>
      </div>

      {/* Ambient atmosphere - restrained, low-contrast */}
      <div className="fixed top-[-10%] left-[-5%] w-[600px] h-[600px] bg-amber-500/[0.03] rounded-full blur-[140px] pointer-events-none z-0" />
      <div className="fixed bottom-[-10%] right-[-5%] w-[550px] h-[550px] bg-amber-600/[0.03] rounded-full blur-[130px] pointer-events-none z-0" />
      <div className="fixed top-[45%] right-[25%] w-[450px] h-[450px] bg-zinc-800/[0.05] rounded-full blur-[150px] pointer-events-none z-0" />

      {/* Toast */}
      {toastMessage && (
        <div className="fixed bottom-6 right-6 z-50 px-4 py-3 rounded-xl bg-[#18181b]/95 border border-white/[0.08] text-xs text-zinc-300 backdrop-blur-xl flex items-center gap-2.5 fade-in shadow-lg">
          <span className="w-1.5 h-1.5 rounded-full bg-amber-400" />
          {toastMessage}
        </div>
      )}
    </div>
  );
}
