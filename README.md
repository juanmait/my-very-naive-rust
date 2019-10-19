I'm trying to learn some [Rust](https://www.rust-lang.org/) from scratch so this
is a **very naive** collection of snippets and notes about it.

## Create a new (bin) Project

Add `myproject` to `Cargo.toml` members. Then run:

```
cargo new myproject --bin
```

Be sure to include `Cargo.lock` to the repo when is a `bin` project. See:
https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries

## Run the bin

```
cd myproject
cargo run
```

## Update rustup

```
rustup self update
```

## Update Rust

```
rustup update
```

## Set toolchain

```
rustup default nightly
```

## Build the project

```
cargo build
```
