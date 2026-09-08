use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An Accessibility Tree node representing a semantic web element.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AXNode {
    pub node_id: u64,
    pub role: String,
    pub name: Option<String>,
    pub value: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub checked: Option<bool>,
    #[serde(default)]
    pub expanded: Option<bool>,
    #[serde(default)]
    pub ignored: bool,
    #[serde(default)]
    pub children: Vec<u64>,
}

impl AXNode {
    pub fn new(node_id: u64, role: impl Into<String>) -> Self {
        Self {
            node_id,
            role: role.into(),
            name: None,
            value: None,
            description: None,
            disabled: false,
            checked: None,
            expanded: None,
            ignored: false,
            children: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// A complete Accessibility Tree for a web page.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AXTree {
    pub nodes: Vec<AXNode>,
    pub root_id: Option<u64>,
}

impl AXTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: AXNode) {
        if self.nodes.is_empty() {
            self.root_id = Some(node.node_id);
        }
        self.nodes.push(node);
    }
}

/// Distills complex DOM and AXTree structures into ultra-compact, token-efficient Markdown for LLMs/VLMs.
#[derive(Debug, Default)]
pub struct DomDistiller;

impl DomDistiller {
    /// Converts an `AXTree` into structured, token-efficient Markdown.
    pub fn distill_axtree(tree: &AXTree) -> String {
        let mut out = String::from("## Interactive Page State\n");
        let node_map: HashMap<u64, &AXNode> = tree.nodes.iter().map(|n| (n.node_id, n)).collect();

        for node in &tree.nodes {
            // Filter out ignored or purely decorative layout elements
            if node.ignored || Self::is_pure_layout_role(&node.role) {
                continue;
            }

            let role = node.role.as_str();
            let name = node.name.as_deref().unwrap_or_default();
            let mut state_tags = Vec::new();

            if node.disabled {
                state_tags.push("disabled");
            }
            if let Some(true) = node.checked {
                state_tags.push("checked");
            }
            if let Some(true) = node.expanded {
                state_tags.push("expanded");
            }

            let state_str = if state_tags.is_empty() {
                String::new()
            } else {
                format!(" ({})", state_tags.join(", "))
            };

            let val_str = if let Some(ref val) = node.value {
                format!(" value='{}'", val)
            } else {
                String::new()
            };

            let desc_str = if let Some(ref desc) = node.description {
                format!(" desc='{}'", desc)
            } else {
                String::new()
            };

            if name.is_empty() && val_str.is_empty() {
                // If it has children with names, skip printing empty container
                let has_named_children = node.children.iter().any(|cid| {
                    node_map
                        .get(cid)
                        .map(|cn| cn.name.is_some())
                        .unwrap_or(false)
                });
                if has_named_children {
                    continue;
                }
            }

            out.push_str(&format!(
                "- [{}] {} '{}'{}{}{}\n",
                node.node_id, role, name, val_str, desc_str, state_str
            ));
        }

        out
    }

