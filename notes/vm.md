# VM

## Implementation

The main function which executes the Bytecode is an infinite loop as opposed to a recursively called evaluate function in AST interpreters.

For values, we use the constant table. Take the `push` instruction in mind for example. The instruction stream might look like this: `[..., OP_CONST, 0, ...]` which means the OP_CONST command should be applied on the index 0 of the constant pool.

## Stack-Based Vs. Register-Based VMs

I've explained it in [interpreters.md](./interpreters.md)
