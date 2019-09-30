rust-sestina
============
Examples from https://github.com/chrisamaphone/sestina using Rust rather than the interpreter there. Rust's type system and rand library include most of the functionality of the language. Rusts's macros can hopefully provide an elegant interface in the future.

Rust
====
install `www.rust-lang.org`
type-check `cargo check`, `cargo check --example <filename-no-ext>`
run `cargo run`, `cargo run --example <filename-no-ext>`

main
====
`cargo run`
Early trial creating a macro parser for the language. The language is experimental and not fully developed, so this was never going to get too far.

examples
========
`cargo run --example <filename-no-ext>`
Recreations of sestina examples, each with a different strategy: run-time data, random distrubutions, enums and iterators.