
use std::env;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct AQI {
    main: AQIInfo,
}
#[derive(Debug, Deserialize, Serialize)]
struct AQIInfo {
    aqi: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let city = env::args()
        .nth(1)
        .unwrap_or_else(|| "London".to_string());
    let api_key = env::var("YOUR_API_KEY").expect("YOUR_API_KEY not set");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/air_pollution?appid={}&q={}",
        api_key, city
    );
    let response = reqwest::get(&url).await?.json::<AQI>().await?;

    println!("{:?}", response);

    Ok(())
}

