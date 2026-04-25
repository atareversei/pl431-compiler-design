# RLox

## Grammar

```bnf
expression   → comma;
comma        → ternary ("," ternary)*;
ternary      → equality ("?" expression ":" ternary)?;
equality     → comparison (("!=" | "==") comparison )*;
comparison   → term ((">" | ">=" | "<" | "<=") term )*;
term         → factor (("-" | "+") factor )*;
factor       → unary (( "/" | "*") unary)*;
unary        → ("!" | "-") unary
             | binary_op_error
             | primary
             ;
primary      → NUMBER
             | STRING
             | "true"
             | "false"
             | "nil"
             | "(" expression ")"
             ;

binary_op_error → "!=" | "==" | ">" | ">=" | "<" | "<=" | "+" | "*" | "/";
```
