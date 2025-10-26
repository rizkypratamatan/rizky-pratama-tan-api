use bson::oid::ObjectId;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Timestamp {
	#[serde(with = "chrono_datetime_as_bson_datetime")]
	pub timestamp: DateTime<Utc>,
	pub user: UserReference,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserReference {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub username: String,
}
