use clap::{Arg, ArgAction, CommandFactory, FromArgMatches, Parser, Subcommand};
use anyhow::{bail, Result};
use std::net::TcpStream;
use std::io::{self, BufRead, BufReader, Read, Write};

#[derive(Parser)]
#[command(name = "mudb")]
#[command(about = "A CLI for muDB", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Open an interactive session with muDB (supports MULTI/EXEC transactions)
    Open {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value = "6380")]
        port: u16,
    },
    /// Send a PING command
    Ping {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value = "6380")]
        port: u16,
    },
    /// Set a key-value pair
    Set {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value = "6380")]
        port: u16,
        key: String,
        value: String,
    },
    /// Get a value by key
    Get {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value = "6380")]
        port: u16,
        key: String,
    },
    /// LPUSH to a list
    Lpush {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value = "6380")]
        port: u16,
        list: String,
        value: String,
    },
    /// LRANGE on a list
    Lrange {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value = "6380")]
        port: u16,
        list: String,
        start: i64,
        stop: i64,
    },
}

fn main() -> Result<()> {
    let cli = Cli::from_arg_matches(&cli_command().get_matches())?;
    match cli.command {
        Commands::Open { host, port } => {
            repl(&host, port)?;
        }
        Commands::Ping { host, port } => {
            let mut stream = TcpStream::connect((host, port))?;
            let ping_cmd = "*1\r\n$4\r\nPING\r\n";
            stream.write_all(ping_cmd.as_bytes())?;
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf)?;
            print_resp(&buf[..n]);
        }
        Commands::Set { host, port, key, value } => {
            let mut stream = TcpStream::connect((host, port))?;
            let cmd = format!("*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n${}\r\n{}\r\n", key.len(), key, value.len(), value);
            stream.write_all(cmd.as_bytes())?;
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf)?;
            print_resp(&buf[..n]);
        }
        Commands::Get { host, port, key } => {
            let mut stream = TcpStream::connect((host, port))?;
            let cmd = format!("*2\r\n$3\r\nGET\r\n${}\r\n{}\r\n", key.len(), key);
            stream.write_all(cmd.as_bytes())?;
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf)?;
            print_resp(&buf[..n]);
        }
        Commands::Lpush { host, port, list, value } => {
            let mut stream = TcpStream::connect((host, port))?;
            let cmd = format!("*3\r\n$5\r\nLPUSH\r\n${}\r\n{}\r\n${}\r\n{}\r\n", list.len(), list, value.len(), value);
            stream.write_all(cmd.as_bytes())?;
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf)?;
            print_resp(&buf[..n]);
        }
        Commands::Lrange { host, port, list, start, stop } => {
            let mut stream = TcpStream::connect((host, port))?;
            let cmd = format!("*4\r\n$6\r\nLRANGE\r\n${}\r\n{}\r\n${}\r\n{}\r\n${}\r\n{}\r\n", list.len(), list, start.to_string().len(), start, stop.to_string().len(), stop);
            stream.write_all(cmd.as_bytes())?;
            let mut buf = [0; 2048];
            let n = stream.read(&mut buf)?;
            print_resp(&buf[..n]);
        }
    }
    Ok(())
}

/// Builds the CLI definition. `-h` is reserved for `--host` in every subcommand
/// (as in redis-cli), so their help flag is replaced with a long-only `--help`.
fn cli_command() -> clap::Command {
    let mut cmd = Cli::command();
    let names: Vec<String> = cmd
        .get_subcommands()
        .map(|sub| sub.get_name().to_string())
        .collect();
    for name in names {
        cmd = cmd.mut_subcommand(name, |sub| {
            sub.disable_help_flag(true).arg(
                Arg::new("help")
                    .long("help")
                    .action(ArgAction::Help)
                    .help("Print help"),
            )
        });
    }
    cmd
}

fn print_resp(resp: &[u8]) {
    let s = String::from_utf8_lossy(resp);
    let mut lines = s.split("\r\n").filter(|l| !l.is_empty());
    if let Some(first) = lines.next() {
        match first.chars().next() {
            Some('+') => println!("{}", &first[1..]), // Simple string
            Some('-') => eprintln!("Error: {}", &first[1..]), // Error
            Some(':') => println!("(integer) {}", &first[1..]), // Integer
            Some('$') => {
                // Bulk string
                if let Some(val) = lines.next() {
                    println!("{}", val);
                } else {
                    println!("(nil)");
                }
            }
            Some('*') => {
                // Array
                let count: usize = first[1..].parse().unwrap_or(0);
                for _ in 0..count {
                    if let Some(len_line) = lines.next() {
                        if len_line.starts_with('$') {
                            if let Some(val_line) = lines.next() {
                                println!("- {}", val_line);
                            } else {
                                println!("- (nil)");
                            }
                        }
                    }
                }
            }
            _ => println!("{}", s),
        }
    } else {
        println!("(empty response)");
    }
}

