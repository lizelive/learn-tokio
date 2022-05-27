#![allow(unused_imports, dead_code)]
use tokio::{join, signal::ctrl_c, spawn, sync::watch, try_join};
use tokio_util::sync::CancellationToken;

use crate::server::Server;

mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let cancellation_token = CancellationToken::new();

    let server = Server::retry_until_cancellation(&cancellation_token);

    // handle keyboard interupts
    handle_interrupt(cancellation_token.clone());

    // wait for all of the tasks to complete
    join!(server);
    Ok(())
}

fn handle_interrupt(cancellation_token: CancellationToken) {
    spawn({
        async move {
            if let Err(e) = ctrl_c().await {
                println!("Error: {}", e);
            }
            cancellation_token.cancel()
        }
    });
}
