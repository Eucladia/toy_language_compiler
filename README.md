# Compiler for Toy Language

## Specification
The compiler for this toy language adheres to the following [specification].

## Installation
Rust is required for this and the binaries aren't published, so you'll have to build from the source. The easiest way do this is to install rust, [via rustup](https://www.rust-lang.org/tools/install).

You can then build the binaries from source with `cargo build --release`. The binary will be available in `target/release/`.

## Running
You can run this via `cargo build --release && target/release/toy_language/ <file>` or simply `cargo run --release <file>`.

<h2 align=center> Design Choices </h2>

### Lexer
The lexer's job is to produce a sequence of tokens from an input source – it simply gives a bit more meaning to arbitrary bytes. The structure of a token is:

```rust
struct Token {
  kind: TokenKind,
  // The span of the token.
  range: std::ops::Range<usize>,
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
  Slash,
  Minus,
  Plus,
  Semicolon,
  EndOfFile,
  Whitespace,
  Unknown,
}
```
As you can see, I chose to ***not*** include the actual lexeme of the token, but rather the range. This is a more efficient design, that results in less heap allocations.

I want the lexer's role to be minimal, which is also why I **don't** parse numbers here. The job of resolving and parsing various things will be done in the parser when we're making the AST.

### Parser
TODO: Fill this out when here

### Syntax Checker
TODO: Fill this out when here

### Evaluator (Interpreter)
Since the language is really simple and it only consists of basic mathemetical operations on integers, we can just convert the infix expression to a postfix one. We can then evaluate that easily since postfix expressions remove ambiguity and the need for parenthesis!


[specification]: SPECIFICATION.md
