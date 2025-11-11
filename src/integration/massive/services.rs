use crate::integration::massive::models::{AggregateTickerRequest, AggregateTickerResponse};
use crate::integration::services::{
	get_request, initialize_params as initialize_params_integration,
};
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

pub async fn aggregate_ticker(
	params: &AggregateTickerRequest,
) -> Option<AggregateTickerResponse> {
	let client: Client = Client::new();
	let url: String = format!(
		"{}/v2/aggs/ticker/{}/range/{}/{}/{}/{}?{}",
		env::var("API_MASSIVE_BASE_URL").unwrap_or_default(),
		params.ticker,
		params.multiplier,
		serde_json::to_string(&params.timespan)
			.unwrap_or_default()
			.trim_matches('"'),
		params.from,
		params.to,
		&initialize_params(params)
	);

	info!("Request to {}\nHeaders : \nBody : ", url);

	get_request(&client, &url).await
}

fn initialize_params<T>(params: &T) -> String
						where
							T: Serialize + for<'de> Deserialize<'de>,
{
	let url: String = initialize_params_integration(params).replace(
		"&apiKey=",
		&format!(
			"&apiKey={}",
			env::var("API_MASSIVE_KEY").unwrap_or_default()
		),
	);

	url[1..].to_string()
}
