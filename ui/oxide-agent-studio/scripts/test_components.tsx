import React from 'react';
import { renderToString } from 'react-dom/server';

// Import all tabs and components
import App from '../src/App';
import { Header } from '../src/components/Header';
import { Sidebar } from '../src/components/Sidebar';
import { ChatTab } from '../src/components/ChatTab';
import { OverviewTab } from '../src/components/OverviewTab';
import { GraphTopologyTab } from '../src/components/GraphTopologyTab';
import { MemoryTab } from '../src/components/MemoryTab';
import { InfraTab } from '../src/components/InfraTab';
import { GrpcBridgeTab } from '../src/components/GrpcBridgeTab';
import { RagPipelineTab } from '../src/components/RagPipelineTab';
import { McpToolsTab } from '../src/components/McpToolsTab';
import { CatalogTab } from '../src/components/CatalogTab';
import { DatasetRecipeTab } from '../src/components/DatasetRecipeTab';
import { TrainingTab } from '../src/components/TrainingTab';
import { ModelSoupTab } from '../src/components/ModelSoupTab';
import { SglangTab } from '../src/components/SglangTab';
import { EndpointsTab } from '../src/components/EndpointsTab';
import { VerificationTab } from '../src/components/VerificationTab';
import { DoctorTab } from '../src/components/DoctorTab';
import { ReForgeTab } from '../src/components/ReForgeTab';
import { SettingsTab } from '../src/components/SettingsTab';
import { HardwareClusterStatus } from '../src/components/HardwareClusterStatus';
import { HitlApprovalModal } from '../src/components/HitlApprovalModal';

interface TestResult {
  name: string;
  passed: boolean;
  htmlLength: number;
  checks: { name: string; found: boolean }[];
  error?: string;
}

const results: TestResult[] = [];

function testComponent(
  name: string,
  element: React.ReactElement,
  checks: { name: string; pattern: string | RegExp }[]
) {
  try {
    const html = renderToString(element);
    const checkResults = checks.map((c) => ({
      name: c.name,
      found: typeof c.pattern === 'string' ? html.includes(c.pattern) : c.pattern.test(html),
    }));

    const allPassed = checkResults.every((c) => c.found);
    results.push({
      name,
      passed: allPassed,
      htmlLength: html.length,
      checks: checkResults,
    });
  } catch (err: any) {
    results.push({
      name,
      passed: false,
      htmlLength: 0,
      checks: [],
      error: err?.message || String(err),
    });
  }
}

console.log('\x1b[1;34m====================================================================\x1b[0m');
console.log('\x1b[1;36m   Oxide Agent Studio — Comprehensive UI Component & View Test      \x1b[0m');
console.log('\x1b[1;34m====================================================================\x1b[0m\n');

// 1. Full App & Layout
testComponent('App Layout & Ambient Glows', <App />, [
  { name: 'Warm carbon background glow', pattern: 'bg-orange-600/10' },
  { name: 'Sidebar rendered', pattern: 'AI Assistant' },
  { name: 'Hardware Cluster status bar', pattern: 'TP=2 CLUSTER' },
]);

// 2. Navigation & Header
testComponent('Header Bar', <Header currentTab="chat" onSelectTab={() => {}} onNewSession={() => {}} onQuickDeploy={() => {}} />, [
  { name: 'Deploy action button', pattern: 'Deploy' },
  { name: 'New Session action button', pattern: 'New Session' },
  { name: 'Latency & health indicator', pattern: 'ms' },
]);

testComponent('Sidebar Nav Matrix', <Sidebar currentTab="chat" onSelectTab={() => {}} />, [
  { name: 'AI Assistant tab button', pattern: 'AI Assistant' },
  { name: 'Overview tab button', pattern: 'Overview &amp; Plan' },
  { name: 'Doctor tab button', pattern: 'Doctor &amp; System' },
  { name: 'RE-Forge tab button', pattern: 'RE-Forge Studio' },
  { name: 'Memory tab button', pattern: 'Project Memory' },
  { name: 'Settings tab button', pattern: 'Settings &amp; Config' },
]);

// 3. Specialized Studio Tabs
testComponent('ChatTab', <ChatTab />, [
  { name: 'Initial assistant greeting', pattern: 'oxide-agent-studio' },
  { name: 'SGLang runtime reference', pattern: 'SGLang TP=2 runtime' },
  { name: 'Tree-Sitter AST context', pattern: 'Tree-Sitter AST' },
]);

testComponent('OverviewTab', <OverviewTab onNavigateTab={() => {}} />, [
  { name: 'Plan milestones', pattern: 'Phase' },
  { name: 'Telemetry panels', pattern: 'Dual RTX 3090' },
]);

testComponent('MemoryTab (oxide-embed)', <MemoryTab />, [
  { name: 'STAIR Code-ToC search header', pattern: 'oxide-embed' },
  { name: 'Remember assertion section', pattern: 'Remember assertion' },
  { name: 'Context packing budget input', pattern: 'Pack context' },
  { name: 'Console log output', pattern: 'console' },
]);

