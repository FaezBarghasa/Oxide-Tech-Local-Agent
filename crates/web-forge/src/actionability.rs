use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::WebForgeError;

/// Bounding rectangle of a DOM element in viewport coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl BoundingBox {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Compute the center coordinate (x, y) for hit-testing and clicking.
    pub fn center(&self) -> (f64, f64) {
        (self.x + (self.width / 2.0), self.y + (self.height / 2.0))
    }

    /// Returns true if width or height is zero or negative.
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    /// Check if point is inside bounding box.
    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.x && px <= (self.x + self.width) && py >= self.y && py <= (self.y + self.height)
    }

    /// Check if two bounding boxes are approximately equal within an epsilon.
    pub fn is_approx_equal(&self, other: &BoundingBox, epsilon: f64) -> bool {
        (self.x - other.x).abs() <= epsilon
            && (self.y - other.y).abs() <= epsilon
            && (self.width - other.width).abs() <= epsilon
            && (self.height - other.height).abs() <= epsilon
    }
}

/// Snapshot of an element's physical and computed layout state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementLayoutState {
    pub node_id: u64,
    pub selector: String,
    pub attached: bool,
    pub bounding_box: Option<BoundingBox>,
    pub visible: bool,
    pub enabled: bool,
    pub opacity: f64,
    pub display: String,
    pub visibility: String,
}

impl ElementLayoutState {
    /// Helper to create a fully actionable button/element.
    pub fn new_actionable(node_id: u64, selector: impl Into<String>, bbox: BoundingBox) -> Self {
        Self {
            node_id,
            selector: selector.into(),
            attached: true,
            bounding_box: Some(bbox),
            visible: true,
            enabled: true,
            opacity: 1.0,
            display: "block".to_string(),
            visibility: "visible".to_string(),
        }
    }
}

/// Result of a viewport coordinate hit-test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HitTestResult {
    pub hit_node_id: u64,
    pub hit_selector: String,
}

/// Trait abstracting Chrome DevTools Protocol (CDP) or in-memory layout engine queries.
pub trait LayoutInspector {
    fn query_element_state(
        &self,
        selector: &str,
    ) -> Result<Option<ElementLayoutState>, WebForgeError>;
    fn perform_hit_test(&self, x: f64, y: f64) -> Result<Option<HitTestResult>, WebForgeError>;
}

/// Configuration parameters for Playwright's 5-condition Auto-Waiting algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionabilityConfig {
    pub timeout: Duration,
    pub poll_interval: Duration,
    pub stability_samples: usize,
    pub stability_epsilon: f64,
    pub check_visible: bool,
    pub check_stable: bool,
    pub check_enabled: bool,
    pub check_uncovered: bool,
}

impl Default for ActionabilityConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            poll_interval: Duration::from_millis(50),
            stability_samples: 3,
            stability_epsilon: 0.5,
            check_visible: true,
            check_stable: true,
            check_enabled: true,
            check_uncovered: true,
        }
    }
}

/// Deterministic Playwright-style Actionability Auto-Wait Engine.
#[derive(Debug, Default)]
pub struct ActionabilityEvaluator;

impl ActionabilityEvaluator {
    pub fn new() -> Self {
        Self
    }

    /// The 5-Check Playwright Auto-Wait Algorithm:
    /// 1. Attached: Element exists in DOM
    /// 2. Visible: Non-empty box, display != none, visibility != hidden, opacity > 0
    /// 3. Stable: Dimensions and position remain unchanged across consecutive samples
    /// 4. Enabled: Not disabled attribute or aria-disabled
    /// 5. Uncovered: Hit test at center point returns the element itself (no overlay/spinner)
    pub async fn wait_for_actionable<I: LayoutInspector>(
        &self,
        inspector: &I,
        selector: &str,
        config: &ActionabilityConfig,
    ) -> Result<ElementLayoutState, WebForgeError> {
        let start = Instant::now();
        let mut stable_samples: Vec<BoundingBox> = Vec::with_capacity(config.stability_samples);

        loop {
            if start.elapsed() > config.timeout {
                return Err(WebForgeError::ActionabilityTimeout(format!(
                    "Timed out after {:?} waiting for element '{}' to become actionable",
                    config.timeout, selector
                )));
            }

            // 1. Attached check
            let state = inspector
                .query_element_state(selector)?
                .ok_or_else(|| WebForgeError::ElementNotFound(selector.to_string()))?;

            if !state.attached {
                sleep(config.poll_interval).await;
                continue;
            }

            // 2. Visible check
            if config.check_visible {
                let is_visible = state.visible
                    && state.display != "none"
                    && state.visibility != "hidden"
                    && state.opacity > 0.0
                    && state.bounding_box.map(|b| !b.is_empty()).unwrap_or(false);

                if !is_visible {
                    sleep(config.poll_interval).await;
                    continue;
                }
            }

            let bbox = match state.bounding_box {
                Some(b) => b,
                None => {
                    sleep(config.poll_interval).await;
                    continue;
                }
            };

            // 3. Stable check (sampling over consecutive frames)
            if config.check_stable {
                if let Some(last_box) = stable_samples.last() {
                    if last_box.is_approx_equal(&bbox, config.stability_epsilon) {
                        stable_samples.push(bbox);
                    } else {
                        stable_samples.clear();
                        stable_samples.push(bbox);
                    }
                } else {
                    stable_samples.push(bbox);
                }

                if stable_samples.len() < config.stability_samples {
                    sleep(config.poll_interval).await;
                    continue;
                }
            }

            // 4. Enabled check
            if config.check_enabled && !state.enabled {
                sleep(config.poll_interval).await;
                continue;
            }

            // 5. Uncovered (Hit-test at center) check
            if config.check_uncovered {
                let (cx, cy) = bbox.center();
                let hit = inspector.perform_hit_test(cx, cy)?;

                match hit {
                    Some(hit_res) if hit_res.hit_node_id == state.node_id => {
                        // All 5 checks passed!
                        return Ok(state);
                    }
                    Some(hit_res) if start.elapsed() + config.poll_interval > config.timeout => {
                        return Err(WebForgeError::ElementOccluded {
                            selector: selector.to_string(),
                            hit_element: hit_res.hit_selector,
                        });
                    }
                    Some(_) | None => {
                        // Element currently covered or not hit; continue polling
                    }
                }
            } else {
                // If hit-test bypassed, return immediately
                return Ok(state);
            }

            sleep(config.poll_interval).await;
        }
    }
}
