use crate::agentic_loop::{AgenticLoopRunner, ToolExecutor};
use crate::provider::{
    ChatMessage, ChatRequest, InferenceProvider, ToolDefinition,
};
use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

// ── Agent Role ────────────────────────────────────────────────────────────────

/// Logical role a registered agent plays in the multi-agent system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentRole {
    /// Receives the initial user task, decomposes it, delegates to workers.
    Supervisor,
    /// Specialised task executor (coder, researcher, planner, etc.)
    Worker,
    /// Reviews and scores a worker's output; may request revisions.
    Verifier,
    /// Aggregates results from multiple agents into a final answer.
    Aggregator,
    /// Peer agent — can initiate and respond to any other agent.
    Peer,
    /// Internal coordinator acting on behalf of the system (e.g., aggregation step).
    Coordinator,
}

impl AgentRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Supervisor => "supervisor",
            Self::Worker => "worker",
            Self::Verifier => "verifier",
            Self::Aggregator => "aggregator",
            Self::Peer => "peer",
            Self::Coordinator => "coordinator",
        }
    }
}

/// Result of a peer dialogue execution, including consensus evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDialogueResult {
    pub final_content: String,
    pub thread_id: String,
    pub rounds_completed: usize,
    pub consensus_reached: bool,
    pub consensus_score: f32,
    pub dtx_id: Option<String>,
}

// ── Inter-Agent Message ───────────────────────────────────────────────────────

/// A message sent from one agent to another within the coordinator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Unique ID for tracing this message through the pipeline.
    pub id: String,
    /// ID of the sending agent (`None` if originated from the user).
    pub from_agent: Option<String>,
    /// ID of the receiving agent. `None` means broadcast to all.
    pub to_agent: Option<String>,
    /// The role context the sender is operating under.
    pub sender_role: AgentRole,
    /// Content of the message.
    pub content: String,
    /// Optional thread ID for grouping messages in a conversation chain.
    pub thread_id: Option<String>,
    /// Optional Oxide DTX transaction ID for distributed audit tracing.
    pub dtx_id: Option<String>,
    /// Optional structured payload (tool output, JSON result, etc.)
    pub payload: Option<serde_json::Value>,
}

impl AgentMessage {
    pub fn new(
        from: impl Into<String>,
        to: impl Into<String>,
        role: AgentRole,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            from_agent: Some(from.into()),
            to_agent: Some(to.into()),
            sender_role: role,
            content: content.into(),
            thread_id: None,
            dtx_id: None,
            payload: None,
        }
    }

    pub fn with_thread(mut self, thread_id: impl Into<String>) -> Self {
        self.thread_id = Some(thread_id.into());
        self
    }

    pub fn with_dtx(mut self, dtx_id: impl Into<String>) -> Self {
        self.dtx_id = Some(dtx_id.into());
        self
    }

    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = Some(payload);
        self
    }
}

// ── Agent Record ──────────────────────────────────────────────────────────────

/// A registered agent within the multi-agent coordinator.
pub struct AgentRecord {
    pub id: String,
    pub role: AgentRole,
    pub system_prompt: String,
    pub provider: Arc<dyn InferenceProvider>,
    pub tool_executor: Option<Arc<dyn ToolExecutor>>,
    pub tool_definitions: Vec<ToolDefinition>,
    pub max_turns: usize,
}

impl AgentRecord {
    /// Build an `AgenticLoopRunner` for this agent on demand.
    pub fn build_loop_runner(&self) -> Option<AgenticLoopRunner> {
        self.tool_executor.as_ref().map(|exec| {
            AgenticLoopRunner::new(
                Arc::clone(&self.provider),
                Arc::clone(exec),
                self.tool_definitions.clone(),
                self.max_turns,
            )
        })
    }

    /// Run a single non-agentic completion (no tool loop).
    pub async fn complete(&self, messages: Vec<ChatMessage>) -> Result<String> {
        let req = ChatRequest {
            model: String::new(),
            messages,
            temperature: Some(0.1),
            max_tokens: Some(4096),
            json_mode: None,
            history: vec![],
            tools: None,
            tool_choice: None,
            images: None,
            grammar: None,
            stop: None,
            slot_id: None,
        };
        let resp = self.provider.chat_completion(req).await?;
        Ok(resp.content)
    }
}

