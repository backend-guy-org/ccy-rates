use reqwest::Error;
use serde::de::DeserializeOwned;

pub async fn get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    println!("[GET] for URL: {}", url);

    reqwest::get(url).await?.json::<T>().await
}
