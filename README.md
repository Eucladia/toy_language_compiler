# Compiler for Toy Language

## Installation
Rust is required for this and the binaries aren't published, so you'll have to build from the source. The easiest way do this is to install rust, [via rustup](https://www.rust-lang.org/tools/install).

## Specification
The compiler for this toy language adheres to the following specification:

```
Program:
	Assignment*

Assignment:
	Identifier = Exp;

Exp:
	Exp + Term | Exp - Term | Term

Term:
	Term * Fact  | Fact

Fact:
	( Exp ) | - Fact | + Fact | Literal | Identifier

Identifier:
     	Letter [Letter | Digit]*

Letter:
	a|...|z|A|...|Z|_

Literal:
	0 | NonZeroDigit Digit*

NonZeroDigit:
	1|...|9

Digit:
	0|1|...|9
````
