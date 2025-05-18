use crate::{blockchain, utils};
use std::hash::{Hash, Hasher};

use super::{
    Result,
    types::{ActixContext, QueryParams, TokenClaims, User},
};
use actix_web::{
    HttpResponse, Responder,
    cookie::{Cookie, time::Duration as CookieDuration},
    http::header::LOCATION,
    web,
};
use jsonwebtoken::{EncodingKey, Header};
use reqwest::{Client, Url};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

pub async fn request_token(client: &Client, authorization_code: &str) -> Result<OAuthResponse> {
    // Todo : have a config in the state
    let redirect_url = std::env::var("GOOGLE_OAUTH_REDIRECT_URL")?;
    let client_secret = std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")?;
    let client_id = std::env::var("GOOGLE_OAUTH_CLIENT_ID")?;
    let token_url = "https://oauth2.googleapis.com/token";

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url.as_str()),
        ("client_id", client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", client_secret.as_str()),
    ];

    let response = client
        .post(token_url)
        .form(&params)
        .send()
        .await?
        .error_for_status()?;

    let oauth_response = response.json::<OAuthResponse>().await?;
    Ok(oauth_response)
}

pub async fn get_google_user(
    client: &Client,
    access_token: &str,
    id_token: &str,
) -> Result<GoogleUserResult> {
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    url.query_pairs_mut()
        .append_pair("alt", "json")
        .append_pair("access_token", access_token);

    let response = client
        .get(url)
        .bearer_auth(id_token)
        .send()
        .await?
        .error_for_status()?;

    Ok(response.json::<GoogleUserResult>().await?)
}

#[actix_web::get("/auth/google")]
async fn google_oauth_handler(
    context: web::Data<ActixContext>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    if query.auth_code.is_empty() {
        return HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "fail", "message": "Authorization code not provided!"}),
        );
    }

    let token_response = request_token(&context.http_client, &query.auth_code).await;

    if token_response.is_err() {
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": "Token request failed!"}));
    }

    let token_response = token_response.unwrap();

    let google_user = get_google_user(
        &context.http_client,
        &token_response.access_token,
        &token_response.id_token,
    )
    .await;

    if google_user.is_err() {
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": "Google user request failed!"}));
    }

    let google_user = google_user.unwrap();
    let mut users = super::USERS.lock().unwrap();
    let google_email = google_user.email.to_lowercase();

    let user = match users.iter().find(|user| user.email == google_email) {
        Some(user) => user.clone(),
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            let new_pk = blockchain::create_eth_account().unwrap();
            let shares = utils::split_secret(&new_pk.credential().to_bytes()).unwrap();

            let user = User {
                id: id.clone(),
                email: google_email,
                key_shares: [shares[0].to_string(), shares[1].to_string()],
                wallet_address: new_pk.address().to_string(),
            };

            users.push(user.clone());

            let mut hasher = std::hash::DefaultHasher::new();
            id.hash(&mut hasher);
            let key = format!("S{}", &hasher.finish());

            // Todo : improve encryption
            match context.secret_manager.create_secret(&key, &shares[2]).await {
                Ok(_) => {}
                Err(_e) => {
                    // Todo: add logs
                    println!("creating secret failed! {:?}", _e);

                    return HttpResponse::InternalServerError().json(
                        serde_json::json!({"status": "fail", "message": "Internal Server Error"}),
                    );
                }
            };

            user
        }
    };

    let jwt_secret = std::env::var("JWT_SECRET").unwrap();
    let jwt_max_age = std::env::var("TOKEN_MAXAGE")
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(jwt_max_age)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id,
        exp,
        iat,
    };

    let jwt_token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", jwt_token)
        .path("/")
        .max_age(CookieDuration::new(60 * jwt_max_age, 0))
        .http_only(true)
        .finish();

    let frontend_origin = std::env::var("CLIENT_ORIGIN").unwrap().to_owned();

    HttpResponse::SeeOther()
        .append_header((LOCATION, frontend_origin))
        .cookie(cookie)
        .finish()
}
