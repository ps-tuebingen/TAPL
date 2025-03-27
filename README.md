# TAPL Implementations

Implements (parts) of the book "Types and Programming Languages" in Rust.
All languages can be found in the `lang` directory, with the following implemented:

* `untyped_arithmetic`: Evaluation and parser for untyped arithmetic expressions (chapter 3 and 4)
* `untyped_lambda` : Evaluation and a simple parser for the untyped lambda calculus (chapters 5 and 7)
* `nameless_representation` : Nameless represntation of lambda terms (chapter 6)
* `stlc`: Type checking and evaluation, including evaluation using evaluation contexts, for the simply typed lambda calculus including extensions (chapters 9-11 (and some of chapter 19))
* `references`: Lambda calculus with units and references (chapter 13)
* `exception`: stlc with exceptions (chapter 14)
* `subtypes`: stlc with subtyping (chapter 15-18)
* `featherweight`: featherweight java (chapter 19)
* `recursive`: Recursive Types (chapter 20)
* `inference`: implements type inference (bidirectional and constraint-based) for STLC with extensions (chapter 22)
* `system_f`: implements system F (chapter 23,25)
* `existential`: Existential Types (chapter 24)
* `bounded_quantification`: kernel F<: (chapter 26-28)
* `lambda_omega`: Higher order stlc (chapter 29)
* `f_omega`: system f omega (chapter 30)
* `f_omega_sub`: system f omega with subtypes (chapter 31,32)
