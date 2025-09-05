use clap::{Parser, Subcommand};
use anyhow::Result;
use std::net::TcpStream;
use std::io::{Read, Write};

#[derive(Parser)]
#[command(name = "mudb")]
#[command(about = "A CLI for muDB", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Open a connection to muDB
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
    let cli = Cli::parse();
    match cli.command {
        Commands::Open { host, port } => {
            println!("Connecting to muDB at {}:{}...", host, port);
            let _stream = TcpStream::connect((host, port))?;
            println!("Connected!");
        }
        Commands::Ping { host, port } => {
            let mut stream = TcpStream::connect((host, port))?;
            let ping_cmd = "*1\r\n$4\r\nPING\r\n";
            stream.write_all(ping_cmd.as_bytes())?;
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf)?;
            let resp = String::from_utf8_lossy(&buf[..n]);
            println!("Response: {}", resp);
        }
        Commands::Set { host, port, key, value } => {
            let mut stream = TcpStream::connect((host, port))?;
            let cmd = format!("*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n${}\r\n{}\r\n", key.len(), key, value.len(), value);
            stream.write_all(cmd.as_bytes())?;
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf)?;
            let resp = String::from_utf8_lossy(&buf[..n]);
            println!("Response: {}", resp);
        }
        Commands::Get { host, port, key } => {
            let mut stream = TcpStream::connect((host, port))?;
            let cmd = format!("*2\r\n$3\r\nGET\r\n${}\r\n{}\r\n", key.len(), key);
            stream.write_all(cmd.as_bytes())?;
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf)?;
            let resp = String::from_utf8_lossy(&buf[..n]);
            println!("Response: {}", resp);
        }
        Commands::Lpush { host, port, list, value } => {
            let mut stream = TcpStream::connect((host, port))?;
            let cmd = format!("*3\r\n$5\r\nLPUSH\r\n${}\r\n{}\r\n${}\r\n{}\r\n", list.len(), list, value.len(), value);
            stream.write_all(cmd.as_bytes())?;
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf)?;
            let resp = String::from_utf8_lossy(&buf[..n]);
            println!("Response: {}", resp);
        }
        Commands::Lrange { host, port, list, start, stop } => {
            let mut stream = TcpStream::connect((host, port))?;
            let cmd = format!("*4\r\n$6\r\nLRANGE\r\n${}\r\n{}\r\n${}\r\n{}\r\n${}\r\n{}\r\n", list.len(), list, start.to_string().len(), start, stop.to_string().len(), stop);
            stream.write_all(cmd.as_bytes())?;
            let mut buf = [0; 2048];
            let n = stream.read(&mut buf)?;
            let resp = String::from_utf8_lossy(&buf[..n]);
            println!("Response: {}", resp);
        }
    }
    Ok(())
}
