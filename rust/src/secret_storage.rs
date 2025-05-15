use super::Result;
use reqwest::{Client, Method, header};
use serde::Deserialize;

#[allow(dead_code)]
struct HcpClient {
    client: Client,
    access_token: String,
    expires_in: u64,
    network_url: String,
    org_id: String,
    proj_id: String,
    app_name: String,
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
        org_id: String,
        proj_id: String,
        app_name: String,
    ) -> Result<Self> {
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
            network_url: String::from("https://api.cloud.hashicorp.com"),
            org_id,
            proj_id,
            app_name,
            expires_in: body.expires_in,
            client_id,
            client_secret,
        })
    }

    async fn create_secret(&self, key: &str, value: &str) -> Result<()> {
        let url = format!(
            "{}/secrets/2023-11-28/organizations/{}/projects/{}/apps/{}/secret/kv",
            self.network_url, self.org_id, self.proj_id, self.app_name
        );

        let json = serde_json::json!({
            "name": key,
            "value": value
        });

        self.client
            .post(url)
            .bearer_auth(&self.access_token)
            .json(&json)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    async fn get_secret(&self, key: &str) -> Result<String> {
        let url = format!(
            "{}/secrets/2023-11-28/organizations/{}/projects/{}/apps/{}/secrets/{}:open",
            self.network_url, self.org_id, self.proj_id, self.app_name, key
        );

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .error_for_status()?;

        let body = response.json::<serde_json::Value>().await?;

        match body["secret"]["static_version"]["value"].as_str() {
            Some(value) => Ok(value.to_string()),
            None => Err("Failed to get secret".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HcpClient;
    use reqwest::Client;

    #[tokio::test]
    async fn test_secret_storage() {
        dotenv::dotenv().ok();
        let client_id = std::env::var("HCP_CLIENT_ID").unwrap();
        let client_secret = std::env::var("HCP_CLIENT_SECRET").unwrap();
        let org_id = std::env::var("HCP_ORG_ID").unwrap();
        let proj_id = std::env::var("HCP_PROJ_ID").unwrap();
        let app_name = std::env::var("HCP_APP_NAME").unwrap();

        let client = Client::new();
        let hcp_client =
            HcpClient::auth(&client, client_id, client_secret, org_id, proj_id, app_name)
                .await
                .unwrap();

        hcp_client
            .create_secret("new_secret", "new_secret_value")
            .await
            .unwrap();

        let value = hcp_client.get_secret("new_secret").await.unwrap();
        assert_eq!(value, "new_secret_value");
    }
}
