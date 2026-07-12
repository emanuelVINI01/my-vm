# My VM & Compiler - v1.0

This project implements a complete, scratch-built **Virtual Machine** with a register-based architecture and a **Python Compiler** (`vm-compiler`). It is designed to translate a strict subset of Python code into a custom bytecode/assembly language, which is then executed by the high-performance Virtual Machine engine.

## Overview

This repository features two main components:
1. **The Python Compiler**: A transpiler written in Python that parses Python source code using the native `ast` (Abstract Syntax Tree) module, compiles it, and generates custom assembly code (`.asm`). It includes an integrated linker to resolve multiple local modules.
2. **The Virtual Machine**: A register-based runtime environment written in Rust. It reads the `.asm` output from the compiler, loads it into its simulated RAM (1024 words), and executes the instruction set.

## Project Structure

- `/src`: The core Virtual Machine engine, written in Rust for safety and performance.
- `/vm-compiler`: The compiler (transpiler) written in Python, responsible for AST parsing and assembly generation.
- `/tests`: Examples of testable and validatable code showcasing the compiler's capabilities.
- `ISA.md`: The official Instruction Set Architecture (ISA) documentation detailing the supported assembly instructions.
- `TODO.md`: Technical debt, known issues, and roadmap for version 2.

## How to Compile and Run

The workflow consists of a two-step process: compiling your Python source code into an assembly file, and then running that assembly file through the Rust Virtual Machine.

### Prerequisites
- **Python 3.x** (for the compiler)
- **Rust / Cargo** (for the VM)

### Workflow

```bash
# Step 1: Compile from Python to Assembly
python3 vm-compiler/main.py tests/test_loops.py target.asm

# Step 2: Execute the compiled Assembly on the VM
cargo run -- target.asm
```

## Writing Programs

When writing programs for this VM, your syntax must follow a clean, strict subset of Python. 

### Supported Features
- Control flow: `if`, `else`, `while`, `for i in range()`
- Pure arithmetic operations
- Multi-file module importing (resolved locally by the Linker)

### Restrictions
- **No Standard Library**: You do not have access to the Python Standard Library (e.g., `import math`, `import sys`). Everything must be written manually or imported from local modules.
- **Complex Types**: Lists, Classes, and Dictionaries are not natively supported by the compiler in v1.0.

### Program Example (Multiple Modules)

You can split your logic across multiple files. The linker will automatically resolve local dependencies.

**`math_lib.py`**
```python
def squared(x):
    return x * x
```

**`main.py`**
```python
from myvm_lib import entry_point, print_str
import math_lib

@entry_point
def main():
    result = math_lib.squared(5)
    print_str("The square is:")
    print_str(str(result))
```
*Note: The `@entry_point` decorator specifies the starting function of your program.*

## Current Limitations (v1.0)
- **Registers**: Limited to 26 general-purpose registers, named `A` through `Z`.
- **Data Types**: No native compiler support for complex data types like Lists, Classes, or Dictionaries. Only primitive types (integers/strings) are currently handled.
- **Error Handling**: Missing modules are treated silently by the Linker.
- **Memory**: The total simulated RAM size is restricted to 1024 words.

For an in-depth look at the architecture and supported instructions, please consult the `ISA.md` document.
