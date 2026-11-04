# Smart Pointers

The core primitive that allows the implementation of shareable mutable containers is UnsafeCell which is itself unsafe code.

## Cell

Cell does not give a reference to the inner value that it is holding and it does not let you pass the Cell type across threads, because two threads could change the value at the same time. Knowing these 2 facts, helps Cell to know that the value only gets updated through its methods calls, therefore mutating it is fine.

Cell should be used only for small copy values like numbers of flags. The classic example would be thread-local states.
