use bytes::BytesMut;
use crate::resp::types::RespType;
// muDB Echo Server
// ----------------
// This file implements a simple asynchronous echo server using Tokio.
// The server accepts multiple TCP clients, prompts for input, and echoes each line
// back to the client as a comment. It is designed to be single-threaded and easy to understand.

use anyhow::{Error, Result}; // For convenient error handling
use log::error; // For logging errors
use tokio::net::{TcpListener, TcpStream}; // For async TCP networking
use tokio::io::{AsyncReadExt, AsyncWriteExt};


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
            let mut sock = match self.accept_conn().await {
                Ok(stream) => stream,
                Err(e) => {
                    error!("{}", e);
                    panic!("Error accepting connection");
                }
            };

            // Spawn a new async task for each client connection
            tokio::spawn(async move {
                // read the TCP message and move the raw bytes into a buffer
                let mut buffer = BytesMut::with_capacity(512);
                let n = match sock.read_buf(&mut buffer).await {
                    Ok(n) if n == 0 => return, // Connection closed, do nothing
                    Ok(_) => {},
                    Err(e) => panic!("Error reading request: {}", e),
                };

                // Only parse if we actually received data
                let resp_data = match RespType::parse(buffer) {
                    Ok((data, _)) => data,
                    Err(e) => RespType::SimpleError(format!("{}", e)),
                };

                // Echo the RESP message back to the client.
                if let Err(e) = sock.write_all(&resp_data.to_bytes()[..]).await {
                    // Log the error and panic if there is an issue writing the response.
                    error!("{}", e);
                    panic!("Error writing response");
                }
                // The connection is closed automatically when `sock` goes out of scope.
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