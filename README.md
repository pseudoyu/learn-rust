# learn-rust
Notes and demo code related to Rust learning

## cargo usage

### Version

```bash
cargo version
```

### Init workspace

#### Create and edit `Cargo.toml`

```toml
[workspace]

members = [
    "basics",
    "value_demo",
    "persist_data",
    "scrape_url",
    "httpie"
]
```

### Init project

```bash
cargo new <project-name>
cargo new <lib-name> --lib
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