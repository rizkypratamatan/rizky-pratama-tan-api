use crate::integration::eodhd::models::{ExchangeSymbolRequest, ExchangeSymbolResponse, IntradayRequest, IntradayResponse, RealtimeRequest, RealtimeResponse};
use crate::integration::services::{get_request, initialize_params};
use log::info;
use reqwest::Client;
use std::env;

pub async fn exchange_symbol(params: &ExchangeSymbolRequest) -> Option<Vec<ExchangeSymbolResponse>> {
	let client: Client = Client::new();
	let url: String = format!(
		"{}/exchange-symbol-list/{}?api_token={}&fmt=json",
		env::var("API_EODHD_BASE_URL").unwrap_or_default(),
		params.code,
		env::var("API_EODHD_TOKEN").unwrap_or_default()
	) + &initialize_params(params);

	info!("Request to {}\nHeaders : \nBody : ", url);

	get_request(&client, &url).await
}

pub async fn intraday(params: &IntradayRequest) -> Option<Vec<IntradayResponse>> {
	let client: Client = Client::new();
	let url: String = format!(
		"{}/intraday/{}?api_token={}&fmt=json",
		env::var("API_EODHD_BASE_URL").unwrap_or_default(),
		params.symbol,
		env::var("API_EODHD_TOKEN").unwrap_or_default()
	) + &initialize_params(params);

	info!("Request to {}\nHeaders : \nBody : ", url);

	get_request(&client, &url).await
}

pub async fn realtime(params: &RealtimeRequest) -> Option<Vec<RealtimeResponse>> {
	let client: Client = Client::new();
	let url: String = format!(
		"{}/real-time/{}?api_token={}&fmt=json",
		env::var("API_EODHD_BASE_URL").unwrap_or_default(),
		params.symbol,
		env::var("API_EODHD_TOKEN").unwrap_or_default()
	) + &initialize_params(params);

	info!("Request to {}\nHeaders : \nBody : ", url);

	get_request(&client, &url).await
}
