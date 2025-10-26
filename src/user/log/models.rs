use crate::core::database::models::{Timestamp, UserReference};
use crate::user::log::enums::{Platform, Type};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLog {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub authentication: String,
	pub browser: UserLogBrowser,
	pub device: UserLogDevice,
	pub ip: String,
	pub os: UserLogOs,
	pub platform: Platform,
	pub remember: bool,
	#[serde(rename = "type")]
	pub _type: Type,
	pub user: UserReference,
	pub created: Timestamp,
	pub modified: Timestamp,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLogBrowser {
	pub family: String,
	pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLogDevice {
	pub bot: bool,
	pub mobile: bool,
	pub pc: bool,
	pub tablet: bool,
	pub touch_capable: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLogOs {
	pub os: String,
	pub version: String,
}
