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
