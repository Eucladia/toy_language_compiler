<h1 align = "center"> Compiler for Toy Language </h1>

## Specification
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;The compiler for this toy language adheres to the following [specification].

## Installation
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Rust is required for this and the binaries aren't published, so you'll have to build from the source. The easiest way do this is to install rust, [via rustup](https://www.rust-lang.org/tools/install).

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;You can then build the binaries from source with `cargo build --release`. The binary will be available in `target/release/`.

## Running
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;You can run this via `cargo build --release && target/release/toy_language/ <file>` or simply `cargo run --release <file>`.

Sample files and output are available in `sample_files/`

<h2 align=center> Design Choices </h2>

### Lexer
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;The lexer's job is to produce a sequence of tokens from an input source – it simply gives a bit more meaning to arbitrary bytes. The structure of a token is:

```rust
struct Token {
  kind: TokenKind,
  // The span of the token.
  range: std::ops::Range<usize>,
  // The line that the token is on.
  line: usize
}
```
where `TokenKind` is
```rust
enum TokenKind {
  Literal,
  Identifier,
  Equal,
  LeftParen,
  RightParen,
  Star,
  Minus,
  Plus,
  Semicolon,
  EndOfFile,
  Whitespace,
  Unknown,
}
```
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;As you can see, I chose to ***not*** include the actual lexeme of the token, but rather the range of the token from the source file. This is a more efficient design, that results in less heap allocations in the long run. In order to have good error diagnostics, I also added the line in which the token is on, so I don't have to include a linear function to determine the line that a token's on.

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;I want the lexer's role to be minimal, which is also why I **don't** parse numbers here. The job of resolving and parsing various things will be done later in the pipeline.

### Parser
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;The parser uses a top-down recursive descent approach and has error recovery as well as good error diagnostics.

<h4 align = "center"> Parser Error Recovery </h4>
The parser tries to recover from errors as much as possible. Such errors may include:

1) A lack of semicolons after an assignment.
2) A lack of variable names before an eq sign.
3) A lack of eq sign in assignments.

In these cases, the compiler will generate a neat error message on the line and column in which this error occurred on. For example, running the parser on the following code that's in `sample_input/err_expr.txt`:
```js
bbb = 6;
aaa =
;
ccc = ;
foo = 6
```

generates the following errors:
```
1) sample_input/err_expr.txt:3:1
      expected either `+`, `-`, `(`, an `Identifier`, or a `Literal`, but found `;` (Semicolon)

2) sample_input/err_expr.txt:4:7
      expected either `+`, `-`, `(`, an `Identifier`, or a `Literal`, but found `;` (Semicolon)

3) sample_input/err_expr.txt:5:8
      expected a `Semicolon` after `6`, but found `` (EndOfFile).
```

<h4 align = "center"> Error Recovery Limitations! </h4>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;One case where error recovery is much harder, due to ambiguity, is the following:

```js
a =
b = 2;
```

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Since whitespace is irrelevant in this language, that code can be seen as `a = b = 2;`. The compiler errors, as it should since the grammar doesn't allow this, but the error diagnostic says that we're missing a semicolon after the `b`, eg `a = b; = 2;`. If someone wrote this code, it could be one of 2 potential mistakes:

1) They're assigning `b` to `2` and forgot to initialize `a`:
```js
a =
b = 2;
```

2) They're indeed assigning `a` to `b`, but forgot a variable name to assign to `2`.
```js
a = b;
 = 2;
```

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Without backtracking, we can't quite figure out how we should interpret this. The parser, since it's top-down without any backtracking, assumes that the programmer meant the latter.

### Evaluator (Interpreter)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;The interpreter recursively traverses the tree, evaluating the node's values with the result of its child nodes. We keep track of variables by storing them in a `HashMap` that maps an identifier to its current value.


[specification]: SPECIFICATION.md
