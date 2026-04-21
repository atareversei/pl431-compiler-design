# Basics

## Setting Up the Environment

Useful VS Code extensions:

- Code LLDB
- Even Better TOML
- Crates

VS Code Settings:

- `Allow breakpoints everywhere: on`
- `Cargo check: on + clippy (for larger projects we might turn it off)`

## Variables

Rust is kind of a hybrid between functional languages and imperative languages, so the syntax and patterns look a little bit strange. The mindset behind Rust's philosophy leads to writing lots and lots of functions, mostly small in size.

```rs
fn main() {
    let n = {
        5
    }; // a block is an expression that returns a value.

    let i = 10;
    let n = if i == 10 {
        6
    }  else {
        7
    }
}
```

## Reading from `stdin`

Like the reader I have defined in `rlox/lox.rs/run_repl()`:

```rs
fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input); // This could fail if there is no stdin capability.
    input
}
```
