use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

fn to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
					   where
						   D: Deserializer<'de>,
{
	let string: &str = Deserialize::deserialize(deserializer)?;
	let date_time: NaiveDateTime = NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S")
		.map_err(serde::de::Error::custom)?;

	Ok(DateTime::<Utc>::from_naive_utc_and_offset(date_time, Utc))
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IntradayRequest {
	pub asset: String,
	pub from: Option<i64>,
	pub interval: Option<String>,
	pub suffix: Option<String>,
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