/// A RESP value read from the server.
enum Resp {
    Simple(String),
    Error(String),
    Integer(i64),
    Bulk(Option<String>),
    Array(Option<Vec<Resp>>),
}

/// Runs an interactive session over a single connection, so that state tied to
/// the connection (such as a MULTI transaction) persists between commands.
fn repl(host: &str, port: u16) -> Result<()> {
    let stream = TcpStream::connect((host, port))?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;
    println!("Connected to muDB at {}:{}. Type 'quit' to exit.", host, port);

    let stdin = io::stdin();
    let mut in_tx = false;
    loop {
        print!("{}:{}{}> ", host, port, if in_tx { "(TX)" } else { "" });
        io::stdout().flush()?;

        let mut input = String::new();
        if stdin.lock().read_line(&mut input)? == 0 {
            // EOF (Ctrl-D)
            println!();
            break;
        }

        let args = match parse_args(input.trim()) {
            Ok(args) => args,
            Err(e) => {
                eprintln!("(error) {}", e);
                continue;
            }
        };
        if args.is_empty() {
            continue;
        }

        let name = args[0].to_lowercase();
        if name == "quit" || name == "exit" {
            break;
        }

        writer.write_all(&encode_command(&args))?;
        let resp = read_resp(&mut reader)?;
        println!("{}", format_resp(&resp));

        // Mirror the server's transaction state for the prompt.
        in_tx = match (name.as_str(), &resp) {
            ("multi", _) => true,
            ("exec", _) | ("discard", _) => false,
            // The server discards an active transaction when a command fails to parse.
            (_, Resp::Error(_)) => false,
            _ => in_tx,
        };
    }
    Ok(())
}

/// Splits an input line into arguments on whitespace, treating double-quoted
/// sections as a single argument.
fn parse_args(line: &str) -> Result<Vec<String>> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut has_token = false;
    let mut in_quotes = false;

    for c in line.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                has_token = true;
            }
            c if c.is_whitespace() && !in_quotes => {
                if has_token {
                    args.push(std::mem::take(&mut current));
                    has_token = false;
                }
            }
            c => {
                current.push(c);
                has_token = true;
            }
        }
    }
    if in_quotes {
        bail!("unbalanced quotes");
    }
    if has_token {
        args.push(current);
    }
    Ok(args)
}

/// Encodes arguments as a RESP array of bulk strings.
fn encode_command(args: &[String]) -> Vec<u8> {
    let mut cmd = format!("*{}\r\n", args.len());
    for arg in args {
        cmd.push_str(&format!("${}\r\n{}\r\n", arg.len(), arg));
    }
    cmd.into_bytes()
}

/// Reads one complete RESP value from the connection.
fn read_resp<R: BufRead>(reader: &mut R) -> Result<Resp> {
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        bail!("connection closed by server");
    }
    let line = line.trim_end_matches("\r\n");
    if line.is_empty() {
        bail!("empty response from server");
    }
    let (prefix, rest) = line.split_at(1);

    let resp = match prefix {
        "+" => Resp::Simple(rest.to_string()),
        "-" => Resp::Error(rest.to_string()),
        ":" => Resp::Integer(rest.parse()?),
        "$" => {
            let len: i64 = rest.parse()?;
            if len < 0 {
                Resp::Bulk(None)
            } else {
                // value followed by a trailing CRLF
                let mut buf = vec![0; len as usize + 2];
                reader.read_exact(&mut buf)?;
                buf.truncate(len as usize);
                Resp::Bulk(Some(String::from_utf8_lossy(&buf).into_owned()))
            }
        }
        "*" => {
            let count: i64 = rest.parse()?;
            if count < 0 {
                Resp::Array(None)
            } else {
                let mut items = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    items.push(read_resp(reader)?);
                }
                Resp::Array(Some(items))
            }
        }
        _ => bail!("unexpected response from server: {}", line),
    };
    Ok(resp)
}

/// Formats a RESP value for display, numbering array items like redis-cli.
fn format_resp(resp: &Resp) -> String {
    match resp {
        Resp::Simple(s) => s.clone(),
        Resp::Error(e) => format!("(error) {}", e),
        Resp::Integer(n) => format!("(integer) {}", n),
        Resp::Bulk(Some(s)) => format!("\"{}\"", s),
        Resp::Bulk(None) | Resp::Array(None) => "(nil)".to_string(),
        Resp::Array(Some(items)) if items.is_empty() => "(empty array)".to_string(),
        Resp::Array(Some(items)) => items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let prefix = format!("{}) ", i + 1);
                let padding = " ".repeat(prefix.len());
                let body = format_resp(item).replace('\n', &format!("\n{}", padding));
                format!("{}{}", prefix, body)
            })
            .collect::<Vec<_>>()
            .join("\n"),
    }
}
