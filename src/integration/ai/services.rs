use crate::core::base::models::BaseResponse;
use crate::core::encryption::rsa::encrypt;
use crate::integration::ai::models::{PredictRequest, PredictResponse, TrainRequest};
use crate::integration::services::post_request;
use chrono::Utc;
use log::info;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};
use std::collections::HashMap;
use std::env;

fn initialize_body<T>(body: &T, path: &str) -> T
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    let mut map: HashMap<String, Value> =
        from_value(to_value(body).unwrap_or_default()).unwrap_or_default();

    map.insert(
        "token".to_string(),
        to_value(encrypt(
            &format!("{}~{}", path, Utc::now().to_rfc3339()),
            &env::var("API_AI_KEY").unwrap_or_default(),
        ))
        .unwrap_or_default(),
    );

    from_value(Value::Object(map.into_iter().collect())).unwrap()
}

fn initialize_header() -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").ok().unwrap(),
    );
    headers.insert(
        "pld-key",
        HeaderValue::from_str(&env::var("API_AI_KEY").unwrap_or_default())
            .ok()
            .unwrap(),
    );

    headers
}

pub async fn predict(params: &PredictRequest) -> Option<PredictResponse> {
    let client: Client = Client::new();
    let url: String = format!(
        "{}/trading/data/predict",
        env::var("API_AI_BASE_URL").unwrap_or_default()
    );

    info!("Request to {}\nHeaders : \nBody : ", url);

    post_request(
        &client,
        &url,
        &initialize_header(),
        &initialize_body(params, &"/trading/data/predict"),
    )
    .await
}

pub async fn train(params: &TrainRequest) -> Option<BaseResponse> {
    let client: Client = Client::new();
    let url: String = format!(
        "{}/trading/data/train",
        env::var("API_AI_BASE_URL").unwrap_or_default()
    );

    info!("Request to {}\nHeaders : \nBody : ", url);

    post_request(
        &client,
        &url,
        &initialize_header(),
        &initialize_body(params, &"/trading/data/train"),
    )
    .await
}
