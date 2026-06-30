mod config;
pub mod app;
use app::state::AppState;
use config::routes::create_router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let state = AppState {
        app_name: "Rust API".to_string(),
    };

    let app = create_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
