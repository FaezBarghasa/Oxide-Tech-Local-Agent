use crate::ast::{ParsedSymbol, SymbolField, SymbolMethod};
use tree_sitter::{Node, Parser};
use tree_sitter_rust::language;

pub fn parse_file(content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String> {
    let mut parser = Parser::new();
    parser.set_language(language()).map_err(|e| e.to_string())?;
    let tree = parser
        .parse(content, None)
        .ok_or("Failed to parse content")?;
    let root_node = tree.root_node();
    let mut symbols = Vec::new();
    traverse_nodes(root_node, content, file_path, &mut symbols);
    Ok(symbols)
}

fn get_previous_doc_comments(node: Node, content: &str) -> Option<String> {
    let mut current = node;
    let mut comments = Vec::new();
    while let Some(prev) = current.prev_sibling() {
        let text = content
            .get(prev.start_byte()..prev.end_byte())
            .unwrap_or("")
            .trim();
        if prev.kind() == "line_comment" && text.starts_with("///") {
            comments.push(text.to_string());
            current = prev;
            continue;
        } else if prev.kind() == "block_comment" && text.starts_with("/**") {
            comments.push(text.to_string());
            current = prev;
            continue;
        }
        break;
    }

    if comments.is_empty() {
        None
    } else {
        comments.reverse();
        Some(comments.join("\n"))
    }
}

fn traverse_nodes(node: Node, content: &str, file_path: &str, symbols: &mut Vec<ParsedSymbol>) {
    let kind = node.kind();

    let get_text = |n: Node| -> String {
        content
            .get(n.start_byte()..n.end_byte())
            .unwrap_or("")
            .to_string()
    };

    let start_line = node.start_position().row + 1;
    let end_line = node.end_position().row + 1;
    let node_content = get_text(node);
    let doc_comment = get_previous_doc_comments(node, content);

    match kind {
        "struct_item" => {
            let mut name = String::new();
            let mut fields = Vec::new();

            if let Some(name_node) = node.child_by_field_name("name") {
                name = get_text(name_node);
            } else {
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind() == "type_identifier" {
                        name = get_text(child);
                        break;
                    }
                }
            }

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "field_declaration_list" {
                    let mut field_cursor = child.walk();
                    for field in child.children(&mut field_cursor) {
                        if field.kind() == "field_declaration" {
                            let mut field_name = String::new();
                            let mut field_type = String::new();
                            if let Some(fn_node) = field.child_by_field_name("name") {
                                field_name = get_text(fn_node);
                            }
                            if let Some(ft_node) = field.child_by_field_name("type") {
                                field_type = get_text(ft_node);
                            }
                            if !field_name.is_empty() {
                                fields.push(SymbolField {
                                    name: field_name,
                                    r#type: field_type,
                                });
                            }
                        }
                    }
                }
            }

            symbols.push(ParsedSymbol {
                name,
                kind: "struct".to_string(),
                file_path: file_path.to_string(),
                start_line,
                end_line,
                content: node_content,
                doc_comment,
                fields: Some(fields),
                variants: None,
                methods: None,
                implements_trait: None,
                target_type: None,
            });
        }
        "enum_item" => {
            let mut name = String::new();
            let mut variants = Vec::new();

            if let Some(name_node) = node.child_by_field_name("name") {
                name = get_text(name_node);
            } else {
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind() == "type_identifier" {
                        name = get_text(child);
                        break;
                    }
                }
            }

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "enum_variant_list" {
                    let mut var_cursor = child.walk();
                    for variant in child.children(&mut var_cursor) {
                        if variant.kind() == "enum_variant" {
                            if let Some(v_name) = variant.child_by_field_name("name") {
                                variants.push(get_text(v_name));
                            }
                        }
                    }
                }
            }

            symbols.push(ParsedSymbol {
                name,
                kind: "enum".to_string(),
                file_path: file_path.to_string(),
                start_line,
                end_line,
                content: node_content,
                doc_comment,
                fields: None,
                variants: Some(variants),
                methods: None,
                implements_trait: None,
                target_type: None,
            });
        }
        "trait_item" => {
            let mut name = String::new();
            let mut methods = Vec::new();

            if let Some(name_node) = node.child_by_field_name("name") {
                name = get_text(name_node);
            } else {
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind() == "type_identifier" {
                        name = get_text(child);
                        break;
                    }
                }
            }

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "declaration_list" {
                    let mut dec_cursor = child.walk();
                    for inner in child.children(&mut dec_cursor) {
                        if inner.kind() == "function_item"
                            || inner.kind() == "function_signature_item"
                        {
                            if let Some(m_name) = inner.child_by_field_name("name") {
                                let sig = if let Some(body) = inner.child_by_field_name("body") {
                                    content
                                        .get(inner.start_byte()..body.start_byte())
                                        .unwrap_or("")
                                        .trim()
                                        .to_string()
                                } else {
                                    get_text(inner)
                                };
                                methods.push(SymbolMethod {
                                    name: get_text(m_name),
                                    signature: sig,
                                });
                            }
                        }
                    }
                }
            }

            symbols.push(ParsedSymbol {
                name,
                kind: "trait".to_string(),
                file_path: file_path.to_string(),
                start_line,
                end_line,
                content: node_content,
                doc_comment,
                fields: None,
                variants: None,
                methods: Some(methods),
                implements_trait: None,
                target_type: None,
            });
        }
        "impl_item" => {
            let mut implements_trait = None;
            let mut target_type = String::new();
            let mut methods = Vec::new();

            let mut is_for = false;
            let mut type_nodes = Vec::new();

            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "declaration_list" {
                    break;
                }
                if child.kind() == "for"
                    || child.kind() == "type_identifier"
                    || child.kind() == "generic_type"
                    || child.kind() == "primitive_type"
                    || child.kind() == "scoped_type_identifier"
                {
                    if child.kind() == "for" || get_text(child) == "for" {
                        is_for = true;
                    } else {
                        type_nodes.push(child);
                    }
                }
            }

            if is_for && type_nodes.len() >= 2 {
                implements_trait = Some(get_text(type_nodes[0]));
                target_type = get_text(type_nodes[1]);
            } else if !type_nodes.is_empty() {
                target_type = get_text(type_nodes[type_nodes.len() - 1]);
            }

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "declaration_list" {
                    let mut dec_cursor = child.walk();
                    for inner in child.children(&mut dec_cursor) {
                        if inner.kind() == "function_item" {
                            if let Some(m_name) = inner.child_by_field_name("name") {
                                let sig = if let Some(body) = inner.child_by_field_name("body") {
                                    content
                                        .get(inner.start_byte()..body.start_byte())
                                        .unwrap_or("")
                                        .trim()
                                        .to_string()
                                } else {
                                    get_text(inner)
                                };
                                methods.push(SymbolMethod {
                                    name: get_text(m_name),
                                    signature: sig,
                                });
                            }
                        }
                    }
                }
            }

            let name = if let Some(ref tr) = implements_trait {
                format!("impl {} for {}", tr, target_type)
            } else {
                format!("impl {}", target_type)
            };

            symbols.push(ParsedSymbol {
                name,
                kind: "impl".to_string(),
                file_path: file_path.to_string(),
                start_line,
                end_line,
                content: node_content,
                doc_comment,
                fields: None,
                variants: None,
                methods: Some(methods),
                implements_trait,
                target_type: Some(target_type),
            });
        }
        "function_item" => {
            let mut name = String::new();
            if let Some(name_node) = node.child_by_field_name("name") {
                name = get_text(name_node);
            }

            if !name.is_empty() {
                let is_standalone = if let Some(parent) = node.parent() {
                    parent.kind() == "source_file"
                } else {
                    true
                };

                if is_standalone {
                    symbols.push(ParsedSymbol {
                        name,
                        kind: "function".to_string(),
                        file_path: file_path.to_string(),
                        start_line,
                        end_line,
                        content: node_content,
                        doc_comment,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            }
        }
        _ => {}
    }

    if kind == "source_file" || kind == "mod_item" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            traverse_nodes(child, content, file_path, symbols);
        }
    }
}

