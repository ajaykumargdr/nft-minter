use actix_web::{FromRequest, HttpRequest, dev::Payload, error as actix_error, http};
use jsonwebtoken::{self as jwt, Algorithm, DecodingKey, Validation};
use serde_json::json;
use std::future;

pub struct AuthenticationGuard {
    #[allow(dead_code)]
    pub user_id: String,
}

impl FromRequest for AuthenticationGuard {
    type Error = actix_error::Error;
    type Future = future::Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return future::ready(Err(actix_error::ErrorUnauthorized(
                json!({"status": "fail", "message": "You are not logged in, please provide token"}),
            )));
        }

        // Todo : have a config in the state
        let jwt_secret = match std::env::var("JWT_SECRET") {
            Ok(secret) => secret,
            Err(_) => {
                return future::ready(Err(actix_error::ErrorInternalServerError(
                    json!({"status": "fail", "message": "Internal Server Error"}),
                )));
            }
        };

        let decode = jwt::decode::<super::types::TokenClaims>(
            token.unwrap().as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => {
                let users = super::USERS.lock().unwrap();
                let user = users.iter().find(|user| user.id == token.claims.sub);

                if user.is_none() {
                    return future::ready(Err(actix_error::ErrorUnauthorized(
                        json!({"status": "fail", "message": "User belonging to this token no logger exists"}),
                    )));
                }

                future::ready(Ok(AuthenticationGuard {
                    user_id: token.claims.sub,
                }))
            }
            Err(_) => future::ready(Err(actix_error::ErrorUnauthorized(
                json!({"status": "fail", "message": "Invalid token or user doesn't exists"}),
            ))),
        }
    }
}
