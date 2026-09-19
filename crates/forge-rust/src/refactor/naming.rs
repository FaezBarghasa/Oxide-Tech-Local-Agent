use crate::ir::{UirItem, UirModule};
use crate::refactor::RefactorPass;

/// Normalizes identifier names to idiomatic Rust conventions:
/// - Functions & variables: snake_case
/// - Structs, Enums, Traits: PascalCase
/// - Constants: SCREAMING_SNAKE_CASE
pub struct NamingPass;

impl RefactorPass for NamingPass {
    fn run(&self, module: &mut UirModule) {
        for item in &mut module.items {
            match item {
                UirItem::Struct(s) => {
                    s.name = to_pascal_case(&s.name);
                    for field in &mut s.fields {
                        field.name = to_snake_case(&field.name);
                    }
                    for method in &mut s.methods {
                        method.name = to_snake_case(&method.name);
                        for param in &mut method.params {
                            param.name = to_snake_case(&param.name);
                        }
                    }
                }
                UirItem::Enum(e) => {
                    e.name = to_pascal_case(&e.name);
                    for variant in &mut e.variants {
                        variant.name = to_pascal_case(&variant.name);
                    }
                }
                UirItem::Function(f) => {
                    f.name = to_snake_case(&f.name);
                    for param in &mut f.params {
                        param.name = to_snake_case(&param.name);
                    }
                }
                UirItem::Trait(t) => {
                    t.name = to_pascal_case(&t.name);
                    for method in &mut t.methods {
                        method.name = to_snake_case(&method.name);
                        for param in &mut method.params {
                            param.name = to_snake_case(&param.name);
                        }
                    }
                }
                UirItem::Const(c) => {
                    c.name = to_screaming_snake_case(&c.name);
                }
                UirItem::RawBlock(_) => {}
            }
        }
    }
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_is_upper = false;

    for (i, c) in s.chars().enumerate() {
        if c == '-' || c == ' ' {
            result.push('_');
            prev_is_upper = false;
        } else if c.is_uppercase() {
            if i > 0 && !prev_is_upper && !result.ends_with('_') {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_is_upper = true;
        } else {
            result.push(c);
            prev_is_upper = false;
        }
    }

    // Keyword escape
    match result.as_str() {
        "type" => "r#type".to_string(),
        "match" => "r#match".to_string(),
        "fn" => "r#fn".to_string(),
        "struct" => "r#struct".to_string(),
        "enum" => "r#enum".to_string(),
        "trait" => "r#trait".to_string(),
        "move" => "r#move".to_string(),
        "ref" => "r#ref".to_string(),
        "in" => "r#in".to_string(),
        "loop" => "r#loop".to_string(),
        other => other.to_string(),
    }
}

pub fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

pub fn to_screaming_snake_case(s: &str) -> String {
    to_snake_case(s).to_ascii_uppercase()
}
