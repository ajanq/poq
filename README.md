# poq

POQ is a Rust CLI that scaffolds boilerplate for new projects across languages, their ecosystems (libraries and tools), and varied project types, with an extensible, template-based design.

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/poq`.

## Usage

Run interactively:

```bash
cargo run
```

Or with arguments:

```bash
cargo run -- python web my-web-app
cargo run -- python cli my-cli-app
cargo run -- python data_science my-ds-project
```

## Supported Languages

- Python (base, web, cli, data_science)

## License

MIT
