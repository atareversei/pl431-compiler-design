# Lifetime

- Source: following along with Jon Gjengset's Crust of Rust Playlist on YouTube

## Notes

Compiler needs to know for how long do certain pointers live. In our case the `remainder` is passed from the outside of `StrSplit` and the compiler wants to know does `remainder` live longer than the struct or not.

We can't be wrong about lifetimes, lifetimes are like types, eventually compiler will point out any inconsistencies in our code and we can fix it.

Anonymous lifetimes are like type inferences. They can be used where ever only one lifetime can be inferred:

```rust
// compiler knows that the lifetime is tied to lifetime of the struct itself (`&self`)
impl Foo {
    do_sth(&self) -> &'_ str {/*do something*/}
}
```

---

Example: The compiler will complain that lifetime of haystack is unknown and it won't compile. The caller might just remove the `haystack` or `delimiter` right after creating the `StrSplit` struct which should not be ok and because of that the compiler throws an error.

```rust
impl StrSplit<'_> {
    pub fn new(haystack: &str, delimiter: &str) -> Self {
```

Compare the code above with the fixed version below. In the code snippet below, we can read it as: "I can give you a struct of lifetime 'a if you give me input of at least lifetime 'a":

```rust
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
```

---

The literal string has the lifetime of `'static` which lives for the lifetime of the whole program. The values live in the program's binary.

There was this syntax, that looked weird:

```rust
if let Some(ref mut remainder) /* &mut &'a str */ = self.remainder /* Option<&'a str> */ {
```

Someone asked what if it was like this?

```rust
if let Some(&mut remainder) = self.something {
```

It means the type of `self.something` was `Option<&mut Type>` and we were matching against that.

Differences between string related types:

- str -> [char]: points to a sequence of character and doesn't know its length
- &std-> [char]: points to a sequence of character and knows the length
- String -> Vec<char>: always heap allocated and it is dynamic which means it can grow and shrink

String to &str is easy but &str to String requires allocation and copying.

---

Multiple lifetimes are rare. In this code snippet, the delimiter has the scope lifetime and lives a shorter lifetime than the haystack, so the whole StrSplit gets reduced to scope lifetime and when you return a reference value, the Rust can only assume that the returned reference has the scope lifetime and cannot live out of the function and that is why it complains:

```rust
fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit always returns a value")
}
```
