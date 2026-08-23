import React, { useState } from 'react';
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
import { GraphTopologyTab } from './components/GraphTopologyTab';
import { HardwareClusterStatus } from './components/HardwareClusterStatus';

export default function App() {
  const [currentTab, setCurrentTab] = useState<TabId>('chat');
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const showToast = (msg: string) => {
    setToastMessage(msg);
    setTimeout(() => setToastMessage(null), 3000);
  };

  const handleNewSession = () => {
    setCurrentTab('chat');
    showToast('🦥 FastLanguageModel session initialized (sess_' + Math.random().toString(36).substring(2, 8) + ')');
  };

  const handleQuickDeploy = () => {
    showToast('🚀 Deploying Unsloth QDoRA & Model Soup to SGLang TP=2 (:8080)...');
  };

  return (
    <div className="min-h-screen bg-[#0b0c10] text-gray-100 flex flex-row antialiased selection:bg-orange-500/30 selection:text-orange-200 relative overflow-x-hidden font-sans">
      {/* Ambient background glows - Unsloth signature warm carbon atmosphere */}
      <div className="fixed top-[-10%] left-[-5%] w-[600px] h-[600px] bg-orange-600/10 rounded-full blur-[140px] pointer-events-none z-0" />
      <div className="fixed bottom-[-10%] right-[-5%] w-[550px] h-[550px] bg-amber-500/8 rounded-full blur-[130px] pointer-events-none z-0" />
      <div className="fixed top-[45%] right-[25%] w-[450px] h-[450px] bg-orange-950/15 rounded-full blur-[150px] pointer-events-none z-0" />

      {/* Sidebar */}
      <Sidebar currentTab={currentTab} onSelectTab={setCurrentTab} />

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col min-w-0 z-10 relative">
        <Header
          currentTab={currentTab}
          onSelectTab={setCurrentTab}
          onNewSession={handleNewSession}
          onQuickDeploy={handleQuickDeploy}
        />

        <main className="flex-1 p-4 md:p-6 max-w-[1720px] w-full mx-auto">
          {currentTab === 'chat' && <ChatTab />}
          {currentTab === 'overview' && <OverviewTab onNavigateTab={setCurrentTab} />}
          {currentTab === 'graph' && <GraphTopologyTab />}
          {currentTab === 'infra' && <InfraTab />}
          {currentTab === 'grpc' && <GrpcBridgeTab />}
          {currentTab === 'rag' && <RagPipelineTab />}
          {currentTab === 'mcp' && <McpToolsTab />}
          {currentTab === 'catalog' && <CatalogTab />}
          {currentTab === 'dataset' && <DatasetRecipeTab />}
          {currentTab === 'training' && <TrainingTab />}
          {currentTab === 'soup' && <ModelSoupTab />}
          {currentTab === 'sglang' && <SglangTab />}
          {currentTab === 'endpoints' && <EndpointsTab />}
          {currentTab === 'verify' && <VerificationTab />}
        </main>

        {/* Real-time Hardware Telemetry & Cluster Status */}
        <HardwareClusterStatus />
      </div>

      {/* Action Toast */}
      {toastMessage && (
        <div className="fixed bottom-6 right-6 z-50 px-4 py-3 rounded-xl bg-[#13141c]/95 border border-orange-500/40 text-xs text-orange-200 shadow-[0_0_25px_rgba(249,115,22,0.3)] backdrop-blur-xl flex items-center gap-2.5 animate-in fade-in slide-in-from-bottom-3 duration-200">
          <span className="w-2 h-2 rounded-full bg-orange-400 shadow-[0_0_8px_rgba(249,115,22,0.8)] animate-pulse" />
          <span className="font-mono text-[11px] font-medium">{toastMessage}</span>
        </div>
      )}
    </div>
  );
}
