use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Default, Deserialize, EnumIter, Eq, PartialEq, Serialize)]
pub enum Timespan {
	#[serde(rename = "day")]
	Day,
	#[default]
	#[serde(rename = "hour")]
	Hour,
	#[serde(rename = "minute")]
	Minute,
	#[serde(rename = "month")]
	Month,
	#[serde(rename = "quarter")]
	Quarter,
	#[serde(rename = "second")]
	Second,
	#[serde(rename = "week")]
	Week,
	#[serde(rename = "year")]
	Year,
}
