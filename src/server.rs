// muDB Echo Server
// ----------------
// This file implements a simple asynchronous echo server using Tokio.
// The server accepts multiple TCP clients, prompts for input, and echoes each line
// back to the client as a comment. It is designed to be single-threaded and easy to understand.

use anyhow::{Error, Result}; // For convenient error handling
use log::error; // For logging errors
use tokio::net::{TcpListener, TcpStream}; // For async TCP networking


/// Server struct: Holds the TCP listener for incoming connections.
#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    /// Create a new Server instance with the given TcpListener.
    pub fn new(listener: TcpListener) -> Server {
        Server { listener }
    }

    /// Run the server: accept and handle multiple clients asynchronously.
    /// Each client is prompted for input and receives an echo of their input as a comment.
    pub async fn run(&mut self) -> Result<()> {
        loop {
            // Accept a new TCP connection (or panic on error)
            let stream = match self.accept_conn().await {
                Ok(stream) => stream,
                Err(e) => {
                    error!("{}", e);
                    panic!("Error accepting connection");
                }
            };

            // Spawn a new async task for each client connection
            tokio::spawn(async move {
                use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
                use tokio::time::{sleep, Duration};
                let mut reader = BufReader::new(stream);
                let mut line = String::new();
                loop {
                    // Prompt the client for input
                    if let Err(e) = reader.get_mut().write_all(b"> ").await {
                        error!("Error writing prompt to socket: {}", e);
                        break;
                    }

                    // Read a line of input from the client
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break, // Connection closed
                        Ok(_) => {
                            // Add a 5 second delay before responding
                            sleep(Duration::from_secs(5)).await;
                            // Remove trailing newline and echo the line as a comment
                            let trimmed = line.trim_end_matches(['\r', '\n'].as_ref());
                            let response = format!("# {}\r\n", trimmed);
                            if let Err(e) = reader.get_mut().write_all(response.as_bytes()).await {
                                error!("Error writing to socket: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Error reading from socket: {}", e);
                            break;
                        }
                    }
                }
            });
        }
    }

    /// Accept a new incoming TCP connection and return the TcpStream.
    /// Returns an error if the accept fails.
    async fn accept_conn(&mut self) -> Result<TcpStream> {
        loop {
            // Wait for an incoming connection.
            // The `accept()` method returns a tuple of (TcpStream, SocketAddr),
            // but we only need the TcpStream.
            match self.listener.accept().await {
                Ok((sock, _)) => return Ok(sock),
                Err(e) => return Err(Error::from(e)),
            }
        }
    }
}