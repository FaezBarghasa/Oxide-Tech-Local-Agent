use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CloudBudgetManager {
    pub daily_limit_usd: f32,
    pub monthly_limit_usd: f32,
    pub current_daily_spend: Arc<RwLock<f32>>,
    pub current_monthly_spend: Arc<RwLock<f32>>,
}

impl CloudBudgetManager {
    pub fn new(daily_limit_usd: f32, monthly_limit_usd: f32) -> Self {
        Self {
            daily_limit_usd,
            monthly_limit_usd,
            current_daily_spend: Arc::new(RwLock::new(0.0)),
            current_monthly_spend: Arc::new(RwLock::new(0.0)),
        }
    }

    pub async fn check_budget_ok(&self) -> bool {
        let daily = *self.current_daily_spend.read().await;
        let monthly = *self.current_monthly_spend.read().await;
        
        daily < self.daily_limit_usd && monthly < self.monthly_limit_usd
    }

    pub async fn record_spend(&self, cost: f32) {
        let mut daily = self.current_daily_spend.write().await;
        let mut monthly = self.current_monthly_spend.write().await;
        
        *daily += cost;
        *monthly += cost;
    }

    pub fn get_provider_tier(&self, is_critical: bool) -> &'static str {
        if is_critical {
            "expensive" // OpenAI GPT-4o-level
        } else {
            "cheap" // Groq / DeepSeek free tiers
        }
    }
}
