use anyhow::*;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use std::net::{Ipv4Addr, SocketAddr};
use tokio_util::sync::CancellationToken;

pub struct Server {
    cancellation_token: CancellationToken,
    addr: SocketAddr,
}

impl Default for Server {
    fn default() -> Self {
        let cancellation_token = CancellationToken::new();
        let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 6969));
        Server {
            cancellation_token,
            addr,
        }
    }
}

impl Server {
    pub fn stop(&self) {
        self.cancellation_token.cancel();
    }
    async fn get_index() -> impl IntoResponse {
        Html("<h1>Hello, World!</h1>")
    }

    async fn get_stop(
        Extension(cancellation_token): Extension<CancellationToken>,
    ) -> impl IntoResponse {
        cancellation_token.cancel();
        "i am going to stop"
    }

    async fn serve(&self) -> anyhow::Result<()> {
        let app = Router::new().route("/", get(Self::get_index)).route(
            "/stop",
            get(Self::get_stop).layer(Extension(self.cancellation_token.clone())),
        );
        axum::Server::try_bind(&self.addr)?
            .serve(app.into_make_service())
            .with_graceful_shutdown(self.cancellation_token.cancelled())
            .await?;
        Ok(())
    }
    pub async fn retry_until_cancellation(cancellation_token: &CancellationToken) {
        let server = Server {
            cancellation_token: cancellation_token.clone(),
            ..Default::default()
        };
        while !cancellation_token.is_cancelled() {
            if let Err(e) = server.serve().await {
                eprintln!("{}", e)
            }
        }
    }
}