testComponent('ReForgeTab', <ReForgeTab />, [
  { name: 'File path input', pattern: 'target/release/oxide-tech-local-agent' },
  { name: 'Analyze Binary button', pattern: 'Analyze' },
  { name: 'Architecture selector', pattern: 'auto' },
  { name: 'Decompile checkbox', pattern: 'checkbox' },
]);

testComponent('DoctorTab', <DoctorTab />, [
  { name: 'Diagnostics header', pattern: 'Doctor' },
  { name: 'Run Diagnostics button', pattern: 'Run Diagnostics' },
]);

testComponent('VerificationTab', <VerificationTab />, [
  { name: 'Run Verification Suite button', pattern: 'Run Verification Suite' },
  { name: 'Workspace target path input', pattern: 'Workspace Target Path' },
]);

testComponent('HardwareClusterStatus', <HardwareClusterStatus />, [
  { name: 'Dual GPU VRAM status', pattern: 'Dual RTX 3090' },
  { name: 'TP=2 cluster indicator', pattern: 'TP=2' },
  { name: 'Expand/collapse chevron', pattern: 'button' },
]);

testComponent('SettingsTab', <SettingsTab />, [
  { name: 'Operating profiles (lite/standard/pro)', pattern: 'Profile' },
  { name: 'Save configuration button', pattern: 'Save' },
]);

testComponent('GraphTopologyTab', <GraphTopologyTab />, [
  { name: 'Graph canvas area', pattern: 'svg' },
]);

testComponent('InfraTab', <InfraTab />, [
  { name: 'Daemons and services status', pattern: 'SurrealDB' },
]);

testComponent('GrpcBridgeTab', <GrpcBridgeTab />, [
  { name: 'Bridge endpoints table', pattern: 'gRPC' },
]);

testComponent('RagPipelineTab', <RagPipelineTab />, [
  { name: 'Hybrid search metrics', pattern: 'AST' },
]);

testComponent('McpToolsTab', <McpToolsTab />, [
  { name: 'MCP Tool inventory', pattern: 'MCP' },
]);

testComponent('CatalogTab', <CatalogTab />, [
  { name: 'Model catalog list', pattern: 'AWQ' },
]);

testComponent('DatasetRecipeTab', <DatasetRecipeTab />, [
  { name: 'Token recipe builder', pattern: 'Dataset' },
]);

testComponent('TrainingTab', <TrainingTab />, [
  { name: 'LoRA fine-tuning tracker', pattern: 'LoRA' },
]);

testComponent('ModelSoupTab', <ModelSoupTab />, [
  { name: 'SLERP weight slider', pattern: 'Soup' },
]);

testComponent('SglangTab', <SglangTab />, [
  { name: 'SGLang runtime metrics', pattern: 'SGLang' },
]);

testComponent('EndpointsTab', <EndpointsTab />, [
  { name: 'Active endpoints list', pattern: 'Runner' },
]);

testComponent(
  'HitlApprovalModal',
  (
    <HitlApprovalModal
      requests={[
        {
          id: 'hitl-001',
          timestamp: '10:00:00',
          title: 'Flash STM32 Firmware',
          subsystem: 'probe-rs',
          impactLevel: 'CRITICAL',
          description: 'Flashing firmware to STM32F401 target',
          parameters: { chip: 'STM32F401RE' },
          verificationPassed: true,
          status: 'PENDING',
        },
      ]}
      onApprove={() => {}}
      onReject={() => {}}
    />
  ),
  [
    { name: 'Modal title', pattern: 'Flash STM32 Firmware' },
    { name: 'Approve button', pattern: 'Approve' },
    { name: 'Reject button', pattern: 'Reject' },
  ]
);

// Print Results Table
let totalPassed = 0;
let totalFailed = 0;

for (const r of results) {
  if (r.passed) {
    totalPassed++;
    console.log(`  \x1b[1;32m[PASS]\x1b[0m \x1b[1m${r.name.padEnd(35)}\x1b[0m (${r.htmlLength} bytes rendered)`);
    for (const c of r.checks) {
      console.log(`         \x1b[32m✓\x1b[0m ${c.name}`);
    }
  } else {
    totalFailed++;
    console.log(`  \x1b[1;31m[FAIL]\x1b[0m \x1b[1m${r.name.padEnd(35)}\x1b[0m ${r.error ? `(${r.error})` : ''}`);
    for (const c of r.checks) {
      console.log(`         ${c.found ? '\x1b[32m✓\x1b[0m' : '\x1b[31m✗\x1b[0m'} ${c.name}`);
    }
  }
}

console.log('\n\x1b[1;34m--------------------------------------------------------------------\x1b[0m');
console.log(`Summary: \x1b[1;32m${totalPassed} passed\x1b[0m, \x1b[1;31m${totalFailed} failed\x1b[0m across ${results.length} UI components.`);

if (totalFailed > 0) {
  process.exit(1);
} else {
  console.log('\x1b[1;32m[✓] All studio tabs, buttons, menus, inputs, modals, and layouts verified!\x1b[0m\n');
}
