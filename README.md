# multiboot2-rs
Multiboot2 boot information parser for Rust.

There are various parsers already available. This, however, serves as an excercise for myself in iterators and binary parsing, and to be a more readable (and perhaps more complete) version of those that already exist.

This library can be used by specifying the github url in the dependencies section of a parent project's Cargo.toml:
```
[dependencies]
multiboot2 = { git = "https://github.com/Adam-Gleave/multiboot2-rs" }
```
(Crate has not yet been submitted to crates.io)
