use crate::core::base::enums::{Country, Language, Sidebar, Status, Version};
use crate::core::database::models::Timestamp;
use crate::user::enums::{Gender, UserType};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct User {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	pub address: UserAddress,
	pub avatar: String,
	pub contact: UserContact,
	pub gender: Gender,
	pub language: Language,
	pub layout: UserLayout,
	pub name: UserName,
	pub password: UserPassword,
	pub status: Status,
	#[serde(rename = "type")]
	pub _type: UserType,
	pub username: String,
	pub created: Timestamp,
	pub modified: Timestamp,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserAddress {
	pub city: String,
	pub country: Country,
	pub district: String,
	pub state: String,
	pub street: String,
	pub zip: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserContact {
	pub discord: String,
	pub email: String,
	pub phone: String,
	pub skype: String,
	pub slack: String,
	pub telegram: String,
	pub wechat: String,
	pub whatsapp: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLayout {
	pub sidebar: Sidebar,
	pub version: Version,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserName {
	pub first: String,
	pub last: String,
	pub middle: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserPassword {
	pub main: String,
	pub recovery: String,
}
