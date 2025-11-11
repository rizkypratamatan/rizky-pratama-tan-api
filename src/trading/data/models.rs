use crate::core::database::models::Timestamp;
use crate::trading::asset::models::Asset;
use crate::trading::data::enums::{Analysis, Timeframe};
use bson::oid::ObjectId;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Data {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub analysis: DataAnalysis,
	pub change: DataChange,
	#[serde(with = "chrono_datetime_as_bson_datetime")]
	pub datetime: DateTime<Utc>,
	pub price: DataPrice,
	pub timeframe: Timeframe,
	pub volume: f64,
	pub created: Timestamp,
	pub modified: Timestamp,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DataAnalysis {
	pub classification: Analysis,
	pub prediction: Analysis,
	pub target: f64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DataChange {
	pub amount: f64,
	pub percentage: f64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DataPrice {
	pub close: f64,
	pub high: f64,
	pub low: f64,
	pub open: f64,
}

#[derive(Clone, Debug)]
pub struct DataSyncArc {
	pub asset: Asset,
	pub database: Database,
	pub timeframe: Timeframe,
}
