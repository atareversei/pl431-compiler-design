# Declarative Macros

They accept correct rust syntax and output correct rust statements.

```rust
macro_rules! something {
    ($arg1:ty => $arg2:expr; $arg3:path) {};
}

something {u32=>x.foo(); std::path};
``` 

Macro variables don't get mixed with the variables outside of it automatically, so they have different scopes.

The reason for weird `{{...}}` syntax is that rust allows an expression inside the macro body, so if our macro logic is multiple lines of statements we have to wrap them inside a block.

An example:
```rust
macro_rules! max_impl {
    ($t: ty) => {
        imple $crate::MaxValue for $t {
            fn max_value() -> Self {
                $t::MAX
            }
        }
    }
}

max_impl!(u32);
max_impl!(u64);
```

We have to keep in mind that a macro is just really a text substitution mechanism and if we do things like:

```rust
avec!(x.take().unwrap(); 4);
```

It might fail because it will perform the `take()` function twice, so we might want to clone the value.