// ── Conversation Thread ───────────────────────────────────────────────────────

/// Running record of messages exchanged in a multi-agent conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentThread {
    pub id: String,
    pub messages: Vec<AgentMessage>,
    pub final_answer: Option<String>,
}

impl AgentThread {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            messages: Vec::new(),
            final_answer: None,
        }
    }

    pub fn push(&mut self, msg: AgentMessage) {
        self.messages.push(msg);
    }

    /// Build a simple `Vec<ChatMessage>` for a specific recipient (their view).
    pub fn as_chat_history_for(&self, agent_id: &str) -> Vec<ChatMessage> {
        self.messages
            .iter()
            .filter(|m| {
                m.to_agent.as_deref() == Some(agent_id)
                    || m.from_agent.as_deref() == Some(agent_id)
                    || m.to_agent.is_none() // broadcast
            })
            .map(|m| ChatMessage {
                role: if m.from_agent.as_deref() == Some(agent_id) {
                    "assistant".to_string()
                } else {
                    "user".to_string()
                },
                content: m.content.clone(),
            })
            .collect()
    }
}

// ── Multi-Agent Coordinator ───────────────────────────────────────────────────

/// Central coordinator for multi-agent message passing and orchestration.
///
/// # Topology patterns supported
///
/// - **Supervisor → Worker(s) → Verifier**: Supervisor decomposes task, Workers
///   execute in parallel, Verifier scores and requests revisions.
/// - **Chain**: A → B → C → final, each agent's output becomes the next input.
/// - **Peer dialogue**: Any two agents exchange messages in a shared thread.
/// - **Broadcast**: One agent sends to all others simultaneously.
pub struct MultiAgentCoordinator {
    agents: Arc<RwLock<HashMap<String, AgentRecord>>>,
    threads: Arc<RwLock<HashMap<String, AgentThread>>>,
    event_bus: tokio::sync::broadcast::Sender<AgentMessage>,
}

