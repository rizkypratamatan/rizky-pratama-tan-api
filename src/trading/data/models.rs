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
	pub close: f64,
	#[serde(with = "chrono_datetime_as_bson_datetime")]
	pub datetime: DateTime<Utc>,
	pub high: f64,
	pub low: f64,
	pub open: f64,
	pub timeframe: Timeframe,
	pub volume: f64,
	pub created: Timestamp,
	pub modified: Timestamp,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DataAnalysis {
	pub classification: Analysis,
	pub prediction: Analysis,
}

#[derive(Clone, Debug)]
pub struct DataSyncArc {
	pub asset: Asset,
	pub database: Database,
	pub timeframe: Timeframe,
}
