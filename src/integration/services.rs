use log::{error, info};
use reqwest::header::HeaderMap;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn initialize_params<T>(params: &T) -> String
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    let mut url: String = "".to_string();

    let json: Value = serde_json::to_value(&params).unwrap();

    if let Value::Object(map) = json {
        for (key, value) in map {
            if !value.is_null() {
                if value.is_string() {
                    url += &format!("&{}={}", key, value.as_str().unwrap_or_default());
                } else {
                    url += &format!("&{}={}", key, value);
                }
            }
        }
    }

    url
}

async fn initialize_response<T>(response: Response, url: &str) -> Option<T> where
    T: Serialize + for<'de> Deserialize<'de>,
{
    let log: String = format!(
        "Response from {}\nStatus: {}\nHeaders : {:?}\n",
        url,
        response.status(),
        response.headers().clone()
    );
    let response_text: String = response.text().await.unwrap();
    info!("{}Body : {}", log, response_text);

    match serde_json::from_str(&response_text) {
        Ok(data) => Some(data),
        Err(err) => {
            error!("{:?}", err);
            None
        }
    }
}

pub async fn get_request<T>(client: &Client, url: &str) -> Option<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    match client.get(url).send().await {
        Ok(response) => initialize_response(response, url).await,
        Err(err) => {
            error!("{:?}", err);
            None
        }
    }
}

pub async fn post_request<T, U>(
    client: &Client,
    url: &str,
    headers: &HeaderMap,
    body: &T,
) -> Option<U>
where
    T: Serialize + for<'de> Deserialize<'de>,
    U: Serialize + for<'de> Deserialize<'de>,
{
    match client
        .post(url)
        .headers(headers.clone())
        .body(serde_json::to_string(body).unwrap())
        .send()
        .await
    {
        Ok(response) => initialize_response(response, url).await,
        Err(err) => {
            error!("{:?}", err);
            None
        }
    }
}
