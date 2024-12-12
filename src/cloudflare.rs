use anyhow::Result;
use serde_json::{json, Value};

pub struct CloudflareApi {
    zone_id: String,
    token: String,
}

impl CloudflareApi {
    pub fn new(zone_id: String, token: String) -> Self {
        Self { zone_id, token }
    }

    pub async fn get_records(&self) -> Result<Value, anyhow::Error> {
        let uri = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
            self.zone_id
        );
        let res = reqwest::Client::new()
            .get(uri)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(res)
    }

    pub async fn update_record(&self, record_id: String, ip: String) -> Result<(), anyhow::Error> {
        let uri = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{record_id}",
            self.zone_id
        );
        let body = json!({"content": ip});

        reqwest::Client::new()
            .patch(uri)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(())
    }
}
