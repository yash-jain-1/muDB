# Binaries Overview

MuDB provides two binaries:

- **mudb**: The server. Starts the in-memory cache database and listens for RESP commands.
- **mudb-cli**: The client. Allows you to interact with the server using commands like `ping`, `set`, `get`, etc.

## Example Usage

### Start the Server

```bash
mudb --port 6380
```

### Use the CLI Client

```bash
mudb-cli ping --host 127.0.0.1 --port 6380
mudb-cli set --host 127.0.0.1 --port 6380 mykey myvalue
mudb-cli get --host 127.0.0.1 --port 6380 mykey
mudb-cli lpush --host 127.0.0.1 --port 6380 mylist item1
mudb-cli lrange --host 127.0.0.1 --port 6380 mylist 0 -1
```

## Troubleshooting

- **Connection refused**: Make sure the server is running (`mudb --port 6380`) before using the CLI client.
- **Command not found**: Ensure your Cargo bin directory (usually `~/.cargo/bin`) is in your PATH.
	You can add it with:
	```bash
	export PATH="$HOME/.cargo/bin:$PATH"
	```
- **Port conflicts**: If another process is using the port, specify a different port for both server and client.
- **Binary confusion**: Use `mudb` for the server and `mudb-cli` for client commands. Do not run CLI commands with the server binary.
### Installing the CLI Client

After publishing, users can install the CLI client globally with:

```bash
cargo install mudb-cli
```

This will make the `mudb-cli` command available anywhere on your system.

Example usage:

```bash
mudb-cli ping --host 127.0.0.1 --port 6380
```
### Using the `mudb` Command Anywhere

After publishing, you can install MuDB globally with:

```bash
cargo install mudb
```

For local development, you can install the binary from your project directory:

```bash
cargo install --path .
```

This will place the `mudb` binary in your Cargo bin directory (usually `~/.cargo/bin`).
Make sure this directory is in your system's PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Now you can run `mudb` from anywhere:

```bash
mudb --port 6380
```
# MuDB

A RESP-based in-memory cache server written in Rust, inspired by Redis. Includes a CLI client for easy interaction.

## Features
- RESP protocol support
- In-memory key-value and list storage
- CLI client for user-friendly commands
- Colorful ASCII bull banner on server startup

## Getting Started

### Prerequisites
- Rust (https://rustup.rs)
- Cargo
- (Optional) Docker

### Running the Server

```bash
# Clone the repository
$ git clone https://github.com/yash-jain-1/muDB.git
$ cd muDB

# Build and run the server
$ cargo run -- --port 6380
```

You should see a colorful MuDB bull banner and server logs.

### Installation

You can install MuDB directly from crates.io (after publishing):

```bash
cargo install mudb
```

Or build from source:

```bash
# Clone the repository
$ git clone https://github.com/yash-jain-1/muDB.git
$ cd muDB

# Build and run the server
$ cargo run -- --port 6380
```

You should see a colorful MuDB bull banner and server logs.

### Using the CLI Client

```bash
# Build the CLI tool
$ cd cli
$ cargo build --release

# Example commands
$ ./target/release/mudb-cli ping --host 127.0.0.1 --port 6380
$ ./target/release/mudb-cli set mykey myvalue --host 127.0.0.1 --port 6380
$ ./target/release/mudb-cli get mykey --host 127.0.0.1 --port 6380
$ ./target/release/mudb-cli lpush mylist item1 --host 127.0.0.1 --port 6380
$ ./target/release/mudb-cli lrange mylist 0 --host 127.0.0.1 --port 6380 -- -1
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

### Code Style
- Use Rust 2021 edition
- Format code with `cargo fmt`
- Run `cargo clippy` for lint checks

### Running Tests
```bash
$ cargo test
```

## Docker

A sample Dockerfile is provided. To build and run with Docker:

```bash
$ docker build -t mudb .
$ docker run -p 6380:6380 mudb
```

## License

MIT

---

Enjoy hacking on MuDB! 🐂
