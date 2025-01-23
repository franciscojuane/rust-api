use std::time::Duration;
use axum::http::HeaderValue;
use tower::{Layer, ServiceBuilder};
use tower::limit::rate::RateLimitLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn get_service() -> ServiceBuilder<L> {

}

pub fn get_rate_limit_layer<S>() -> impl Layer<S> {
    RateLimitLayer::new(100, Duration::from_secs(1))
}

pub fn get_cors_limit_layer<S>() -> impl Layer<S> {
    CorsLayer::new()
        .allow_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        .allow_origin(
            AllowOrigin::exact("http://localhost".parse().unwrap())
        )
}