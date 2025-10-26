use crate::core::database::models::Timestamp;
use crate::trading::data::status::enums::Status as DataStatus;
use bson::oid::ObjectId;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Status {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub asset: String,
	pub interval: String,
	#[serde(with = "chrono_datetime_as_bson_datetime")]
	pub last: DateTime<Utc>,
	pub status: DataStatus,
	pub created: Timestamp,
	pub modified: Timestamp,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StatusAsset {
	pub asset: String,
	pub suffix: Option<String>,
}
