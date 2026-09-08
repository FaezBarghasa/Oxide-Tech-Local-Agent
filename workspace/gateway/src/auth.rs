use actix_web::{FromRequest, HttpRequest, dev::Payload};
use futures_util::future::{Ready, ready};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

pub mod api_keys;
pub mod rbac;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Admin,
    Developer,
    Viewer,
}

impl Role {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => Role::Admin,
            "developer" => Role::Developer,
            _ => Role::Viewer,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: usize,
}

pub fn generate_token(
    username: &str,
    role: Role,
    secret: &str,
    exp_hours: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(exp_hours as i64))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_string(),
        role,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

impl Claims {
    pub fn enforce_role(&self, required: Role) -> Result<(), actix_web::Error> {
        match (self.role, required) {
            (Role::Admin, _) => Ok(()), // Admin has all rights
            (Role::Developer, Role::Developer | Role::Viewer) => Ok(()),
            (Role::Viewer, Role::Viewer) => Ok(()),
            _ => Err(actix_web::error::ErrorForbidden("Insufficient permissions")),
        }
    }
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(h) => h,
            None => {
                return ready(Err(actix_web::error::ErrorUnauthorized(
                    "Missing Authorization header",
                )));
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => {
                return ready(Err(actix_web::error::ErrorUnauthorized(
                    "Invalid Authorization header",
                )));
            }
        };

        if !auth_str.starts_with("Bearer ") {
            return ready(Err(actix_web::error::ErrorUnauthorized(
                "Authorization scheme must be Bearer",
            )));
        }

        let token = &auth_str[7..];

        let config = match req.app_data::<actix_web::web::Data<common::config::AppConfig>>() {
            Some(cfg) => cfg,
            None => {
                return ready(Err(actix_web::error::ErrorInternalServerError(
                    "Server config missing",
                )));
            }
        };

        match verify_token(token, &config.auth.jwt_secret) {
            Ok(claims) => ready(Ok(claims)),
            Err(_) => ready(Err(actix_web::error::ErrorUnauthorized(
                "Invalid or expired token",
            ))),
        }
    }
}
