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
    }
    Ok(())
}
