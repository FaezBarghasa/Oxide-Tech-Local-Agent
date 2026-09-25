use crate::ast::ParsedSymbol;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    // Code
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    Kotlin,
    C,
    Cpp,
    SystemVerilog,
    OpenScad,
    Mojo,
    Slint,
    Markdown,
    Xml,
    Bash,
    Css,
    Html,
    Json,
    Yaml,
    Toml,
    Gradle,
    Docker,
    Qemu,
    Cfg,
    Csv,
    Svg,
    Svd,
    LinkerScript,
    Assembly,

    // Media & Assets (Metadata indexing)
    Image(ImageFormat),
    Audio(AudioFormat),
    Video(VideoFormat),
    Document(DocFormat),

    Unknown,
}

pub type LanguageId = Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Svg,
    Webp,
    Gif,
    Ico,
    Bmp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioFormat {
    Mp3,
    Wav,
    Flac,
    Ogg,
    Aac,
    M4a,
    Midi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoFormat {
    Mp4,
    Mkv,
    Webm,
    Avi,
    Mov,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocFormat {
    Pdf,
    Docx,
    Epub,
}

impl Language {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let p = path.as_ref();
        let file_name = p
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        if file_name == "dockerfile" || file_name.starts_with("dockerfile.") {
            return Self::Docker;
        }
        if file_name.ends_with(".gradle") || file_name.ends_with(".gradle.kts") {
            return Self::Gradle;
        }

        let ext = p
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            // Code
            "rs" => Self::Rust,
            "c" | "h" => Self::C,
            "cpp" | "hpp" | "cc" | "cxx" | "hh" => Self::Cpp,
            "sv" | "svh" | "v" | "vh" => Self::SystemVerilog,
            "scad" => Self::OpenScad,
            "py" | "pyi" => Self::Python,
            "js" | "mjs" | "cjs" | "jsx" => Self::JavaScript,
            "ts" | "mts" | "cts" | "tsx" => Self::TypeScript,
            "go" => Self::Go,
            "java" => Self::Java,
            "kt" | "kts" => Self::Kotlin,
            "mojo" | "🔥" => Self::Mojo,
            "slint" => Self::Slint,
            "sh" | "bash" | "zsh" => Self::Bash,
            "css" | "scss" | "sass" | "less" => Self::Css,
            "html" | "htm" => Self::Html,
            "xml" => Self::Xml,
            "gradle" => Self::Gradle,
            "dockerfile" => Self::Docker,
            "qemu" => Self::Qemu,
            "svd" => Self::Svd,
            "ld" => Self::LinkerScript,
            "s" | "asm" => Self::Assembly,

            // Data & Config
            "json" | "jsonc" | "json5" => Self::Json,
            "yaml" | "yml" => Self::Yaml,
            "toml" => Self::Toml,
            "cfg" | "ini" | "conf" => Self::Cfg,
            "csv" | "tsv" => Self::Csv,
            "md" | "markdown" => Self::Markdown,

            // Images
            "png" => Self::Image(ImageFormat::Png),
            "jpg" | "jpeg" => Self::Image(ImageFormat::Jpeg),
            "svg" => Self::Svg,
            "webp" => Self::Image(ImageFormat::Webp),
            "gif" => Self::Image(ImageFormat::Gif),
            "ico" => Self::Image(ImageFormat::Ico),
            "bmp" => Self::Image(ImageFormat::Bmp),

            // Audio / Sound
            "mp3" => Self::Audio(AudioFormat::Mp3),
            "wav" => Self::Audio(AudioFormat::Wav),
            "flac" => Self::Audio(AudioFormat::Flac),
            "ogg" | "oga" => Self::Audio(AudioFormat::Ogg),
            "aac" => Self::Audio(AudioFormat::Aac),
            "m4a" => Self::Audio(AudioFormat::M4a),
            "mid" | "midi" => Self::Audio(AudioFormat::Midi),

            // Video
            "mp4" | "m4v" => Self::Video(VideoFormat::Mp4),
            "mkv" => Self::Video(VideoFormat::Mkv),
            "webm" => Self::Video(VideoFormat::Webm),
            "avi" => Self::Video(VideoFormat::Avi),
            "mov" => Self::Video(VideoFormat::Mov),

            // Documents
            "pdf" => Self::Document(DocFormat::Pdf),
            "docx" => Self::Document(DocFormat::Docx),
            "epub" => Self::Document(DocFormat::Epub),

            _ => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::SystemVerilog => "systemverilog",
            Self::OpenScad => "openscad",
            Self::Python => "python",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Go => "go",
            Self::Java => "java",
            Self::Kotlin => "kotlin",
            Self::Mojo => "mojo",
            Self::Slint => "slint",
            Self::Bash => "bash",
            Self::Css => "css",
            Self::Html => "html",
            Self::Xml => "xml",
            Self::Gradle => "gradle",
            Self::Docker => "docker",
            Self::Qemu => "qemu",
            Self::Svd => "svd",
            Self::LinkerScript => "linkerscript",
            Self::Assembly => "assembly",
            Self::Json => "json",
            Self::Yaml => "yaml",
            Self::Toml => "toml",
            Self::Cfg => "cfg",
            Self::Csv => "csv",
            Self::Markdown => "markdown",
            Self::Svg => "svg",
            Self::Image(_) => "image",
            Self::Audio(_) => "audio",
            Self::Video(_) => "video",
            Self::Document(_) => "document",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_binary(&self) -> bool {
        match self {
            Self::Svg => false,
            Self::Image(_) | Self::Audio(_) | Self::Video(_) | Self::Document(_) => true,
            _ => false,
        }
    }
}

pub trait LanguageParser: Send + Sync {
    fn parse(&self, content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String>;
}

pub struct RustLanguageParser;

impl LanguageParser for RustLanguageParser {
    fn parse(&self, content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String> {
        crate::parser::parse_rust_content(content, file_path)
    }
}

pub struct PythonLanguageParser;

impl LanguageParser for PythonLanguageParser {
    fn parse(&self, content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_class: Option<String> = None;
        let mut current_class_indent = 0;

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let indent = line.chars().take_while(|c| c.is_whitespace()).count();
            if current_class.is_some() && indent <= current_class_indent {
                current_class = None;
            }

            let line_no = idx + 1;

            if trimmed.starts_with("class ") {
                let rest = trimmed.trim_start_matches("class ").trim();
                let class_name = rest
                    .split(['(', ':'])
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();

                if !class_name.is_empty() {
                    current_class = Some(class_name.clone());
                    current_class_indent = indent;

                    let doc = extract_python_doc(&lines, idx + 1);

                    symbols.push(ParsedSymbol {
                        name: class_name,
                        kind: "class".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: doc,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            } else if trimmed.starts_with("def ") {
                let rest = trimmed.trim_start_matches("def ").trim();
                let fn_name = rest.split('(').next().unwrap_or("").trim().to_string();

                if !fn_name.is_empty() {
                    let doc = extract_python_doc(&lines, idx + 1);
                    let (kind, target_type) = if let Some(ref cls) = current_class {
                        ("method".to_string(), Some(cls.clone()))
                    } else {
                        ("function".to_string(), None)
                    };

                    symbols.push(ParsedSymbol {
                        name: fn_name,
                        kind,
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: doc,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type,
                    });
                }
            }
        }

        Ok(symbols)
    }
}

fn extract_python_doc(lines: &[&str], start_idx: usize) -> Option<String> {
    if start_idx >= lines.len() {
        return None;
    }
    let first = lines[start_idx].trim();
    if first.starts_with("\"\"\"") || first.starts_with("'''") {
        let quote = &first[0..3];
        if first.len() > 3 && first[3..].contains(quote) {
            return Some(
                first
                    .trim_start_matches(quote)
                    .trim_end_matches(quote)
                    .trim()
                    .to_string(),
            );
        }
        let mut doc = Vec::new();
        doc.push(first.trim_start_matches(quote));
        for line in lines.iter().skip(start_idx + 1) {
            if line.contains(quote) {
                let end_part = line.split(quote).next().unwrap_or("");
                doc.push(end_part);
                break;
            }
            doc.push(line.trim());
        }
        return Some(doc.join("\n").trim().to_string());
    }
    None
}

pub struct CAndCppLanguageParser;

impl LanguageParser for CAndCppLanguageParser {
    fn parse(&self, content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("/*") {
                continue;
            }

            let line_no = idx + 1;

            if (trimmed.starts_with("struct ") || trimmed.starts_with("typedef struct "))
                && (trimmed.contains('{') || trimmed.ends_with(';'))
            {
                let cleaned = trimmed
                    .trim_start_matches("typedef ")
                    .trim_start_matches("struct ");
                let name = cleaned
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_end_matches('{')
                    .trim_end_matches(';')
                    .trim();

                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: "struct".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            } else if (trimmed.starts_with("class ") || trimmed.starts_with("class __declspec"))
                && (trimmed.contains('{') || trimmed.contains(':'))
            {
                let cleaned = trimmed.trim_start_matches("class ");
                let name = cleaned
                    .split([':', '{', ' '])
                    .next()
                    .unwrap_or("")
                    .trim();

                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: "class".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            } else if trimmed.starts_with("enum ") && trimmed.contains('{') {
                let cleaned = trimmed.trim_start_matches("enum ");
                let name = cleaned
                    .trim_start_matches("class ")
                    .split(['{', ':', ' '])
                    .next()
                    .unwrap_or("")
                    .trim();

                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: "enum".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            } else if trimmed.starts_with("#define ") {
                let rest = trimmed.trim_start_matches("#define ").trim();
                let macro_name = rest
                    .split(['(', ' ', '\t'])
                    .next()
                    .unwrap_or("")
                    .trim();

                if !macro_name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: macro_name.to_string(),
                        kind: "macro".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            } else if trimmed.contains('(')
                && (trimmed.ends_with('{') || trimmed.contains(") {"))
                && !trimmed.starts_with("if ")
                && !trimmed.starts_with("for ")
                && !trimmed.starts_with("while ")
                && !trimmed.starts_with("switch ")
            {
                let before_paren = trimmed.split('(').next().unwrap_or("").trim();
                let fn_name = before_paren
                    .split(['*', ' ', '&', ':'])
                    .next_back()
                    .unwrap_or("")
                    .trim();

                if !fn_name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: fn_name.to_string(),
                        kind: "function".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            }
        }

        Ok(symbols)
    }
}

pub struct SystemVerilogParser;

impl LanguageParser for SystemVerilogParser {
    fn parse(&self, content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            let line_no = idx + 1;

            if trimmed.starts_with("module ") {
                let rest = trimmed.trim_start_matches("module ").trim();
                let name = rest
                    .split(['#', '(', ';', ' '])
                    .next()
                    .unwrap_or("")
                    .trim();
                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: "module".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            } else if trimmed.starts_with("interface ") {
                let rest = trimmed.trim_start_matches("interface ").trim();
                let name = rest
                    .split(['#', '(', ';', ' '])
                    .next()
                    .unwrap_or("")
                    .trim();
                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: "interface".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            } else if trimmed.starts_with("package ") {
                let rest = trimmed.trim_start_matches("package ").trim();
                let name = rest.split([';', ' ']).next().unwrap_or("").trim();
                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: "package".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            } else if trimmed.starts_with("task ") || trimmed.starts_with("function ") {
                let is_task = trimmed.starts_with("task ");
                let keyword = if is_task { "task " } else { "function " };
                let rest = trimmed.trim_start_matches(keyword).trim();
                let cleaned = rest.trim_start_matches("automatic ").trim();
                let before_paren = cleaned.split('(').next().unwrap_or("").trim();
                let name = before_paren.split_whitespace().next_back().unwrap_or("").trim_end_matches(';');

                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: if is_task { "task" } else { "function" }.to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            }
        }

        Ok(symbols)
    }
}

pub struct OpenScadParser;

impl LanguageParser for OpenScadParser {
    fn parse(&self, content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            let line_no = idx + 1;

            if trimmed.starts_with("module ") {
                let rest = trimmed.trim_start_matches("module ").trim();
                let name = rest.split('(').next().unwrap_or("").trim();
                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: "module".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            } else if trimmed.starts_with("function ") {
                let rest = trimmed.trim_start_matches("function ").trim();
                let name = rest.split('(').next().unwrap_or("").trim();
                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: "function".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            }
        }

        Ok(symbols)
    }
}

pub struct GenericConfigParser;

impl LanguageParser for GenericConfigParser {
    fn parse(&self, content: &str, file_path: &str) -> Result<Vec<ParsedSymbol>, String> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let line_no = idx + 1;

            if trimmed.starts_with('#') && !trimmed.starts_with("#!") && !trimmed.starts_with("#include") {
                let name = trimmed.trim_start_matches('#').trim();
                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: "section".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
                let name = trimmed.trim_start_matches('[').trim_end_matches(']').trim();
                if !name.is_empty() {
                    symbols.push(ParsedSymbol {
                        name: name.to_string(),
                        kind: "table".to_string(),
                        file_path: file_path.to_string(),
                        start_line: line_no,
                        end_line: line_no,
                        content: trimmed.to_string(),
                        doc_comment: None,
                        fields: None,
                        variants: None,
                        methods: None,
                        implements_trait: None,
                        target_type: None,
                    });
                }
            }
        }

        Ok(symbols)
    }
}

pub fn get_parser(lang: LanguageId) -> Box<dyn LanguageParser> {
    match lang {
        Language::Rust => Box::new(RustLanguageParser),
        Language::Python | Language::Mojo => Box::new(PythonLanguageParser),
        Language::C | Language::Cpp => Box::new(CAndCppLanguageParser),
        Language::SystemVerilog => Box::new(SystemVerilogParser),
        Language::OpenScad => Box::new(OpenScadParser),
        _ => Box::new(GenericConfigParser),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_file;

    #[test]
    fn test_language_detection() {
        assert_eq!(Language::from_path("main.rs"), Language::Rust);
        assert_eq!(Language::from_path("script.py"), Language::Python);
        assert_eq!(Language::from_path("driver.c"), Language::C);
        assert_eq!(Language::from_path("header.hpp"), Language::Cpp);
        assert_eq!(Language::from_path("alu.sv"), Language::SystemVerilog);
        assert_eq!(Language::from_path("model.scad"), Language::OpenScad);
        assert_eq!(Language::from_path("config.toml"), Language::Toml);
    }

    #[test]
    fn test_python_parsing() {
        let py_code = r#"
class MotorController:
    """Controls motor speeds and kinematics."""
    def __init__(self, port):
        self.port = port

    def set_speed(self, speed: float):
        """Set RPM velocity."""
        pass

def global_health_check():
    return True
"#;
        let symbols = parse_file(py_code, "controller.py").expect("parse python");
        assert_eq!(symbols.len(), 4);
        assert_eq!(symbols[0].name, "MotorController");
        assert_eq!(symbols[0].kind, "class");
        assert_eq!(symbols[0].doc_comment.as_deref(), Some("Controls motor speeds and kinematics."));

        assert_eq!(symbols[1].name, "__init__");
        assert_eq!(symbols[1].kind, "method");
        assert_eq!(symbols[1].target_type.as_deref(), Some("MotorController"));

        assert_eq!(symbols[2].name, "set_speed");
        assert_eq!(symbols[2].kind, "method");

        assert_eq!(symbols[3].name, "global_health_check");
        assert_eq!(symbols[3].kind, "function");
    }

    #[test]
    fn test_c_cpp_parsing() {
        let c_code = r#"
#define MAX_BUFFER_SIZE 4096

struct CanPacket {
    uint32_t id;
    uint8_t data[8];
};

enum SystemStatus {
    STATUS_OK = 0,
    STATUS_FAULT = 1
};

int send_can_packet(struct CanPacket* pkt) {
    return 0;
}
"#;
        let symbols = parse_file(c_code, "can_driver.c").expect("parse c");
        assert!(symbols.iter().any(|s| s.name == "MAX_BUFFER_SIZE" && s.kind == "macro"));
        assert!(symbols.iter().any(|s| s.name == "CanPacket" && s.kind == "struct"));
        assert!(symbols.iter().any(|s| s.name == "SystemStatus" && s.kind == "enum"));
        assert!(symbols.iter().any(|s| s.name == "send_can_packet" && s.kind == "function"));
    }

    #[test]
    fn test_systemverilog_parsing() {
        let sv_code = r#"
module spi_master #(
    parameter CLK_DIV = 4
)(
    input logic clk,
    input logic rst_n
);
endmodule

interface axi_stream_if;
endinterface

task automatic reset_bus();
endtask
"#;
        let symbols = parse_file(sv_code, "spi_master.sv").expect("parse sv");
        assert!(symbols.iter().any(|s| s.name == "spi_master" && s.kind == "module"));
        assert!(symbols.iter().any(|s| s.name == "axi_stream_if" && s.kind == "interface"));
        assert!(symbols.iter().any(|s| s.name == "reset_bus" && s.kind == "task"));
    }

    #[test]
    fn test_openscad_parsing() {
        let scad_code = r#"
module motor_mount(width, height) {
    cube([width, height, 10]);
}

function calculate_pitch(dia, teeth) = dia / teeth;
"#;
        let symbols = parse_file(scad_code, "mount.scad").expect("parse scad");
        assert!(symbols.iter().any(|s| s.name == "motor_mount" && s.kind == "module"));
        assert!(symbols.iter().any(|s| s.name == "calculate_pitch" && s.kind == "function"));
    }

    #[test]
    fn test_rust_parsing() {
        let rs_code = r#"
/// Device telemetry packet
pub struct TelemetryPacket {
    pub seq: u64,
}

pub fn handle_packet(pkt: &TelemetryPacket) -> bool {
    true
}
"#;
        let symbols = parse_file(rs_code, "lib.rs").expect("parse rust");
        assert!(symbols.iter().any(|s| s.name == "TelemetryPacket" && s.kind == "struct"));
        assert!(symbols.iter().any(|s| s.name == "handle_packet" && s.kind == "function"));
    }
}

