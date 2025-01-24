use std::env;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use axum::http::{StatusCode};
use axum::{Json, Router};
use axum::extract::{State};
use axum::response::{IntoResponse};
use axum::routing::post;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use crate::errors::errors::CustomError;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use crate::AppState;

#[derive(Deserialize)]
struct Params {
    username: String,
    password: String
}

#[derive(Serialize)]
struct JwtTokenResponse {
    jwt: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    pub iat: u64,
    pub user_id: i32
}

pub fn get_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login)).with_state(app_state)
}

#[axum::debug_handler]
async fn login(State(app_state): State<Arc<AppState>>, Json(params): Json<Params> ) -> impl IntoResponse {
    let key = env::var("SECRET_KEY").expect("Secret key couldn't be read");

    let result = app_state.user_repository.as_ref().unwrap().read().await.find_by_email(&params.username).await;
    match result {
        Ok(user) => {
            let check_hash_result = check_hash(&params.password,&user.password);
            if check_hash_result {
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
                let jwt_token_response = JwtTokenResponse {
                    jwt: token,
                };
                Ok(Json(jwt_token_response))
            }else{
                Err(StatusCode::UNAUTHORIZED)
            }
        },
        Err(_) => {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}






pub fn check_jwt_token(token: &str) -> Result<Claims, CustomError> {
    let key = env::var("SECRET_KEY").expect("Secret key couldn't be read");
    let token_result = decode::<Claims>(&token, &DecodingKey::from_secret(key.as_ref()), &Validation::default());
    if let Ok(token_data) = token_result {
        Ok(token_data.claims)
    } else{
        println!("{:?}", token_result.err().unwrap());
        info!("Invalid JWT Token : {}" , token);
        Err(CustomError::InvalidJwtToken)
    }
}


pub fn generate_hash(password: &str) -> String {
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

    password_hash
}

pub fn check_hash(password: &str , hashed_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()

}
