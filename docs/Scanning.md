# Scanning

Scanning takes input raw source code and converts them into tokens. Tokens are language specific characters. These make up the grammar for the language.

Lexemes is first step into process to converting tokens. These are smallest sequence representation of the language. For example,

```
var god_language = "rust";
```

Here, smallest entities from the raw source code are `var`, `god_language`, `=`, `"rust"` and `;`.

| Source | Token type | Lexeme | Literal value |
|---|---|---|---|
| `var` | `VAR` | `var` | — |
| `language` | `IDENTIFIER` | `language` | — |
| `=` | `EQUAL` | `=` | — |
| `"lox"` | `STRING` | `"lox"` | `lox` |
| `;` | `SEMICOLON` | `;` | — |

The lexmes can be used to identify the 
* token type: keywords specific to particular language
* literal values: the values for integer and string. The interpreter uses this information to create appropriate object in runtime.
* location information: where the particular lexeme starts and end

All these information bundled together: lexeme + information (token type, literal value, location) = token