impl Default for MultiAgentCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiAgentCoordinator {
    pub fn new() -> Self {
        let (event_bus, _) = tokio::sync::broadcast::channel(256);
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            threads: Arc::new(RwLock::new(HashMap::new())),
            event_bus,
        }
    }

    /// Subscribe to the live EventBus stream of all inter-agent messages.
    pub fn subscribe_events(&self) -> tokio::sync::broadcast::Receiver<AgentMessage> {
        self.event_bus.subscribe()
    }

    /// Register an agent with the coordinator.
    pub async fn register(&self, record: AgentRecord) {
        let mut agents = self.agents.write().await;
        info!(agent_id = %record.id, role = %record.role.as_str(), "Agent registered");
        agents.insert(record.id.clone(), record);
    }

    /// Create a new conversation thread and return its ID.
    pub async fn new_thread(&self) -> String {
        let thread_id = uuid::Uuid::new_v4().to_string();
        let mut threads = self.threads.write().await;
        threads.insert(thread_id.clone(), AgentThread::new(&thread_id));
        thread_id
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Core messaging primitives
    // ─────────────────────────────────────────────────────────────────────────

    /// Send a message to a specific agent and get its reply.
    ///
    /// The full thread history visible to the receiving agent is included.
    pub async fn send(
        &self,
        thread_id: &str,
        message: AgentMessage,
    ) -> Result<AgentMessage> {
        let to_agent_id = message
            .to_agent
            .clone()
            .ok_or_else(|| anyhow!("send() requires a specific to_agent; use broadcast() for all"))?;

        // Append the outgoing message to the thread
        {
            let mut threads = self.threads.write().await;
            let thread = threads
                .entry(thread_id.to_string())
                .or_insert_with(|| AgentThread::new(thread_id));
            thread.push(message.clone());
        }

        // Build history for the receiving agent
        let history = {
            let threads = self.threads.read().await;
            threads
                .get(thread_id)
                .map(|t| t.as_chat_history_for(&to_agent_id))
                .unwrap_or_default()
        };

        // Look up the agent
        let agents = self.agents.read().await;
        let agent = agents
            .get(&to_agent_id)
            .ok_or_else(|| anyhow!("Agent '{}' not registered", to_agent_id))?;

        // Build messages: system prompt + thread history + new content
        let mut messages = vec![ChatMessage {
            role: "system".to_string(),
            content: agent.system_prompt.clone(),
        }];
        messages.extend(history);
        // Ensure the latest message ends with the user turn
        if messages.last().map(|m| m.role.as_str()) != Some("user") {
            messages.push(ChatMessage {
                role: "user".to_string(),
                content: message.content.clone(),
            });
        }

        debug!(
            thread = %thread_id,
            from = ?message.from_agent,
            to = %to_agent_id,
            "Dispatching message"
        );

        let reply_content = if let Some(runner) = agent.build_loop_runner() {
            // Agentic tool-calling loop
            let (answer, _history) = runner
                .run(&agent.system_prompt, &message.content)
                .await?;
            answer
        } else {
            // Simple completion
            agent.complete(messages).await?
        };

        let reply = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            from_agent: Some(to_agent_id.clone()),
            to_agent: message.from_agent.clone(),
            sender_role: agent.role.clone(),
            content: reply_content.clone(),
            thread_id: Some(thread_id.to_string()),
            dtx_id: message.dtx_id.clone(),
            payload: None,
        };

        // Append reply to thread
        {
            let mut threads = self.threads.write().await;
            if let Some(thread) = threads.get_mut(thread_id) {
                thread.push(reply.clone());
            }
        }

        // Notify live event bus
        let _ = self.event_bus.send(message);
        let _ = self.event_bus.send(reply.clone());

        Ok(reply)
    }

    /// Broadcast a message to all agents and collect their replies in parallel.
    pub async fn broadcast(
        &self,
        thread_id: &str,
        from_agent: &str,
        from_role: AgentRole,
        content: &str,
    ) -> Result<Vec<AgentMessage>> {
        let agent_ids: Vec<String> = {
            let agents = self.agents.read().await;
            agents
                .keys()
                .filter(|id| id.as_str() != from_agent)
                .cloned()
                .collect()
        };

        let mut handles = Vec::new();
        let coord = Arc::new(self as *const Self as usize); // raw pointer workaround for async closures

        for agent_id in &agent_ids {
            let msg = AgentMessage::new(from_agent, agent_id, from_role.clone(), content)
                .with_thread(thread_id);
            let thread_id = thread_id.to_string();
            let agents_arc = Arc::clone(&self.agents);
            let threads_arc = Arc::clone(&self.threads);
            let agent_id = agent_id.clone();
            let msg_clone = msg.clone();

            // Spawn a task per agent
            handles.push(tokio::spawn(async move {
                // Append outgoing to thread
                {
                    let mut threads = threads_arc.write().await;
                    let thread = threads
                        .entry(thread_id.clone())
                        .or_insert_with(|| AgentThread::new(&thread_id));
                    thread.push(msg_clone.clone());
                }

                let agents = agents_arc.read().await;
                let agent = match agents.get(&agent_id) {
                    Some(a) => a,
                    None => return Err::<AgentMessage, anyhow::Error>(anyhow!("Agent not found")),
                };

                let history = {
                    let threads = threads_arc.read().await;
                    threads
                        .get(&thread_id)
                        .map(|t| t.as_chat_history_for(&agent_id))
                        .unwrap_or_default()
                };

                let mut messages = vec![ChatMessage {
                    role: "system".to_string(),
                    content: agent.system_prompt.clone(),
                }];
                messages.extend(history);
                messages.push(ChatMessage {
                    role: "user".to_string(),
                    content: msg_clone.content.clone(),
                });

                let reply_content = agent.complete(messages).await?;

                let reply = AgentMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    from_agent: Some(agent_id.clone()),
                    to_agent: msg_clone.from_agent.clone(),
                    sender_role: agent.role.clone(),
                    content: reply_content,
                    thread_id: Some(thread_id.clone()),
                    dtx_id: msg_clone.dtx_id.clone(),
                    payload: None,
                };

                {
                    let mut threads = threads_arc.write().await;
                    if let Some(thread) = threads.get_mut(&thread_id) {
                        thread.push(reply.clone());
                    }
                }

                Ok(reply)
            }));
        }

        let _ = coord; // suppress lint

        let mut replies = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(reply)) => replies.push(reply),
                Ok(Err(e)) => warn!("Broadcast reply error: {}", e),
                Err(e) => warn!("Broadcast task panic: {}", e),
            }
        }
        Ok(replies)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // High-level topology runners
    // ─────────────────────────────────────────────────────────────────────────

    /// **Sequential chain**: Pass the task through agents A → B → C → …,
    /// feeding each agent's output as the next agent's input.
    ///
    /// Returns the final agent's output and the full thread ID.
    pub async fn run_chain(
        &self,
        agent_ids: &[&str],
        initial_prompt: &str,
    ) -> Result<(String, String)> {
        self.run_chain_with_dtx(agent_ids, initial_prompt, None).await
    }

    /// **Sequential chain with DTX**: Executes run_chain with an explicit or generated DTX trace token.
    pub async fn run_chain_with_dtx(
        &self,
        agent_ids: &[&str],
        initial_prompt: &str,
        dtx_id: Option<String>,
    ) -> Result<(String, String)> {
        if agent_ids.is_empty() {
            bail!("run_chain requires at least one agent");
        }

        let dtx = dtx_id.unwrap_or_else(|| format!("dtx-chain-{}", uuid::Uuid::new_v4()));
        let thread_id = self.new_thread().await;
        let mut current_content = initial_prompt.to_string();

        for (i, &agent_id) in agent_ids.iter().enumerate() {
            info!(step = i + 1, agent = %agent_id, dtx = %dtx, "Chain step");

            // The "from" is the previous agent or "user" for step 0
            let from = if i == 0 {
                "user".to_string()
            } else {
                agent_ids[i - 1].to_string()
            };

            let msg = AgentMessage {
                id: uuid::Uuid::new_v4().to_string(),
                from_agent: Some(from),
                to_agent: Some(agent_id.to_string()),
                sender_role: AgentRole::Peer,
                content: current_content.clone(),
                thread_id: Some(thread_id.clone()),
                dtx_id: Some(dtx.clone()),
                payload: None,
            };

            let reply = self.send(&thread_id, msg).await?;
            current_content = reply.content;
        }

        Ok((current_content, thread_id))
    }

    /// **Supervisor–Worker–Verifier** pattern:
    ///
    /// 1. Supervisor receives the task and produces a work plan.
    /// 2. Each worker receives the work plan and produces output.
    /// 3. Verifier scores all worker outputs and picks the best (or requests revisions).
    ///
    /// Returns the verifier's final synthesis and the thread ID.
    pub async fn run_supervisor_worker_verifier(
        &self,
        supervisor_id: &str,
        worker_ids: &[&str],
        verifier_id: &str,
        task: &str,
    ) -> Result<(String, String)> {
        self.run_supervisor_worker_verifier_with_dtx(
            supervisor_id,
            worker_ids,
            verifier_id,
            task,
            None,
        )
        .await
    }

    /// **Supervisor–Worker–Verifier with DTX**:
    /// Full SWV pipeline propagating a single distributed audit DTX token across all steps.
    pub async fn run_supervisor_worker_verifier_with_dtx(
        &self,
        supervisor_id: &str,
        worker_ids: &[&str],
        verifier_id: &str,
        task: &str,
        dtx_id: Option<String>,
    ) -> Result<(String, String)> {
        if worker_ids.is_empty() {
            bail!("At least one worker agent is required");
        }

        let dtx = dtx_id.unwrap_or_else(|| format!("dtx-swv-{}", uuid::Uuid::new_v4()));
        let thread_id = self.new_thread().await;
        info!(thread = %thread_id, dtx = %dtx, "Starting Supervisor-Worker-Verifier flow");

        // ── Step 1: Supervisor decomposes the task ───────────────────────────
        let sup_msg = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            from_agent: Some("user".to_string()),
            to_agent: Some(supervisor_id.to_string()),
            sender_role: AgentRole::Peer,
            content: format!(
                "You are the supervisor. Break down the following task into a clear work plan for your team of workers.\n\nTask: {task}"
            ),
            thread_id: Some(thread_id.clone()),
            dtx_id: Some(dtx.clone()),
            payload: None,
        };

        let work_plan = self.send(&thread_id, sup_msg).await?.content;
        info!(thread = %thread_id, "Supervisor produced work plan");

        // ── Step 2: Workers execute in parallel ──────────────────────────────
        let mut worker_handles = Vec::new();
        for &worker_id in worker_ids {
            let agents_arc = Arc::clone(&self.agents);
            let threads_arc = Arc::clone(&self.threads);
            let work_plan_clone = work_plan.clone();
            let worker_id = worker_id.to_string();
            let thread_id_clone = thread_id.clone();
            let dtx_clone = dtx.clone();

            worker_handles.push(tokio::spawn(async move {
                let worker_msg = AgentMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    from_agent: Some("supervisor".to_string()),
                    to_agent: Some(worker_id.clone()),
                    sender_role: AgentRole::Supervisor,
                    content: format!(
                        "Work plan from supervisor:\n{work_plan_clone}\n\nExecute your portion and provide a complete, well-reasoned output."
                    ),
                    thread_id: Some(thread_id_clone.clone()),
                    dtx_id: Some(dtx_clone.clone()),
                    payload: None,
                };

                // Append message to thread
                {
                    let mut threads = threads_arc.write().await;
                    let thread = threads
                        .entry(thread_id_clone.clone())
                        .or_insert_with(|| AgentThread::new(&thread_id_clone));
                    thread.push(worker_msg.clone());
                }

                let agents = agents_arc.read().await;
                let agent = agents
                    .get(&worker_id)
                    .ok_or_else(|| anyhow!("Worker agent '{}' not found", worker_id))?;

                let messages = vec![
                    ChatMessage {
                        role: "system".to_string(),
                        content: agent.system_prompt.clone(),
                    },
                    ChatMessage {
                        role: "user".to_string(),
                        content: worker_msg.content.clone(),
                    },
                ];

                let reply_content = if let Some(runner) = agent.build_loop_runner() {
                    let (ans, _) = runner.run(&agent.system_prompt, &worker_msg.content).await?;
                    ans
                } else {
                    agent.complete(messages).await?
                };

                let reply = AgentMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    from_agent: Some(worker_id.clone()),
                    to_agent: Some("verifier".to_string()),
                    sender_role: AgentRole::Worker,
                    content: reply_content,
                    thread_id: Some(thread_id_clone.clone()),
                    dtx_id: Some(dtx_clone),
                    payload: None,
                };

                {
                    let mut threads = threads_arc.write().await;
                    if let Some(thread) = threads.get_mut(&thread_id_clone) {
                        thread.push(reply.clone());
                    }
                }

                Ok::<AgentMessage, anyhow::Error>(reply)
            }));
        }

        let mut worker_outputs = Vec::new();
        for handle in worker_handles {
            match handle.await {
                Ok(Ok(msg)) => worker_outputs.push(msg),
                Ok(Err(e)) => warn!("Worker error: {}", e),
                Err(e) => warn!("Worker panic: {}", e),
            }
        }
        info!(thread = %thread_id, outputs = worker_outputs.len(), "Workers completed");

        // ── Step 3: Verifier synthesises and scores ─────────────────────────
        let aggregated = worker_outputs
            .iter()
            .enumerate()
            .map(|(i, m)| {
                format!(
                    "=== Worker {} ({}) ===\n{}",
                    i + 1,
                    m.from_agent.as_deref().unwrap_or("unknown"),
                    m.content
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        let verifier_msg = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            from_agent: Some("coordinator".to_string()),
            to_agent: Some(verifier_id.to_string()),
            sender_role: AgentRole::Coordinator,
            content: format!(
                "Original task: {task}\n\nWork plan:\n{work_plan}\n\nWorker outputs:\n{aggregated}\n\n\
                 Review all outputs. Identify the strongest solution, correct any errors, \
                 and synthesise a single high-quality final answer."
            ),
            thread_id: Some(thread_id.clone()),
            dtx_id: Some(dtx),
            payload: None,
        };

        let final_answer = self.send(&thread_id, verifier_msg).await?.content;
        info!(thread = %thread_id, "Verifier produced final answer");

        // Mark thread as resolved
        {
            let mut threads = self.threads.write().await;
            if let Some(thread) = threads.get_mut(&thread_id) {
                thread.final_answer = Some(final_answer.clone());
            }
        }

        Ok((final_answer, thread_id))
    }

    /// **Peer dialogue**: Two agents exchange messages for up to `max_rounds`
    /// rounds. Useful for debate, critique, or co-refinement flows.
    ///
    /// Returns the last reply and the thread ID.
    pub async fn run_peer_dialogue(
        &self,
        agent_a: &str,
        agent_b: &str,
        opening: &str,
        max_rounds: usize,
    ) -> Result<(String, String)> {
        let res = self
            .run_peer_dialogue_with_eval(agent_a, agent_b, opening, max_rounds, None)
            .await?;
        Ok((res.final_content, res.thread_id))
    }

    /// **Peer dialogue with structured consensus evaluation**:
    /// Two agents exchange messages while evaluating mutual alignment score,
    /// tracking completed rounds, and detecting early consensus keywords.
    pub async fn run_peer_dialogue_with_eval(
        &self,
        agent_a: &str,
        agent_b: &str,
        opening: &str,
        max_rounds: usize,
        dtx_id: Option<String>,
    ) -> Result<PeerDialogueResult> {
        let thread_id = self.new_thread().await;
        let dtx = dtx_id.unwrap_or_else(|| format!("dtx-peer-{}", uuid::Uuid::new_v4()));
        info!(thread = %thread_id, rounds = max_rounds, dtx = %dtx, "Starting peer dialogue");

        let mut current_content = opening.to_string();
        let mut current_from = "user".to_string();
        let mut current_to = agent_a.to_string();
        let mut consensus_reached = false;
        let mut consensus_score: f32 = 0.0;
        let mut rounds_completed = 0;

        for round in 0..max_rounds {
            rounds_completed = round + 1;
            debug!(thread = %thread_id, round = rounds_completed, from = %current_from, to = %current_to);

            let msg = AgentMessage {
                id: uuid::Uuid::new_v4().to_string(),
                from_agent: Some(current_from.clone()),
                to_agent: Some(current_to.clone()),
                sender_role: AgentRole::Peer,
                content: current_content.clone(),
                thread_id: Some(thread_id.clone()),
                dtx_id: Some(dtx.clone()),
                payload: None,
            };

            let reply = self.send(&thread_id, msg).await?;
            current_content = reply.content.clone();

            // Swap speakers
            current_from = current_to.clone();
            current_to = if current_to == agent_a {
                agent_b.to_string()
            } else {
                agent_a.to_string()
            };

            // Evaluate consensus & alignment indicators
            let lower = current_content.to_lowercase();
            let mut score = 0.0f32;
            if lower.contains("[agree]")
                || lower.contains("[final answer]")
                || lower.contains("[consensus]")
            {
                score += 0.5;
                consensus_reached = true;
            }
            if lower.contains("agree") || lower.contains("concur") {
                score += 0.2;
            }
            if lower.contains("verified") || lower.contains("approved") {
                score += 0.2;
            }
            if lower.contains("solution") || lower.contains("correct") {
                score += 0.1;
            }
            consensus_score = score.clamp(0.0, 1.0);

            if consensus_reached {
                info!(thread = %thread_id, round = rounds_completed, "Consensus reached early");
                break;
            }
        }

        Ok(PeerDialogueResult {
            final_content: current_content,
            thread_id,
            rounds_completed,
            consensus_reached,
            consensus_score,
            dtx_id: Some(dtx),
        })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Introspection & Audit
    // ─────────────────────────────────────────────────────────────────────────

    /// Retrieve the full message history for a thread.
    pub async fn thread_history(&self, thread_id: &str) -> Option<AgentThread> {
        self.threads.read().await.get(thread_id).cloned()
    }

    /// Retrieve all messages associated with a specific DTX transaction ID across a thread.
    pub async fn thread_dtx_messages(&self, thread_id: &str, dtx_id: &str) -> Vec<AgentMessage> {
        let threads = self.threads.read().await;
        threads
            .get(thread_id)
            .map(|t| {
                t.messages
                    .iter()
                    .filter(|m| m.dtx_id.as_deref() == Some(dtx_id))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// List all registered agents (id → role).
    pub async fn list_agents(&self) -> Vec<(String, AgentRole)> {
        self.agents
            .read()
            .await
            .iter()
            .map(|(id, rec)| (id.clone(), rec.role.clone()))
            .collect()
    }
}

// ── AgentBuilder ─────────────────────────────────────────────────────────────

/// Ergonomic builder for constructing `AgentRecord`s.
pub struct AgentBuilder {
    id: String,
    role: AgentRole,
    system_prompt: String,
    provider: Option<Arc<dyn InferenceProvider>>,
    tool_executor: Option<Arc<dyn ToolExecutor>>,
    tool_definitions: Vec<ToolDefinition>,
    max_turns: usize,
}

impl AgentBuilder {
    pub fn new(id: impl Into<String>, role: AgentRole) -> Self {
        Self {
            id: id.into(),
            role,
            system_prompt: String::new(),
            provider: None,
            tool_executor: None,
            tool_definitions: Vec::new(),
            max_turns: 8,
        }
    }

    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    pub fn provider(mut self, provider: Arc<dyn InferenceProvider>) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn tool_executor(mut self, executor: Arc<dyn ToolExecutor>) -> Self {
        self.tool_executor = Some(executor);
        self
    }

    pub fn tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.tool_definitions = tools;
        self
    }

    pub fn max_turns(mut self, turns: usize) -> Self {
        self.max_turns = turns;
        self
    }

    pub fn build(self) -> Result<AgentRecord> {
        let provider = self
            .provider
            .ok_or_else(|| anyhow!("AgentBuilder: provider is required"))?;
        Ok(AgentRecord {
            id: self.id,
            role: self.role,
            system_prompt: self.system_prompt,
            provider,
            tool_executor: self.tool_executor,
            tool_definitions: self.tool_definitions,
            max_turns: self.max_turns,
        })
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{
        BackendHealth, ChatRequest, ChatResponse, InferenceCapabilities, InferenceProvider,
        ProviderKind, StreamResult,
    };

    /// Deterministic mock provider that echoes its system prompt + the user message.
    struct EchoProvider {
        name: String,
    }

    #[async_trait::async_trait]
    impl InferenceProvider for EchoProvider {
        fn capabilities(&self) -> InferenceCapabilities {
            InferenceCapabilities {
                provider: ProviderKind::OpenAiCompatible,
                supports_streaming: false,
                supports_tool_calls: false,
                supports_lora_hotswap: false,
                supports_json_mode: false,
                supports_grammar_constrained: false,
                supports_speculative_decoding: false,
                supports_prefix_cache: false,
                supports_multimodal: false,
                context_window: 4096,
            }
        }

        async fn chat_completion(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
            let last = req.messages.last().map(|m| m.content.as_str()).unwrap_or("");
            let content = format!("[{}] {}", self.name, last);
            Ok(ChatResponse {
                content,
                prompt_tokens: 10,
                completion_tokens: 5,
                finish_reason: Some("stop".to_string()),
                tool_calls: vec![],
                latency_ms: 1,
                slot_id: None,
            })
        }

        async fn stream_chat(&self, _req: ChatRequest) -> anyhow::Result<StreamResult> {
            unimplemented!()
        }

        async fn health(&self) -> anyhow::Result<BackendHealth> {
            Ok(BackendHealth {
                healthy: true,
                provider_name: self.name.clone(),
                active_model: "echo".to_string(),
                memory_used_mb: None,
                vram_used_mb: None,
                available_slots: None,
                queue_depth: None,
            })
        }

        fn provider_name(&self) -> &str {
            &self.name
        }
    }

    fn make_echo_agent(id: &str, role: AgentRole) -> AgentRecord {
        AgentRecord {
            id: id.to_string(),
            role,
            system_prompt: format!("You are {id}."),
            provider: Arc::new(EchoProvider { name: id.to_string() }),
            tool_executor: None,
            tool_definitions: vec![],
            max_turns: 4,
        }
    }

    #[tokio::test]
    async fn test_register_and_list() {
        let coord = MultiAgentCoordinator::new();
        coord.register(make_echo_agent("alpha", AgentRole::Worker)).await;
        coord.register(make_echo_agent("beta", AgentRole::Verifier)).await;

        let agents = coord.list_agents().await;
        assert_eq!(agents.len(), 2);
    }

    #[tokio::test]
    async fn test_send_and_reply() {
        let coord = MultiAgentCoordinator::new();
        coord.register(make_echo_agent("worker1", AgentRole::Worker)).await;

        let thread_id = coord.new_thread().await;
        let msg = AgentMessage::new("user", "worker1", AgentRole::Peer, "hello worker");
        let reply = coord.send(&thread_id, msg).await.unwrap();

        assert!(reply.content.contains("[worker1]"));
        assert!(reply.content.contains("hello worker"));
    }

    #[tokio::test]
    async fn test_chain_two_agents() {
        let coord = MultiAgentCoordinator::new();
        coord.register(make_echo_agent("planner", AgentRole::Supervisor)).await;
        coord.register(make_echo_agent("coder", AgentRole::Worker)).await;

        let (result, _thread) = coord
            .run_chain(&["planner", "coder"], "Build a sorting algorithm")
            .await
            .unwrap();

        // coder echoes what planner produced
        assert!(result.contains("[coder]"));
    }

    #[tokio::test]
    async fn test_peer_dialogue_terminates() {
        let coord = MultiAgentCoordinator::new();
        coord.register(make_echo_agent("agent_a", AgentRole::Peer)).await;
        coord.register(make_echo_agent("agent_b", AgentRole::Peer)).await;

        let (last, thread_id) = coord
            .run_peer_dialogue("agent_a", "agent_b", "Let's discuss Rust async", 4)
            .await
            .unwrap();

        let history = coord.thread_history(&thread_id).await.unwrap();
        assert!(!last.is_empty());
        assert!(history.messages.len() <= 8 + 1); // 4 rounds × 2 speakers + initial
    }

    #[tokio::test]
    async fn test_broadcast_reaches_all() {
        let coord = MultiAgentCoordinator::new();
        coord.register(make_echo_agent("sup", AgentRole::Supervisor)).await;
        coord.register(make_echo_agent("w1", AgentRole::Worker)).await;
        coord.register(make_echo_agent("w2", AgentRole::Worker)).await;

        let thread_id = coord.new_thread().await;
        let replies = coord
            .broadcast(&thread_id, "sup", AgentRole::Supervisor, "status check")
            .await
            .unwrap();

        // sup broadcasts to w1 and w2
        assert_eq!(replies.len(), 2);
    }

    #[tokio::test]
    async fn test_agent_builder() {
        let provider: Arc<dyn InferenceProvider> = Arc::new(EchoProvider { name: "test".into() });
        let record = AgentBuilder::new("researcher", AgentRole::Worker)
            .system_prompt("You research topics.")
            .provider(Arc::clone(&provider))
            .max_turns(6)
            .build()
            .unwrap();

        assert_eq!(record.id, "researcher");
        assert_eq!(record.max_turns, 6);
    }

    #[tokio::test]
    async fn test_peer_dialogue_with_eval() {
        let coord = MultiAgentCoordinator::new();
        coord.register(make_echo_agent("debater_a", AgentRole::Peer)).await;
        coord.register(make_echo_agent("debater_b", AgentRole::Peer)).await;

        let eval = coord
            .run_peer_dialogue_with_eval(
                "debater_a",
                "debater_b",
                "Proposal: [agree] We should use embedded-hal for the driver.",
                3,
                Some("dtx-test-123".to_string()),
            )
            .await
            .unwrap();

        assert!(eval.consensus_reached);
        assert!(eval.consensus_score >= 0.5);
        assert_eq!(eval.dtx_id.as_deref(), Some("dtx-test-123"));

        let dtx_msgs = coord.thread_dtx_messages(&eval.thread_id, "dtx-test-123").await;
        assert!(!dtx_msgs.is_empty());
    }

    #[tokio::test]
    async fn test_swv_with_dtx() {
        let coord = MultiAgentCoordinator::new();
        coord.register(make_echo_agent("supervisor", AgentRole::Supervisor)).await;
        coord.register(make_echo_agent("coder", AgentRole::Worker)).await;
        coord.register(make_echo_agent("verifier", AgentRole::Verifier)).await;

        let (final_ans, thread_id) = coord
            .run_supervisor_worker_verifier_with_dtx(
                "supervisor",
                &["coder"],
                "verifier",
                "Write an embassy-stm32 SPI driver",
                Some("dtx-swv-456".to_string()),
            )
            .await
            .unwrap();

        assert!(final_ans.contains("[verifier]"));
        let dtx_msgs = coord.thread_dtx_messages(&thread_id, "dtx-swv-456").await;
        assert!(dtx_msgs.len() >= 3);
    }
}
