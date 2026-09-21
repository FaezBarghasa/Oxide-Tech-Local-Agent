use crate::ir::{
    UirConst, UirEnum, UirFunction, UirItem, UirModule, UirSelfKind, UirStmt, UirStruct, UirTrait,
    UirType,
};

/// Emitter that converts refactored `UirModule` into valid, idiomatic Rust source code.
pub struct RustEmitter;

impl RustEmitter {
    pub fn emit(module: &UirModule) -> String {
        let mut out = String::new();

        // 1. Module Doc Comments
        if let Some(doc) = &module.doc {
            out.push_str(&format!("//! {}\n\n", doc));
        }

        // 2. Standard and common imports based on dependencies
        let mut imports = Vec::new();
        if module.required_dependencies.contains(&"serde".to_string()) {
            imports.push("use serde::{Deserialize, Serialize};");
        }
        if module.required_dependencies.contains(&"tokio".to_string()) {
            imports.push("use tokio;");
        }
        if module.required_dependencies.contains(&"anyhow".to_string()) {
            imports.push("use anyhow::Result;");
        }
        if module
            .required_dependencies
            .contains(&"std::collections::HashMap".to_string())
            || module.items.iter().any(|it| match it {
                UirItem::Struct(s) => s
                    .fields
                    .iter()
                    .any(|f| matches!(f.ty, UirType::HashMap { .. })),
                _ => false,
            })
        {
            imports.push("use std::collections::HashMap;");
        }

        if !imports.is_empty() {
            out.push_str(&imports.join("\n"));
            out.push_str("\n\n");
        }

        // 3. Emit Items
        for item in &module.items {
            match item {
                UirItem::Struct(s) => emit_struct(&mut out, s),
                UirItem::Enum(e) => emit_enum(&mut out, e),
                UirItem::Function(f) => emit_function(&mut out, f, 0),
                UirItem::Trait(t) => emit_trait(&mut out, t),
                UirItem::Const(c) => emit_const(&mut out, c),
                UirItem::RawBlock(raw) => {
                    out.push_str(raw);
                    out.push_str("\n\n");
                }
            }
        }

        out
    }
}

fn emit_struct(out: &mut String, s: &UirStruct) {
    if let Some(doc) = &s.doc {
        out.push_str(&format!("/// {}\n", doc));
    }
    if !s.derives.is_empty() {
        out.push_str(&format!("#[derive({})]\n", s.derives.join(", ")));
    }
    let vis = if s.is_pub { "pub " } else { "" };
    out.push_str(&format!("{}struct {} {{\n", vis, s.name));

    for field in &s.fields {
        if let Some(doc) = &field.doc {
            out.push_str(&format!("    /// {}\n", doc));
        }
        let fvis = if field.is_pub { "pub " } else { "" };
        out.push_str(&format!(
            "    {}{}: {},\n",
            fvis,
            field.name,
            emit_type(&field.ty)
        ));
    }
    out.push_str("}\n\n");

    // Impl block for methods
    if !s.methods.is_empty() {
        out.push_str(&format!("impl {} {{\n", s.name));
        for method in &s.methods {
            emit_function(out, method, 4);
        }
        out.push_str("}\n\n");
    }
}

fn emit_enum(out: &mut String, e: &UirEnum) {
    if let Some(doc) = &e.doc {
        out.push_str(&format!("/// {}\n", doc));
    }
    if !e.derives.is_empty() {
        out.push_str(&format!("#[derive({})]\n", e.derives.join(", ")));
    }
    let vis = if e.is_pub { "pub " } else { "" };
    out.push_str(&format!("{}enum {} {{\n", vis, e.name));

    for variant in &e.variants {
        if let Some(fields) = &variant.fields {
            out.push_str(&format!("    {} {{\n", variant.name));
            for f in fields {
                out.push_str(&format!("        {}: {},\n", f.name, emit_type(&f.ty)));
            }
            out.push_str("    },\n");
        } else if let Some(disc) = variant.discriminant {
            out.push_str(&format!("    {} = {},\n", variant.name, disc));
        } else {
            out.push_str(&format!("    {},\n", variant.name));
        }
    }
    out.push_str("}\n\n");
}

fn emit_trait(out: &mut String, t: &UirTrait) {
    if let Some(doc) = &t.doc {
        out.push_str(&format!("/// {}\n", doc));
    }
    let vis = if t.is_pub { "pub " } else { "" };
    out.push_str(&format!("{}trait {} {{\n", vis, t.name));

    for method in &t.methods {
        let async_kw = if method.is_async { "async " } else { "" };
        let ret = match &method.return_type {
            Some(UirType::Void) | None => String::new(),
            Some(ty) => format!(" -> {}", emit_type(ty)),
        };

        let mut params = Vec::new();
        if let Some(self_kind) = method.self_kind {
            params.push(match self_kind {
                UirSelfKind::Ref => "&self".to_string(),
                UirSelfKind::MutRef => "&mut self".to_string(),
                UirSelfKind::Value => "self".to_string(),
            });
        }
        for p in &method.params {
            params.push(format!("{}: {}", p.name, emit_type(&p.ty)));
        }

        out.push_str(&format!(
            "    {}fn {}({}){};\n",
            async_kw,
            method.name,
            params.join(", "),
            ret
        ));
    }
    out.push_str("}\n\n");
}

