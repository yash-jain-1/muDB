use crate::handler::FrameHandler;
// muDB Echo Server
// ----------------
// This file implements a simple asynchronous echo server using Tokio.
// The server accepts multiple TCP clients, prompts for input, and echoes each line
// back to the client as a comment. It is designed to be single-threaded and easy to understand.

use anyhow::{Error, Result};
use log::error;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Framed;
use crate::resp::frame::RespCommandFrame;
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
            let sock = match self.accept_conn().await {
                Ok(stream) => stream,
                Err(e) => {
                    error!("{}", e);
                    panic!("Error accepting connection");
                }
            };

            // Use RespCommandFrame codec to read incoming TCP messages as Redis command frames,
            // and to write RespType values into outgoing TCP messages.
            let resp_command_frame = Framed::with_capacity(sock, RespCommandFrame::new(), 8 * 1024);

             // Spawn a new asynchronous task to handle the connection.
             // This allows the server to handle multiple connections concurrently.
             tokio::spawn(async move {
                let handler = FrameHandler::new(resp_command_frame);
                if let Err(e) = handler.handle().await {
                    error!("Failed to handle command: {}", e);
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