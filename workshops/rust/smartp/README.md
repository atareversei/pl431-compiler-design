# Smart Pointers

The core primitive that allows the implementation of shareable mutable containers is UnsafeCell which is itself unsafe code.

## Cell

Cell does not give a reference to the inner value that it is holding and it does not let you pass the Cell type across threads, because two threads could change the value at the same time. Knowing these 2 facts, helps Cell to know that the value only gets updated through its methods calls, therefore mutating it is fine.

Cell should be used only for small copy values like numbers of flags. The classic example would be thread-local states.

Why don't we set the restriction of `T: Copy` on the type itself? Because by convention in Rust we only put the restriction on the place where it is needed only (putting it on the type, requires the type holding a Cell to also be Copy).

```rust
struct Cell<T: Copy> {} // This is not used too often

impl<T> for Cell<T> {
    pub fn get where T: Copy {} // This is used more often
}
```

## RefCell

RefCell dynamically checks the borrow rules in runtime.

## Rc
Rc (Reference counter pointer) is a pointer to some data that has been allocated on the heap and it does not let mutability. When all references are gone, the inner value is dropped as well. This makes it ideal for large strings and blobs inside program which require to be accessed by multiple data structures.

## Thread-Safe Variants

- RwLock for RefCell
- Arc for Rc
