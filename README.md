# TAPL Implementation

Implements (parts) of the book "Types and Programming Languages" in Rust. Both a
cli and web interface.

## Languages

The following languages are implemented

- `untyped_arithmetic`, Untyped Arithmetic Expressions, chapters 3 and 4
- `untyped_lambda`, Untyped Lambda Calculus, chapters 5 and 7
- `typed_arithmetic`, Type arithmetic expressions, chapter 8
- `stlc`, Simply-Typed Lambda Calculus, chapters 9-11 (and some of chapter 19)
- `references`, STLC with References, chapter 13
- `exceptions`, STLC with exceptions, chapter 14
- `subtypes`, STLC with subtyping, chapters 15-18
- `recursive`, Recursive Types, chapter 20
- `system_f`, implements system F, chapters 23,25
- `existential`,Existential Types, chapter 24
- `bounded_quantification`, kernel F<:, chapter 26-28
- `lambda_omega`, Higher order STLC, chapter 29
- `f_omega` system f omega, chapter 30
- `f_omega_sub`,system f omega with subtypes, chapters 31,32

## Features

For each language, the following is implemented

- `Parse`, parses a program according to the shared grammar (found in
  `lib/parser/src/grammar.pest`)
- `Evaluate`, evaluates the main function of the program (currently only CBV)
- `Check`, typechecks each definition in the program (where applicable)
- `Grammar`, shows the grammar (terms, types, values) of the language

All used types can be formatted as Latex using either `bussproofs` or
`frac`+`arrays`

## CLI

The binary `tapl_cli` implements a command line interface to easily run/check
programs

```
tapl_cli [OPTIONS] <LANG> <CMD> [OUT_METHOD]
```

Both the `LANG` and `CMD` options are required, choosing which language to use
and which of the above commands to run. Additionaly an input needs to be
provided using either `-f` with a file path or `-i` with a literal string to
use.

By default, output is printed to stdout, but using the `-o` flag, an output file
path can be provided instead.

The last argument, `OUT_METHOD` describes how to print the results. If not
specified, the default `Display` method is used, to simply show the values as
strings. The available options are

- `simple` (same as none), uses `Display`
- `debug`, uses `Debug` formatting instead
- `latexbuss`, formats using bussproofs syntax for proof trees, producing a
  compilable latex document
- `bussstripped`, also uses bussproofs, but removes document formatting, only
  including the content
- `frac` - Uses latex fractions+arrays to format trees instead, produces an
  entire document
- `fracstripped` - Uses latex fractions+arrays, but only including content (also
  used for the web interface)

## Web Interface

Everything can also be compiled to web assembly using `make web`. This requires
having installed `wasm-pack` (`cargo install wasm-pack`). wasm binaries are then
saved in the `html` folder, along with generated html and css. Because of
cross-site policies, most browsers will not allow simply opening the html files
in a browser directly. Instead, they need to be statically served (for example
`python -m http.server`). Alternatively, the web interface is also pushed to
[Github Pages](https://ps-tuebingen.github.io/TAPL/).
