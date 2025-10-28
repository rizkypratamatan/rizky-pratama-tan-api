use crate::core::base::enums::Status;
use crate::core::database::models::Timestamp;
use crate::trading::asset::enums::Provider;
use crate::user::log::models::UserLog;
use bson::oid::ObjectId;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Asset {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub provider: Provider,
	pub status: Status,
	pub sync: AssetSync,
	pub ticker: String,
	pub watchlist: bool,
	pub created: Timestamp,
	pub modified: Timestamp,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AssetSync {
	#[serde(with = "chrono_datetime_as_bson_datetime")]
	pub last: DateTime<Utc>,
	pub symbol: String,
	pub synchronized: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AssetValidation {
	pub data: Asset,
	pub log: UserLog,
	pub response: String,
	pub result: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SaveAssetRequest {
	pub authentication: String,
	pub id: Option<ObjectId>,
	pub provider: Provider,
	pub symbol: String,
	pub status: Status,
	pub token: String,
}
