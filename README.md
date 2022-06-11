# learn-rust
Notes and demo code related to Rust learning

## cargo commands

### Init project

```bash
cargo new <project-name>
```

### Add dependency

#### Edit `Cargo.toml`

```toml
[dependencies]
anyhow = "1"
```

#### cargo-edit

```bash
cargo install cargo-edit
cargo add anyhow colored jsonxf mime
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

### Test project

```bash
cargo test
```