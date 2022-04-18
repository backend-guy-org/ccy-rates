use crate::rest_adapter::get::get;
use serde::Deserialize;
use std::env;

//https://exchangeratesapi.io/
pub struct ERAApi {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct ERARatesResponse {
    #[serde(rename = "rates")]
    pub rates: EraRatesCcys,
}

#[derive(Deserialize, Debug)]
pub struct EraRatesCcys {
    #[serde(rename = "EUR")]
    pub eur: f32,
    #[serde(rename = "USD")]
    pub usd: f32,
}

impl Default for ERAApi {
    fn default() -> ERAApi {
        ERAApi {
            url: get_full_era_api_endpoint(),
        }
    }
}

impl ERAApi {
    pub async fn refresh_rates(self) -> ERARatesResponse {
        let response = get::<ERARatesResponse>(self.url).await;
        match response {
            Ok(r) => r,
            Err(e) => {
                panic!("Error when calling ERAApi.refresh_rates: {}", e);
            }
        }
    }
}

fn get_full_era_api_endpoint() -> String {
    let endpoint = env::var("EXCHANGE_RATES_API_ENDPOINT").unwrap();
    let access_key = env::var("EXCHANGE_RATES_API_ACCESS_KEY").unwrap();

    endpoint + "?access_key=" + access_key.as_str()
}
