# learn-rust
Notes, demo projects related to Rust learning

## cargo commands

### Init project

```bash
cargo new <project-name>
```

### Run project

```bash
cargo run
```

### Build project

```bash
cargo build
cargo build --quiet
cargo build --release
```

### Add dependency
#### cargo-edit

```bash
cargo install cargo-edit
cargo add anyhow colored jsonxf mime
```

#### Edit `Cargo.toml`

```toml
[dependencies]
anyhow = "1"
...
```