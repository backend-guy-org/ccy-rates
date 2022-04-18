use crate::era_adapter::main::ERAApi;

pub async fn refresh() {
    let era_api = ERAApi {
        ..Default::default()
    };
    let response = era_api.refresh_rates();

    println!("response {:?}", response.await.rates);
}
