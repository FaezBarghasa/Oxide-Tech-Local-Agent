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
import { MobileCompanionModal } from './components/MobileCompanionModal';
import { HitlApprovalModal, HitlActionRequest } from './components/HitlApprovalModal';
import { UIProvider, useUI } from './store/uiStore';

const INITIAL_HITL_REQUESTS: HitlActionRequest[] = [
  {
    id: 'hitl-001',
    timestamp: '11:42:09',
    title: 'Flash STM32F401 Bare-Metal Firmware (SWD Target)',
    subsystem: 'probe-rs',
    impactLevel: 'CRITICAL',
    description: 'Autonomous agent generated binary patch for RTIC v2 USB-MIDI endpoint. Requests automated erase and flash over ST-LINK v2 on thumbv7em-none-eabihf.',
    proposedCommand: 'probe-rs run --chip STM32F401CEUx target/thumbv7em-none-eabihf/release/app',
    targetPath: '/dev/bus/usb/001/004',
    parameters: {
      chip: 'STM32F401CEUx',
      speed_khz: 4000,
      reset_after_flash: true,
      verify: true,
    },
    verificationPassed: true,
    status: 'PENDING',
    reviewerScore: 0.94,
    reviewerVerdict: 'Formal memory map bounds check passed. Zero bootloader overwrite risk.',
    confidenceScore: 0.98,
  },
  {
    id: 'hitl-002',
    timestamp: '11:43:28',
    title: 'Sync KiCad PCB Netlist & Route Power Traces',
    subsystem: 'eda-kicad',
    impactLevel: 'HIGH',
    description: 'Agent calculated trace impedance for 12V 5A power bus on layer F.Cu. Netlist changes ready to merge into PCB layout.',
    targetPath: 'hardware/schematics/oxide_power_stage.kicad_pcb',
    parameters: {
      net: 'VBUS_12V',
      trace_width_mm: 1.5,
      clearance_mm: 0.35,
      copper_oz: 2,
    },
    verificationPassed: true,
    status: 'PENDING',
    reviewerScore: 0.89,
    reviewerVerdict: 'IPC-2152 thermal dissipation verified (ΔT < 10°C). DRC check 0 errors.',
    confidenceScore: 0.95,
  },
];

function Shell() {
  const {
    tab,
    setTab,
    toasts,
    toast,
    setDeployModel,
    mobileCompanionOpen,
    setMobileCompanionOpen,
    hitlOpen,
    setHitlOpen,
    setHitlPendingCount,
  } = useUI();
  const [hitlRequests, setHitlRequests] = React.useState<HitlActionRequest[]>(INITIAL_HITL_REQUESTS);

  React.useEffect(() => {
    const handleKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.shiftKey && e.key.toLowerCase() === 'h') {
        e.preventDefault();
        setHitlOpen(!hitlOpen);
      }
    };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [hitlOpen, setHitlOpen]);

  const handleHitlApprove = (id: string) => {
    setHitlRequests((prev) => prev.map((r) => (r.id === id ? { ...r, status: 'APPROVED' } : r)));
    setHitlPendingCount((prev) => Math.max(0, prev - 1));
    toast(`Authorized operation: ${id}`);
    setHitlOpen(false);
  };

  const handleHitlReject = (id: string, reason?: string) => {
    setHitlRequests((prev) => prev.map((r) => (r.id === id ? { ...r, status: 'REJECTED' } : r)));
    setHitlPendingCount((prev) => Math.max(0, prev - 1));
    toast(`Rejected operation: ${id}${reason ? ` (${reason})` : ''}`);
    setHitlOpen(false);
  };

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
      case 'pair-mobile':
        setMobileCompanionOpen(true);
        break;
      case 'hitl-gate':
        setHitlOpen(true);
        break;
      case 'nav-graph':
        setTab('graph');
        break;
      case 'nav-reforge':
        setTab('reforge');
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
      <MobileCompanionModal
        isOpen={mobileCompanionOpen}
        onClose={() => setMobileCompanionOpen(false)}
      />
      {hitlOpen && (
        <HitlApprovalModal
          requests={hitlRequests.filter((r) => r.status === 'PENDING')}
          onApprove={handleHitlApprove}
          onReject={handleHitlReject}
          onClose={() => setHitlOpen(false)}
        />
      )}

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
