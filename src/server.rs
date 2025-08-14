// anyhow provides the Error and Result types for convenient error handling
use anyhow::{Error, Result};

// log crate provides macros for logging at various levels (error, warn, info, debug, trace)
use log::error;

use tokio::{
    // AsyncWriteExt trait provides asynchronous write methods like write_all
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

/// The Server struct holds the tokio TcpListener which listens for
/// incoming TCP connections.
#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    /// Creates a new Server instance with the given TcpListener.
    pub fn new(listener: TcpListener) -> Server {
        Server { listener }
    }

    /// Runs the server in an infinite loop, continuously accepting and handling
    /// incoming connections.
    pub async fn run(&mut self) -> Result<()> {
        loop {
            
            let sock = match self.accept_conn().await {
                Ok(stream) => stream,
                Err(e) => {
                    error!("{}", e);
                    panic!("Error accepting connection");
                }
            };

            // Spawn a new asynchronous task to handle the connection.
            tokio::spawn(async move {
                use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
                let mut reader = BufReader::new(sock);
                let mut line = String::new();
                loop {
                    // Send prompt for input
                    if let Err(e) = reader.get_mut().write_all(b"> ").await {
                        error!("Error writing prompt to socket: {}", e);
                        break;
                    }
                    line.clear();
                    let bytes_read = match reader.read_line(&mut line).await {
                        Ok(0) => break, // Connection closed
                        Ok(n) => n,
                        Err(e) => {
                            error!("Error reading from socket: {}", e);
                            break;
                        }
                    };
                    // Remove trailing newline (if any) and echo the line with CRLF for proper cursor placement
                    let trimmed = line.trim_end_matches(['\r', '\n'].as_ref());
                    let response = format!("# {}\r\n", trimmed);
                    if let Err(e) = reader.get_mut().write_all(response.as_bytes()).await {
                        error!("Error writing to socket: {}", e);
                        break;
                    }
                }
            });
        }
    }

    /// Accepts a new incoming TCP connection and returns the corresponding
    /// tokio TcpStream.
    async fn accept_conn(&mut self) -> Result<TcpStream> {
        loop {
            // Wait for an incoming connection.
            // The `accept()` method returns a tuple of (TcpStream, SocketAddr),
            // but we only need the TcpStream.
            match self.listener.accept().await {
                // Return the TcpStream if a connection is successfully accepted.
                Ok((sock, _)) => return Ok(sock),
                // Return an error if there is an issue accepting a connection.
                Err(e) => return Err(Error::from(e)),
            }
        }
    }
}