# TAPL Implementations

Implements (parts) of the book "Types and Programming Languages" in Rust.
All languages can be found in the `lang` directory, with the following implemented:

## I Untyped Systems 

* `untyped_arithmetic`: Untyped Arithmetic Expressions (chapter 3 and 4)
* `untyped_lambda` : Untyped Lambda Calculus (chapters 5 and 7)
* `nameless_representation`: Nameless Representation of Untyped Lambda Calculus (chapter 6)

## II Simple Types 

* `typed-arithmetic`: Typed Arithmetic expressions (chapter 8), TODO
* `stlc`: Simply-Typed Lambda Calculus (chapters 9-11 (and some of chapter 19))
* `references`: STLC with References (chapter 13)
* `exception`: STLC with exceptions (chapter 14)

## III Subtyping 

* `subtypes`: STLC with subtyping (chapter 15-18)
* `featherweight`: Featherweight Java (chapter 19)

## IV Recursive Types 

* `recursive`: Recursive Types (chapter 20), TODO: parser+tests

## V Polymorphism

* `inference`: implements type inference (bidirectional and constraint-based) for STLC (chapter 22)
* `system_f`: implements system F (chapter 23,25)
* `existential`: Existential Types (chapter 24), TODO: parser+tests
* `bounded_quantification`: kernel F<: (chapter 26-28)

## VI Higher-Order Systems 

* `lambda_omega`: Higher order STLC (chapter 29)
* `f_omega`: system f omega (chapter 30), TODO:tests+parser
* `f_omega_sub`: system f omega with subtypes (chapter 31,32), tests+parser
