use reqwest::{Client, Method, header};
use serde::Deserialize;

#[allow(dead_code)]
struct HcpClient {
    client: Client,
    access_token: String,
    expires_in: u64,
    client_id: String,
    client_secret: String,
}


#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct HcpAuth {
    access_token: String,
    expires_in: u64, // seconds
}

#[allow(dead_code)]
impl HcpClient {
    async fn auth(
        client: &Client,
        client_id: String,
        client_secret: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let params = [
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "client_credentials"),
            ("audience", "https://api.hashicorp.cloud"),
        ];

        let response = client
            .request(Method::POST, "https://auth.idp.hashicorp.com/oauth2/token")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?
            .error_for_status()?;

        let body = response.json::<HcpAuth>().await?;

        Ok(Self {
            client: client.clone(),
            access_token: body.access_token,
            expires_in: body.expires_in,
            client_id,
            client_secret,
        })
    }
}

#[tokio::test]

async fn test_auth() {
    dotenv::dotenv().ok();
    let client = Client::new();
    let client_id = std::env::var("HCP_CLIENT_ID").unwrap();
    let client_secret = std::env::var("HCP_CLIENT_SECRET").unwrap();
    let res = HcpClient::auth(&client, client_id, client_secret).await;
    assert!(res.is_ok());
}
