use axum::{
    extract::{ConnectInfo, State},
    response::Html,
    routing::get,
    Router,
};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    let mut args = env::args();
    let _arg0 = args.next();
    let port: u16 = args.next().map(|s| s.parse().unwrap()).unwrap_or(8080);
    let text = args.next().unwrap_or("Hello World".to_string());

    let app = Router::new()
        .route("/", get(handler))
        .with_state(text)
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(text): State<String>,
) -> Html<String> {
    println!("Request from {}", addr);
    let text = format!("<p>{}</p>", text);
    Html(text)
}
