## Tree Command Line Utility in Rust
A simple command line tool in Rust to list the contents of directory similar to [tree command](https://en.wikipedia.org/wiki/Tree_(command)).

## Build Local
1. Install cargo and rust from official rust's [website](https://www.rust-lang.org/tools/install).

2. Cargo build
```console
cargo build --release
```

3. Execute the executable created
```console
.\target\release\tree "src"
```
Output
```console
|-main.rs
|-tree.rs
```

## Copyrights & License

Licensed Under [@MIT](./LICENSE)
