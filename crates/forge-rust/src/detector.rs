use serde::{Deserialize, Serialize};

/// Supported source languages for lifting to Polyglot UIR and refactoring into Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceLanguage {
    C,
    Cpp,
    Python,
    TypeScript,
    JavaScript,
    Go,
    Java,
    Rust,
    Generic,
}

impl SourceLanguage {
    pub fn name(&self) -> &'static str {
        match self {
            Self::C => "C",
            Self::Cpp => "C++",
            Self::Python => "Python",
            Self::TypeScript => "TypeScript",
            Self::JavaScript => "JavaScript",
            Self::Go => "Go",
            Self::Java => "Java",
            Self::Rust => "Rust",
            Self::Generic => "Generic",
        }
    }
}

/// Detects the source language based on file path extension or source code content heuristics.
pub struct LanguageDetector;

impl LanguageDetector {
    pub fn detect(source: &str, file_path: Option<&str>) -> SourceLanguage {
        if let Some(path) = file_path
            && let Some(lang) = Self::detect_by_extension(path)
        {
            return lang;
        }
        Self::detect_by_content(source)
    }

    pub fn detect_by_extension(path: &str) -> Option<SourceLanguage> {
        let p = path.to_lowercase();
        if p.ends_with(".c") || p.ends_with(".h") {
            Some(SourceLanguage::C)
        } else if p.ends_with(".cpp") || p.ends_with(".cc") || p.ends_with(".cxx") || p.ends_with(".hpp") {
            Some(SourceLanguage::Cpp)
        } else if p.ends_with(".py") || p.ends_with(".pyi") {
            Some(SourceLanguage::Python)
        } else if p.ends_with(".ts") || p.ends_with(".tsx") {
            Some(SourceLanguage::TypeScript)
        } else if p.ends_with(".js") || p.ends_with(".jsx") || p.ends_with(".mjs") {
            Some(SourceLanguage::JavaScript)
        } else if p.ends_with(".go") {
            Some(SourceLanguage::Go)
        } else if p.ends_with(".java") {
            Some(SourceLanguage::Java)
        } else if p.ends_with(".rs") {
            Some(SourceLanguage::Rust)
        } else {
            None
        }
    }

    pub fn detect_by_content(source: &str) -> SourceLanguage {
        let trimmed = source.trim();
        if trimmed.starts_with("#!/usr/bin/env python") || trimmed.starts_with("#!/usr/bin/python") {
            return SourceLanguage::Python;
        }
        if trimmed.starts_with("#!/usr/bin/env node") {
            return SourceLanguage::JavaScript;
        }

        // Check Go patterns
        if trimmed.starts_with("package ") || (source.contains("func ") && source.contains("package ")) {
            return SourceLanguage::Go;
        }

        // Check Python patterns
        if source.contains("def ") && (source.contains(":\n") || source.contains("import ")) && !source.contains("fn ")
            && !source.contains("function ") && !source.contains("public class ")
        {
            return SourceLanguage::Python;
        }

        // Check TypeScript / JavaScript patterns
        if source.contains("interface ") || source.contains("export const ") || source.contains("export default ") || source.contains("import {") {
            if source.contains(": string") || source.contains(": number") || source.contains(": boolean") || source.contains("interface ") {
                return SourceLanguage::TypeScript;
            }
            return SourceLanguage::JavaScript;
        }

        // Check Java patterns
        if source.contains("public class ") || source.contains("public static void main") || source.contains("package com.") {
            return SourceLanguage::Java;
        }

        // Check C / C++ patterns
        if source.contains("#include <") || source.contains("#include \"") {
            if source.contains("class ") || source.contains("std::") || source.contains("template<") || source.contains("namespace ") {
                return SourceLanguage::Cpp;
            }
            return SourceLanguage::C;
        }

        // Fallback checks
        if source.contains("fn main()") || source.contains("pub struct ") || source.contains("impl ") {
            return SourceLanguage::Rust;
        }

        SourceLanguage::Generic
    }
}
