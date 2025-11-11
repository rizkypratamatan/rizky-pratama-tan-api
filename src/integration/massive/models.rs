use crate::integration::massive::enums::Timespan;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};

fn to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
					   where
						   D: Deserializer<'de>,
{
	let timestamp: i64 = i64::deserialize(deserializer)?;
	let secs: i64 = timestamp / 1000;
	let nsecs: i64 = (timestamp % 1000) * 1_000_000;

	Ok(Utc
		.timestamp_opt(secs, nsecs as u32)
		.single()
		.ok_or_else(|| serde::de::Error::custom(format!("Invalid timestamp: {}", timestamp)))?)
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AggregateTickerRequest {
	pub from: String,
	#[serde(rename = "apiKey")]
	pub key: String,
	pub multiplier: i64,
	#[serde(rename = "forexTicker")]
	pub ticker: String,
	pub timespan: Timespan,
	pub to: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AggregateTickerResponse {
	pub adjusted: bool,
	#[serde(rename = "queryCount")]
	pub query_count: i64,
	pub request_id: String,
	pub results: Vec<AggregateTickerResponseResult>,
	#[serde(rename = "resultsCount")]
	pub results_count: i64,
	pub status: String,
	pub ticker: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AggregateTickerResponseResult {
	#[serde(rename = "c")]
	pub close: f64,
	#[serde(rename = "h")]
	pub high: f64,
	#[serde(rename = "l")]
	pub low: f64,
	#[serde(rename = "o")]
	pub open: f64,
	#[serde(rename = "t", deserialize_with = "to_datetime")]
	pub timestamp: DateTime<Utc>,
	#[serde(rename = "n")]
	pub transaction_numbers: i64,
	#[serde(rename = "v")]
	pub volume: f64,
	#[serde(rename = "vw")]
	pub volume_weighted: f64,
}
