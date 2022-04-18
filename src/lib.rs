pub mod application;
pub mod era_adapter;
pub mod rest_adapter;
pub mod scheduler_adapter;

use scheduler_adapter::main::Scheduler;

#[derive(Debug)]
pub struct Config {
    pub exchange_rates_api_access_key: String,
    pub exchange_rates_api_endpoint: String,
}

pub async fn run(config: Config) {
    println!("Starting project with config: {:?}", config);

    Scheduler::init();
}
