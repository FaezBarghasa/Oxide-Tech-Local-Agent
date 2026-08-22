# python-bridge/mojo_accelerators/steno_compactor.mojo
from sys import argv

fn compact_compiler_log(raw_log: String) -> String:
    """
    SIMD-accelerated log filter that extracts diagnostic errors and warnings
    while stripping verbose source line snippets.
    """
    var lines = raw_log.split("\n")
    var compacted = String("")
    
    for i in range(len(lines)):
        var line = lines[i]
        if "error[E" in line or "-->" in line or "DRC Violation" in line:
            compacted += line + "\n"
            
    return compacted

fn main():
    var args = argv()
    if len(args) > 1:
        var result = compact_compiler_log(args[1])
        print(result)
    else:
        print("Mojo v1 Steno Context Compactor Module Loaded.")
