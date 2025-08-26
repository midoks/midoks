use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
struct IpApiResponse {
    country: Option<String>,
    #[serde(rename = "regionName")]
    region_name: Option<String>,
    city: Option<String>,
    isp: Option<String>,
    status: String,
}

pub struct GeoLocator {
    client: Client,
}

impl GeoLocator {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_location(&self, ip: IpAddr) -> Result<String> {
        let url = format!("http://ip-api.com/json/{}", ip);
        
        let response: IpApiResponse = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        if response.status == "success" {
            let mut location_parts = Vec::new();
            
            if let Some(city) = response.city {
                location_parts.push(city);
            }
            if let Some(region) = response.region_name {
                location_parts.push(region);
            }
            if let Some(country) = response.country {
                location_parts.push(country);
            }
            if let Some(isp) = response.isp {
                location_parts.push(format!("ISP: {}", isp));
            }
            
            Ok(location_parts.join(", "))
        } else {
            Ok("Unknown".to_string())
        }
    }
}