pub struct AstCompactor {
    parser: Parser,
}

impl AstCompactor {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(language())
            .expect("Error loading Rust grammar");
        Self { parser }
    }

    pub fn extract_signatures(&mut self, code: &str) -> String {
        if let Some(tree) = self.parser.parse(code, None) {
            let root = tree.root_node();
            let mut compacted = String::new();
            let mut cursor = root.walk();
            for child in root.children(&mut cursor) {
                let kind = child.kind();
                match kind {
                    "struct_item" | "enum_item" | "trait_item" | "type_item" => {
                        let text = code.get(child.start_byte()..child.end_byte()).unwrap_or("");
                        compacted.push_str(text);
                        compacted.push_str("\n\n");
                    }
                    "function_item" => {
                        if let Some(body) = child.child_by_field_name("body") {
                            let sig = code
                                .get(child.start_byte()..body.start_byte())
                                .unwrap_or("")
                                .trim();
                            compacted.push_str(sig);
                            compacted.push_str(" { /* ... */ }\n\n");
                        } else {
                            let text = code.get(child.start_byte()..child.end_byte()).unwrap_or("");
                            compacted.push_str(text);
                            compacted.push_str("\n\n");
                        }
                    }
                    "impl_item" => {
                        // Include impl header and inner function signatures
                        let mut impl_str = String::new();
                        if let Some(body) = child.child_by_field_name("body") {
                            let header = code
                                .get(child.start_byte()..body.start_byte())
                                .unwrap_or("")
                                .trim();
                            impl_str.push_str(header);
                            impl_str.push_str(" {\n");
                            let mut body_cursor = body.walk();
                            for inner in body.children(&mut body_cursor) {
                                if inner.kind() == "function_item" {
                                    if let Some(inner_body) = inner.child_by_field_name("body") {
                                        let sig = code
                                            .get(inner.start_byte()..inner_body.start_byte())
                                            .unwrap_or("")
                                            .trim();
                                        impl_str.push_str("    ");
                                        impl_str.push_str(sig);
                                        impl_str.push_str(" { /* ... */ }\n");
                                    }
                                }
                            }
                            impl_str.push_str("}\n\n");
                            compacted.push_str(&impl_str);
                        } else {
                            let text = code.get(child.start_byte()..child.end_byte()).unwrap_or("");
                            compacted.push_str(text);
                            compacted.push_str("\n\n");
                        }
                    }
                    "use_declaration" => {
                        let text = code.get(child.start_byte()..child.end_byte()).unwrap_or("");
                        compacted.push_str(text);
                        compacted.push_str("\n");
                    }
                    _ => {}
                }
            }
            if !compacted.trim().is_empty() {
                return compacted;
            }
        }
        code.to_string()
    }
}

impl Default for AstCompactor {
    fn default() -> Self {
        Self::new()
    }
}
