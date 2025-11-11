use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

fn to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
					   where
						   D: Deserializer<'de>,
{
	let string: &str = Deserialize::deserialize(deserializer)?;
	let date_time: NaiveDateTime = NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)?;

	Ok(DateTime::<Utc>::from_naive_utc_and_offset(date_time, Utc))
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExchangeSymbolRequest {
	pub code: String,
	pub token: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExchangeSymbolResponse {
	#[serde(rename = "Code")]
	pub code: String,
	#[serde(rename = "Country")]
	pub country: String,
	#[serde(rename = "Currency")]
	pub currency: String,
	#[serde(rename = "Exchange")]
	pub exchange: String,
	#[serde(rename = "Isin")]
	pub isin: Option<String>,
	#[serde(rename = "Name")]
	pub name: String,
	#[serde(rename = "Type")]
	pub _type: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IntradayRequest {
	pub from: Option<i64>,
	pub interval: Option<String>,
	pub symbol: String,
	pub to: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IntradayResponse {
	pub close: f64,
	#[serde(deserialize_with = "to_datetime")]
	pub datetime: DateTime<Utc>,
	pub gmtoffset: i64,
	pub high: f64,
	pub low: f64,
	pub open: f64,
	pub volume: f64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RealtimeRequest {
	pub symbol: String,
	pub s: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RealtimeResponse {
	pub change: f64,
	pub change_p: f64,
	pub close: f64,
	pub code: String,
	#[serde(deserialize_with = "to_datetime")]
	pub datetime: DateTime<Utc>,
	pub gmtoffset: i64,
	pub high: f64,
	pub low: f64,
	pub open: f64,
	#[serde(rename = "previousClose")]
	pub previous_close: f64,
	pub timestamp: DateTime<Utc>,
	pub volume: f64,
}
