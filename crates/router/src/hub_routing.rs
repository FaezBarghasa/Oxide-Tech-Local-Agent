use ndarray::{Array2, Axis};

pub struct SparseHubRouter {
    pub num_agents: usize,
    pub num_hubs: usize,
}

impl SparseHubRouter {
    pub fn new(num_agents: usize, num_hubs: usize) -> Self {
        Self {
            num_agents,
            num_hubs,
        }
    }

    /// Route messages from N agents through H hubs to other agents
    pub fn route_agent_states(
        &self,
        agent_states: &Array2<f32>,
        hub_weights: &Array2<f32>,
    ) -> Array2<f32> {
        // Step 1: Project agent states to hub representation (N x D) * (D x H) = (N x H)
        let agent_to_hub = agent_states.dot(hub_weights);

        // Step 2: Pool the hub representations
        let hub_representation = agent_to_hub.mean_axis(Axis(0)).unwrap();

        // Step 3: Broadcast pooled hub representation back to agents
        let mut routed_states = Array2::<f32>::zeros(agent_states.dim());
        for i in 0..self.num_agents {
            for j in 0..agent_states.dim().1 {
                routed_states[[i, j]] =
                    agent_states[[i, j]] + hub_representation[j % self.num_hubs];
            }
        }
        routed_states
    }
}
