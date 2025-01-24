use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use http::{Method, StatusCode};
use tower_http::cors::{AllowMethods, AllowOrigin, CorsLayer};
use crate::auth::auth;


pub fn get_cors_limit_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(AllowMethods::list(vec![Method::GET]))
        .allow_origin(
            AllowOrigin::exact("http://localhost".parse().unwrap())
        )
}

pub async fn jwt_check_middleware(request: Request, next: Next) -> Response {
    let headers = request.headers();
    let authorization_header = headers.get("Authorization");
    if authorization_header.is_none() {
        return (StatusCode::UNAUTHORIZED, "No Authorization header included").into_response()
    }

    let token = authorization_header.unwrap().to_str().unwrap();
    if token.len() <= 8{
        return (StatusCode::UNAUTHORIZED, "No Jwt token included").into_response()
    }

    let check_jwt_result = auth::check_jwt_token(&token[8..]);
    if check_jwt_result.is_err(){
        return (StatusCode::UNAUTHORIZED, "Invalid Jwt token").into_response()
    }

    let response =  next.run(request).await;
    response
}