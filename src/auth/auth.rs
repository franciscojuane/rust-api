use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use axum::http::{HeaderMap, StatusCode};
use axum::{Json, Router};
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use sea_orm::sea_query::ExprTrait;
use serde::{Deserialize, Serialize};
use crate::errors::errors::CustomError;

#[derive(Deserialize)]
struct Params {
    username: String,
    password: String
}

struct JwtTokenResponse {
    jwt: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: u64,
    iat: u64,
    user_id: i32
}

pub fn get_router() -> Router {
    Router::new().route("/login", post(login))
}

#[axum::debug_handler]
async fn login(Json(params): Json<Params>) -> impl IntoResponse{
    let key = env::var("SECRET_KEY").expect("Secret key couldn't be read");
    if params.username == "admin" && params.password == "admin" {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let my_claims = Claims {
            exp: now + 3600,
            iat: now,
            user_id: 1,
        };
        let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key.as_ref())).expect("");
        Ok(Json(token))
    }else{
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn jwt_check_middleware(request: Request, next: Next) -> Response {
    let headers = request.headers();
    let authorization_header = headers.get("Authorization");
    if (authorization_header.is_none()) {
            return (StatusCode::UNAUTHORIZED, "No Authorization header included").into_response()
    }

    let token = authorization_header.unwrap().to_str().unwrap();
    if (token.len() <= 8){
        return (StatusCode::UNAUTHORIZED, "No Jwt token included").into_response()
    }

    let check_jwt_result = check_jwt_token(&token[8..]);
    if check_jwt_result.is_err(){
        return (StatusCode::UNAUTHORIZED, "Invalid Jwt token").into_response()
    }

    let claims = check_jwt_result.unwrap();
    let response =  next.run(request).await;
    response
}





fn check_jwt_token(token: &str) -> Result<Claims, CustomError> {
    let key = env::var("SECRET_KEY").expect("Secret key couldn't be read");
    let token_result = decode::<Claims>(&token, &DecodingKey::from_secret(key.as_ref()), &Validation::default());
    if let Ok(tokenData) = token_result {
        Ok(tokenData.claims)
    } else{
        println!("{:?}", token_result.err().unwrap());
        info!("Invalid JWT Token : {}" , token);
        Err(CustomError::INVALID_JWT_TOKEN)
    }
}