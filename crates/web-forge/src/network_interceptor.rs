use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rule for matching intercepted network requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MockRule {
    pub id: String,
    pub url_pattern: String,
    pub method: Option<String>,
}

impl MockRule {
    pub fn new(id: impl Into<String>, url_pattern: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            url_pattern: url_pattern.into(),
            method: None,
        }
    }

    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into().to_uppercase());
        self
    }

    /// Check if an incoming request matches this rule.
    pub fn matches(&self, url: &str, method: &str) -> bool {
        let method_matches = self
            .method
            .as_deref()
            .map(|m| m.eq_ignore_ascii_case(method))
            .unwrap_or(true);

        if !method_matches {
            return false;
        }

        if let Ok(re) = regex::Regex::new(&self.url_pattern) {
            re.is_match(url)
        } else {
            url.contains(&self.url_pattern)
        }
    }
}

/// Simulated response for causal mocking and fault injection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MockResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub delay_ms: u64,
}

impl MockResponse {
    pub fn new(status_code: u16, body: impl Into<Vec<u8>>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        Self {
            status_code,
            headers,
            body: body.into(),
            delay_ms: 0,
        }
    }

    pub fn json<T: Serialize>(status_code: u16, value: &T) -> Result<Self, serde_json::Error> {
        let body = serde_json::to_vec(value)?;
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        Ok(Self {
            status_code,
            headers,
            body,
            delay_ms: 0,
        })
    }

    pub fn error_500(message: &str) -> Self {
        let json_body = format!(r#"{{"error":"{}"}}"#, message);
        Self::new(500, json_body.as_bytes())
    }

    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }
}

/// Metadata of an intercepted HTTP request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterceptedRequest {
    pub request_id: String,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub timestamp: DateTime<Utc>,
}

impl InterceptedRequest {
    pub fn new(url: impl Into<String>, method: impl Into<String>) -> Self {
        Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            url: url.into(),
            method: method.into().to_uppercase(),
            headers: HashMap::new(),
            body: None,
            timestamp: Utc::now(),
        }
    }
}

/// Decision made on an intercepted request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterceptDecision {
    Mock(MockResponse),
    Continue,
    Drop,
}

/// Causal Network Interceptor and Chaos Fault Injection Engine for Crucible.
#[derive(Debug, Default)]
pub struct NetworkInterceptor {
    rules: Vec<(MockRule, MockResponse)>,
    history: Vec<InterceptedRequest>,
}

impl NetworkInterceptor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a mock rule and its corresponding mock response.
    pub fn add_rule(&mut self, rule: MockRule, response: MockResponse) {
        self.rules.push((rule, response));
    }

    /// Convenience helper to mock a route with status and JSON payload.
    pub fn mock_route(&mut self, url_pattern: &str, status: u16, body_json: &str) {
        let rule = MockRule::new(uuid::Uuid::new_v4().to_string(), url_pattern);
        let response = MockResponse::new(status, body_json.as_bytes());
        self.add_rule(rule, response);
    }

    /// Evaluate an intercepted request against configured rules and record history.
    pub fn intercept(&mut self, request: InterceptedRequest) -> InterceptDecision {
        let decision = self
            .rules
            .iter()
            .find(|(rule, _)| rule.matches(&request.url, &request.method))
            .map(|(_, res)| InterceptDecision::Mock(res.clone()))
            .unwrap_or(InterceptDecision::Continue);

        self.history.push(request);
        decision
    }

    /// Get historical log of all intercepted requests.
    pub fn history(&self) -> &[InterceptedRequest] {
        &self.history
    }

    /// Clear all recorded history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
