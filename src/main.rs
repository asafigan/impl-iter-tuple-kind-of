use std::net::SocketAddr;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use impl_iter_tuple_kind_of::build_app;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "impl_iter_tuple_kind_of=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = build_app();

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listing on {address}");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
