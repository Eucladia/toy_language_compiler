# Specification

On top of following the below rules, the compiler should:

1) Detect syntax errors.
2) Report uninitialized variables.
3) Perform the assignments if there is no error.
4) Print out the values of all the variables after all the assignments are done.

## Rules
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
