mod routes;

use routes::get_routes;

pub async fn run() {
    let app = get_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3040").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}