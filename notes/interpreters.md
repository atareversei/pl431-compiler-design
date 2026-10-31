# Interpreters

## Compilers and Interpreters

What is the difference between interpreters and compilers?

- Interpreters implement the semantics themselves
- Compilers delegate semantics to some other interpreter (often times this interpreter is the CPU itself executing instructions)

There are 2 types of interpreters:

1. AST-based interpreters (recursive)
2. Bytecode-interpreters (VMs)

and 3 types of compilers:

1. AOT (Ahead of time) compilers like C++
2. JIT (Just in time) compilers
3. AST-transformers like TypeScript transpiler

When we talk about interpreters and compilers, we are refering to the implementation not the language.

### AST Based Interpreters

The AST is directly piped to the runtime module and the runtime traverses the tree to execute the code and produce the result.

### Bytecode Interpreters

Bytecodes are more efficient in execution due to more granular control on the instructions and more machine friendly format (opposed to trees and following pointers)

There are 2 types:

- Stack-based VMs: The result is always at the top of the stack, e.g. Java, JavaScript, Python
- Register-based VMs: Set of virtual registers and the result is stored in accumulator register

Most machines (physical and virtual) are mixed machines. They use stacks but they still have stack pointer registers and so on.

### AOT Compilers

The AST is passed to a code generator module and it might produce multiple Intermediate Representation of that code which is finally used to generate instructions for a specific machine like x86/x64.

### JIT Compilers

The runtime detects hot code paths and generates native code for those paths and branches.

### AST Transformer Compiler (Transpiler)

The AST gets fed to AST transformer module which creates a new AST and the code generator turns it into another language's syntax. It is purely frontend.

## Implementation

### Environments

We have 3 operations that interact with environments:

- Variable Declaration (`let x = 10`)
- Variable Assignment (`x = 15`)
- Variable Lookup (`x`)

We need some machinery on the environment API:

- resolve -> gets the environment
- lookup -> calls resolve and looks up the variable
