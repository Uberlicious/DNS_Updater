use anyhow::{anyhow, Result};

pub mod cloudflare;
use cloudflare::CloudflareApi;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    let zone_id = dotenvy::var("ZONE_ID")?;
    let cloudflare_token = dotenvy::var("CLOUDFLARE_TOKEN")?;

    let extern_address: String = reqwest::get("https://ipinfo.io/ip").await?.text().await?;
    let cf = CloudflareApi::new(zone_id, cloudflare_token);
    let data = cf.get_records().await?;
    check_ip_and_update(cf, data, &extern_address).await?;

    Ok(())
}

async fn check_ip_and_update(
    cf: CloudflareApi,
    data: serde_json::Value,
    ip: &str,
) -> Result<(), anyhow::Error> {
    let record_name = dotenvy::var("RECORD_NAME")?;
    let record_type = dotenvy::var("RECORD_TYPE")?;

    match data["result"].as_array() {
        Some(results) => {
            for res in results {
                if res.get("name").unwrap().eq(&record_name)
                    && res.get("type").unwrap().eq(&record_type)
                    && res["content"].as_str() != Some(ip)
                {
                    let id = res["id"].to_string().trim_matches('"').to_string();
                    cf.update_record(id, ip.to_string()).await?;
                }
            }
        }
        None => return Err(anyhow!("no results in data")),
    }

    Ok(())
}