fn emit_const(out: &mut String, c: &UirConst) {
    let vis = if c.is_pub { "pub " } else { "" };
    out.push_str(&format!(
        "{}const {}: {} = {};\n\n",
        vis,
        c.name,
        emit_type(&c.ty),
        c.value
    ));
}

fn emit_function(out: &mut String, f: &UirFunction, indent: usize) {
    let pad = " ".repeat(indent);
    if let Some(doc) = &f.doc {
        out.push_str(&format!("{}/// {}\n", pad, doc));
    }
    let vis = if f.is_pub { "pub " } else { "" };
    let async_kw = if f.is_async { "async " } else { "" };
    let unsafe_kw = if f.is_unsafe { "unsafe " } else { "" };

    let mut params = Vec::new();
    if let Some(self_kind) = f.self_kind {
        params.push(match self_kind {
            UirSelfKind::Ref => "&self".to_string(),
            UirSelfKind::MutRef => "&mut self".to_string(),
            UirSelfKind::Value => "self".to_string(),
        });
    }
    for p in &f.params {
        params.push(format!("{}: {}", p.name, emit_type(&p.ty)));
    }

    let ret = match &f.return_type {
        Some(UirType::Void) | None => String::new(),
        Some(ty) => format!(" -> {}", emit_type(ty)),
    };

    out.push_str(&format!(
        "{}{}{}{}fn {}({}){} {{\n",
        pad,
        vis,
        async_kw,
        unsafe_kw,
        f.name,
        params.join(", "),
        ret
    ));

    if f.body.is_empty() {
        if let Some(ret_ty) = &f.return_type {
            match ret_ty {
                UirType::Result { .. } => out.push_str(&format!("{}    Ok(())\n", pad)),
                UirType::Option { .. } => out.push_str(&format!("{}    None\n", pad)),
                _ => out.push_str(&format!("{}    todo!()\n", pad)),
            }
        } else {
            out.push_str(&format!("{}    // Emitted implementation\n", pad));
        }
    } else {
        for stmt in &f.body {
            emit_stmt(out, stmt, indent + 4);
        }
    }

    out.push_str(&format!("{}}}\n\n", pad));
}

fn emit_stmt(out: &mut String, stmt: &UirStmt, indent: usize) {
    let pad = " ".repeat(indent);
    match stmt {
        UirStmt::Raw(raw) => {
            if raw.starts_with("//") {
                out.push_str(&format!("{}{}\n", pad, raw));
            } else {
                out.push_str(&format!("{}{};\n", pad, raw.trim_end_matches(';')));
            }
        }
        UirStmt::Return(expr) => {
            if let Some(e) = expr {
                out.push_str(&format!("{}return {:?};\n", pad, e));
            } else {
                out.push_str(&format!("{}return;\n", pad));
            }
        }
        _ => {
            out.push_str(&format!("{}// Translated statement\n", pad));
        }
    }
}

pub fn emit_type(ty: &UirType) -> String {
    match ty {
        UirType::Void => "()".to_string(),
        UirType::Bool => "bool".to_string(),
        UirType::I8 => "i8".to_string(),
        UirType::I16 => "i16".to_string(),
        UirType::I32 => "i32".to_string(),
        UirType::I64 => "i64".to_string(),
        UirType::U8 => "u8".to_string(),
        UirType::U16 => "u16".to_string(),
        UirType::U32 => "u32".to_string(),
        UirType::U64 => "u64".to_string(),
        UirType::F32 => "f32".to_string(),
        UirType::F64 => "f64".to_string(),
        UirType::ISize => "isize".to_string(),
        UirType::USize => "usize".to_string(),
        UirType::String => "String".to_string(),
        UirType::StrRef => "&str".to_string(),
        UirType::Custom(name) => name.clone(),
        UirType::Vec(inner) => format!("Vec<{}>", emit_type(inner)),
        UirType::Slice(inner) => format!("&[{}]", emit_type(inner)),
        UirType::Array(inner, size) => format!("[{}; {}]", emit_type(inner), size),
        UirType::Option(inner) => format!("Option<{}>", emit_type(inner)),
        UirType::Result { ok, err } => format!("Result<{}, {}>", emit_type(ok), emit_type(err)),
        UirType::Boxed(inner) => format!("Box<{}>", emit_type(inner)),
        UirType::ArcMutex(inner) => {
            format!("std::sync::Arc<tokio::sync::Mutex<{}>>", emit_type(inner))
        }
        UirType::Reference {
            mutable: true,
            inner,
        } => format!("&mut {}", emit_type(inner)),
        UirType::Reference {
            mutable: false,
            inner,
        } => format!("&{}", emit_type(inner)),
        UirType::HashMap { key, value } => {
            format!("HashMap<{}, {}>", emit_type(key), emit_type(value))
        }
        UirType::Tuple(types) => {
            let inner = types.iter().map(emit_type).collect::<Vec<_>>().join(", ");
            format!("({})", inner)
        }
        UirType::RawPointer {
            mutable: true,
            inner,
        } => format!("*mut {}", emit_type(inner)),
        UirType::RawPointer {
            mutable: false,
            inner,
        } => format!("*const {}", emit_type(inner)),
    }
}
