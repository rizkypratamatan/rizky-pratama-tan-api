use crate::core::base::enums::Status;
use crate::core::database::models::Timestamp;
use crate::user::log::models::UserLog;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Asset {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub name: String,
	pub status: Status,
	pub created: Timestamp,
	pub modified: Timestamp,
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
	pub name: String,
	pub status: Status,
	pub token: String,
}
