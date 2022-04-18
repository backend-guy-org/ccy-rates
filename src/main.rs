use dotenv::dotenv;
use std::env;

use ccy_rates::run;
use ccy_rates::Config;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables

    run(Config {
        exchange_rates_api_access_key: env::var("EXCHANGE_RATES_API_ACCESS_KEY").unwrap(),
        exchange_rates_api_endpoint: env::var("EXCHANGE_RATES_API_ENDPOINT").unwrap(),
    })
    .await;

    loop {}
}
