# Rust Advanced CLI

An advanced CLI tool demonstrating Rust best practices including async programming, error handling, configuration management, and structured logging.

## Features

- **Async HTTP Client**: Fetch data from remote APIs with retry logic
- **Local Storage**: Store and retrieve JSON data with metadata
- **Configuration Management**: YAML/JSON config with environment variable support
- **Structured Logging**: File and console logging with configurable levels
- **Error Handling**: Comprehensive error types with context
- **CLI Interface**: Rich command-line interface with subcommands

## Installation

```bash
cargo install --path .
```

## Usage

### Configuration

Generate a default configuration file:
```bash
rcli config init --output config.yaml
```

Show current configuration:
```bash
rcli config show
```

### Data Storage

Store JSON data:
```bash
rcli store user1 '{"name": "John", "age": 30}'
```

Store data from file:
```bash
rcli store config --file config.json
```

Retrieve stored data:
```bash
rcli get user1
rcli get user1 --format yaml
```

List all stored keys:
```bash
rcli list
rcli list --detailed
```

Delete stored data:
```bash
rcli delete user1
```

### HTTP Operations

Fetch data from API:
```bash
rcli fetch https://api.github.com/users/octocat
rcli fetch /users/octocat --format json --output user.json
```

## Configuration

The tool supports configuration via:
1. Configuration files (`config.yaml`, `config.json`)
2. Environment variables (prefixed with `RCLI_`)
3. Command-line arguments

### Environment Variables

- `RCLI_SERVER__BASE_URL`: Default API base URL
- `RCLI_SERVER__TIMEOUT_SECONDS`: HTTP timeout in seconds
- `RCLI_LOGGING__LEVEL`: Log level (trace, debug, info, warn, error)
- `RCLI_STORAGE__DATA_DIR`: Directory for stored data

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy
cargo fmt
```

## Architecture

- `src/main.rs`: Application entry point and command handling
- `src/cli.rs`: Command-line interface definitions
- `src/config.rs`: Configuration management
- `src/error.rs`: Error types and handling
- `src/http.rs`: HTTP client with retry logic
- `src/logging.rs`: Structured logging setup
- `src/storage/`: Local data storage implementation

## License

MIT License