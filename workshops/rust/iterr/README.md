# Iterators

In rust, `for` loops are syntactic sugar and they will get turned into iterators and `while Some()` loops.

```rust
let mut iter = vec![1, 2, 3, 4, 5].into_iter();
while let Some(item) = iter.next() {}
```

## Difference Between Associated Types And Generic Type Parameters

This is an associated type. Associated type gets selected once by the implementer and it cannot be defined for that type again.

```rust
trait Iterator {
    type Item;
    fn next(&mut Self) -> Self::Item;
}

impl Iterator for Hashmap {
    type Item = (Key, Value);
}

// This throws an error
impl Iterator for Hashmap {
    type Item = [Key, Value];
}
```

Generic types let multiple implementations exist for a type.

```rust
trait Handler<Request, Response> {
    fn serve(&mut self, Request) -> Response;
}

impl Handler<HTTPRequest, HTTPResponse> for HttpHandler {
    fn serve(&mut self, request: HTTPRequest) -> HTTPResponse {/*...*/}
}

// Maybe our handler can work on some other form of communication protocol
impl Handler<HTTP2Request, HTTP2Response> for HTTPHandler {
    fn serve(&mut self, request: HTTP2Request) -> HTTP2Response
}
```

## Types

There are some types in code that look similar:
- Iterator: Something that we can iterate over
- IntoIterator: Something that can become an iterator
- IntoIter: The associated type inside of IntoIterator trait that gets returned as the iterator

Iterator and IntoIterator are traits and IntoIter is a struct which is inside IntoIterator and implements Iterator.
