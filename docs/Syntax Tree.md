# Abstract Syntax Tree

In [scanning](./Scanning.md) we looked at how to convert the raw source code into lexemes. For 

Input : `1 + 2`

Lexer Output:

```rust
[
    INumber(1),
    Plus,
    INumber(2),
    Eof,
]
```

Above we identify what each token is but we don't know how they are related. This is where creating a representation that provides a relationship between tokens is helpful. Abstract syntax tree (AST) are the data structure that transforms the tokens into a structure expressing their relationship.

```txt
       Binary(+)
       /       \
      /         \
 Literal(1)   Literal(2)
```

