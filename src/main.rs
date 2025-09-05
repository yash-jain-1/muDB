// Include the server module defined in server.rs
mod server;
mod resp;
pub mod handler;
mod command;
mod storage;


// Import necessary crates and modules
use crate::server::Server;
use anyhow::Result;
use log::info;
use clap::Parser;
use tokio::net::TcpListener;


const DEFAULT_PORT: u16 = 6380;

#[derive(Debug, Parser)]
#[command(
    name = "MuDB-server",
    version,
    author,
    about = "A RESP based in-memory cache"
)]
struct Cli {
    /// Port to be bound to MuDB server
    #[arg(long)]
    port: Option<u16>,
}


#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger.
    // This sets up logging based on the RUST_LOG environment variable
    env_logger::init();

    // Print MuDB bull and sign
    println!(r#"
              
 ███╗   ███╗         ██████╗ ██████╗      
 ████╗ ████║         ██╔══██╗██╔══██╗  
 ██╔████╔██║██║   ██║██║  ██║██║██║
 ██║╚██╔╝██║██║   ██║██║  ██║██║  ██║
 ██║ ╚═╝ ██║╚██████╔╝██████╔╝██████╔╝
 ╚═╝     ╚═╝ ╚═════╝ ╚═════╝ ╚═════╝ 

          ⚡ MuDB ⚡
     (In-Memory Database)
    "#);

    // Get port from --port CLI parameter. Defaults to 6379
    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);

    // Define the address and port for the TCP server to listen on
    // Here we're using localhost (127.0.0.1) and port 6379 (commonly used for Redis)
    let addr = format!("127.0.0.1:{}", port);

    // Attempt to bind the TCP listener to the specified address and port
    let listener = match TcpListener::bind(&addr).await {
        // If successful, return the TcpListener
        Ok(tcp_listener) => {
            info!("TCP listener started on port {}", port);
            tcp_listener
        },
        // If there is an error, panic and print the error message
        // This could happen if the port is already in use, for example
        Err(e) => panic!("Could not bind the TCP listener to {}. Err: {}", &addr, e),
    };
    // initialize shared storage
    let shared_storage = storage::db::Storage::new(storage::db::DB::new());

    // Create a new instance of the Server with the bound TcpListener
    let mut server = Server::new(listener, shared_storage);
    // Run the server to start accepting and handling connections
    // This will run indefinitely until the program is terminated
    server.run().await?;

    // This Ok(()) is technically unreachable as server.run() loops infinitely,
    // but it's needed to satisfy the Result return type of main()
    Ok(())
}