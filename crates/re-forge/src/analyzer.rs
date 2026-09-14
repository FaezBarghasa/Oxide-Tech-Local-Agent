use anyhow::{anyhow, Context, Result};
use goblin::Object;
use serde::{Deserialize, Serialize};
use std::path::Path;
use yaxpeax_arch::Decoder;
use yaxpeax_x86::long_mode as x86;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryFormat {
    Elf64,
    Elf32,
    Pe64,
    Pe32,
    MachO64,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisassembledInstruction {
    pub address: u64,
    pub mnemonic: String,
    pub length: usize,
    pub is_branch: bool,
    pub is_call: bool,
    pub is_return: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisassembledFunction {
    pub name: String,
    pub start_address: u64,
    pub end_address: u64,
    pub instructions: Vec<DisassembledInstruction>,
}

pub struct BinaryAnalyzer {
    pub format: BinaryFormat,
    pub entry_point: u64,
    pub functions: Vec<DisassembledFunction>,
}

impl BinaryAnalyzer {
    pub fn analyze_file(path: impl AsRef<Path>) -> Result<Self> {
        let buffer = std::fs::read(path.as_ref())
            .with_context(|| format!("Failed to read binary at {:?}", path.as_ref()))?;
        Self::analyze_bytes(&buffer)
    }

    pub fn analyze_bytes(buffer: &[u8]) -> Result<Self> {
        let parsed = Object::parse(buffer).context("Failed to parse binary with goblin")?;

        match parsed {
            Object::Elf(elf) => {
                let format = if elf.is_64 {
                    BinaryFormat::Elf64
                } else {
                    BinaryFormat::Elf32
                };

                let mut functions = Vec::new();

                // Look for executable .text section
                if let Some(text_section) = elf.section_headers.iter().find(|s| {
                    elf.shdr_strtab.get_at(s.sh_name) == Some(".text")
                }) {
                    let offset = text_section.sh_offset as usize;
                    let size = text_section.sh_size as usize;
                    let addr = text_section.sh_addr;

                    if offset + size <= buffer.len() {
                        let code_bytes = &buffer[offset..offset + size];
                        let mut decoder = x86::InstDecoder::default();
                        let mut curr_addr = addr;
                        let mut instructions = Vec::new();

                        let mut byte_offset = 0;
                        while byte_offset < code_bytes.len() {
                            match decoder.decode_slice(&code_bytes[byte_offset..]) {
                                Ok(inst) => {
                                    let len = inst.len().to_bytes() as usize;
                                    let mnemonic = format!("{}", inst);
                                    let is_branch = mnemonic.starts_with('j');
                                    let is_call = mnemonic.starts_with("call");
                                    let is_return = mnemonic.starts_with("ret");

                                    instructions.push(DisassembledInstruction {
                                        address: curr_addr,
                                        mnemonic,
                                        length: len,
                                        is_branch,
                                        is_call,
                                        is_return,
                                    });

                                    curr_addr += len as u64;
                                    byte_offset += len;
                                }
                                Err(_) => {
                                    byte_offset += 1;
                                    curr_addr += 1;
                                }
                            }
                        }

                        functions.push(DisassembledFunction {
                            name: "_text_main".to_string(),
                            start_address: addr,
                            end_address: curr_addr,
                            instructions,
                        });
                    }
                }

                Ok(Self {
                    format,
                    entry_point: elf.entry,
                    functions,
                })
            }
            Object::PE(pe) => {
                let format = if pe.is_64 {
                    BinaryFormat::Pe64
                } else {
                    BinaryFormat::Pe32
                };

                Ok(Self {
                    format,
                    entry_point: pe.entry as u64,
                    functions: Vec::new(),
                })
            }
            Object::Mach(mach) => {
                let format = match mach {
                    goblin::mach::Mach::Binary(m) if m.is_64 => BinaryFormat::MachO64,
                    _ => BinaryFormat::Unknown,
                };
                Ok(Self {
                    format,
                    entry_point: 0,
                    functions: Vec::new(),
                })
            }
            _ => Err(anyhow!("Unsupported binary format")),
        }
    }
}
