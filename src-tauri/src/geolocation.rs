use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::models::LocationInfo;

#[derive(Debug, Deserialize, Serialize)]
struct IpGeolocationResponse {
    latitude: String,
    longitude: String,
    time_zone: TimeZone,
    continent_code: String,
    country_code2: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimeZone {
    name: String,
}

pub async fn get_location_from_ip() -> Result<LocationInfo> {
    let api_key = "b9aed80a71d043149013540fb449a384";
    let url = format!("https://api.ipgeolocation.io/ipgeo?apiKey={}", api_key);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Muezzin")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(crate::error::AppError::Custom(
            format!("Failed to get geolocation: {}", response.status())
        ));
    }

    let geo_data: IpGeolocationResponse = response.json().await?;

    let latitude = geo_data.latitude.parse::<f64>()
        .map_err(|e| crate::error::AppError::Custom(format!("Invalid latitude: {}", e)))?;
    
    let longitude = geo_data.longitude.parse::<f64>()
        .map_err(|e| crate::error::AppError::Custom(format!("Invalid longitude: {}", e)))?;

    Ok(LocationInfo {
        latitude,
        longitude,
        timezone: geo_data.time_zone.name,
        continent_code: Some(geo_data.continent_code),
        country_code: Some(geo_data.country_code2),
    })
}

pub fn get_default_calculation_method(continent_code: &str, country_code: &str) -> String {
    match country_code {
        "RU" => "Russia",
        "GB" => "ISNA",
        "SG" => "Singapore",
        "QA" => "Qatar",
        "TR" => "Turkey",
        "IR" => "Tehran",
        "KW" => "Kuwait",
        "AE" => "Dubai",
        "PK" => "Karachi",
        "EG" => "Egyptian",
        "SA" => "UAQ",
        _ => match continent_code {
            "NA" => "ISNA",
            "EU" => "MWL",
            "AS" => "ISNA",
            "SA" => "MWL",
            "OC" => "MWL",
            "AN" => "MC",
            _ => "MWL",
        }
    }.to_string()
}
