use crate::integration::eodhd::models::{IntradayRequest, IntradayResponse};
use crate::integration::services::initialize_params;
use crate::trading::data::status::models::StatusAsset;
use crate::trading::data::status::services::get_symbol;
use log::{error, info};
use reqwest::Client;
use std::env;

pub async fn intraday(params: IntradayRequest) -> Option<Vec<IntradayResponse>> {
	let mut result: Option<Vec<IntradayResponse>> = None;

	let client: Client = Client::new();
	let url: String = format!(
		"{}/intraday/{}?api_token={}&fmt=json",
		env::var("API_EODHD_BASE_URL").unwrap_or_default(),
		get_symbol(&StatusAsset { asset: params.asset.clone(), suffix: params.suffix.clone() }),
		env::var("API_EODHD_TOKEN").unwrap_or_default()
	) + &initialize_params(&params);

	info!("Request to {}\nHeaders : \nBody : ", url);

	match client.get(url.clone()).send().await {
		Ok(response) => {
			let log: String = format!(
				"Response from {}\nStatus: {}\nHeaders : {:?}\n",
				url,
				response.status(),
				response.headers().clone()
			);
			let response_text: String = response.text().await.unwrap();
			info!("{}Body : {}", log, response_text);

			match serde_json::from_str(&response_text) {
				Ok(data) => {
					result = Some(data);
				}
				Err(err) => error!("{:?}", err),
			}
		}
		Err(err) => error!("{:?}", err),
	}

	result
}