    /// Fast semantic HTML parser for converting raw HTML snippets into an `AXTree`.
    pub fn parse_semantic_html(html: &str) -> AXTree {
        let mut tree = AXTree::new();
        let mut current_id = 1u64;

        // Clean up and tokenize HTML tags
        let tag_token_regex =
            regex::Regex::new(r#"(?is)<(/)?([a-zA-Z0-9]+)([^>]*)>"#).expect("Valid tag regex");

        let mut open_tag: Option<(String, String, usize)> = None; // (tag_name, attrs, content_start_pos)

        for cap in tag_token_regex.captures_iter(html) {
            let m = cap.get(0).unwrap();
            let is_close = cap.get(1).is_some();
            let tag_name = cap[2].to_lowercase();
            let attrs = &cap[3];

            if is_close {
                if let Some((prev_tag, prev_attrs, start_idx)) = open_tag.take() {
                    let is_matching_tag = prev_tag == tag_name;
                    if is_matching_tag {
                        let text = html[start_idx..m.start()].trim().to_string();
                        Self::create_node_from_tag(
                            &mut tree,
                            &mut current_id,
                            &prev_tag,
                            &prev_attrs,
                            &text,
                        );
                    }
                }
            } else {
                // If previous open tag wasn't closed by explicit closing tag (or self closing)
                if let Some((prev_tag, prev_attrs, start_idx)) = open_tag.take() {
                    let text = html[start_idx..m.start()].trim().to_string();
                    Self::create_node_from_tag(
                        &mut tree,
                        &mut current_id,
                        &prev_tag,
                        &prev_attrs,
                        &text,
                    );
                }

                // Check if self-closing (e.g. <input ... />) or void element
                let is_self_closing =
                    attrs.trim_end().ends_with('/') || tag_name == "input" || tag_name == "img";

                if is_self_closing {
                    Self::create_node_from_tag(&mut tree, &mut current_id, &tag_name, attrs, "");
                } else {
                    open_tag = Some((tag_name, attrs.to_string(), m.end()));
                }
            }
        }

        if let Some((prev_tag, prev_attrs, start_idx)) = open_tag {
            let text = if start_idx < html.len() {
                html[start_idx..].trim().to_string()
            } else {
                String::new()
            };
            Self::create_node_from_tag(&mut tree, &mut current_id, &prev_tag, &prev_attrs, &text);
        }

        tree
    }

    fn create_node_from_tag(
        tree: &mut AXTree,
        current_id: &mut u64,
        tag: &str,
        attrs: &str,
        inner_text: &str,
    ) {
        let is_hidden = attrs.contains("hidden")
            || attrs.contains("display:none")
            || attrs.contains("display: none")
            || attrs.contains("visibility:hidden")
            || attrs.contains("visibility: hidden");
        let is_disabled = attrs.contains("disabled") || attrs.contains("aria-disabled=\"true\"");

        let (role, name, value) = match tag {
            "button" => ("button", inner_text.to_string(), None),
            "a" => ("link", inner_text.to_string(), None),
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => ("heading", inner_text.to_string(), None),
            "input" => {
                let input_type =
                    Self::extract_attr(attrs, "type").unwrap_or_else(|| "text".to_string());
                let placeholder = Self::extract_attr(attrs, "placeholder").unwrap_or_default();
                let val = Self::extract_attr(attrs, "value");
                let name = if !placeholder.is_empty() {
                    placeholder
                } else {
                    inner_text.to_string()
                };
                (
                    if input_type == "checkbox" {
                        "checkbox"
                    } else {
                        "textbox"
                    },
                    name,
                    val,
                )
            }
            "textarea" => {
                let placeholder = Self::extract_attr(attrs, "placeholder").unwrap_or_default();
                let val = if !inner_text.is_empty() {
                    Some(inner_text.to_string())
                } else {
                    None
                };
                ("textbox", placeholder, val)
            }
            "select" => ("combobox", inner_text.to_string(), None),
            "dialog" => ("dialog", inner_text.to_string(), None),
            _ => ("generic", inner_text.to_string(), None),
        };

        let mut node = AXNode::new(*current_id, role);
        if !name.is_empty() {
            node.name = Some(name);
        }
        if let Some(v) = value {
            node.value = Some(v);
        }
        node.disabled = is_disabled;
        node.ignored = is_hidden || (role == "generic" && node.name.is_none());

        tree.add_node(node);
        *current_id += 1;
    }

    fn extract_attr(attrs: &str, attr_name: &str) -> Option<String> {
        let pattern = format!(r#"{}=["']([^"']*)["']"#, attr_name);
        let re = regex::Regex::new(&pattern).ok()?;
        re.captures(attrs).map(|c| c[1].to_string())
    }

    fn is_pure_layout_role(role: &str) -> bool {
        matches!(
            role,
            "generic" | "none" | "presentation" | "group" | "section" | "paragraph" | "LineBreak"
        )
    }
}
