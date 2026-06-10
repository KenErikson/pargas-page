use axum::{
    routing::get,
    Router,
};
use axum::http::header::CACHE_CONTROL;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }) )
        // .route("/", get(root));
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(tower_http::set_header::SetRequestHeaderLayer::overriding(
                    CACHE_CONTROL,
                    axum::http::HeaderValue::from_static("public, max-age=30, s-maxage=30, stale-while-revalidate=60")
                ))
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}