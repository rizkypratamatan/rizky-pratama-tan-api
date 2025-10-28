use crate::core::database::models::UserReference;
use crate::core::database::services::timestamp;
use crate::trading::asset::models::Asset;
use crate::user::models::User;
use bson::oid::ObjectId;
use bson::{doc, Document};
use log::error;
use mongodb::error::Error;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::Database;

pub async fn find_one_by_id(database: &Database, id: &ObjectId) -> Option<Asset> {
	database.collection::<Asset>("user").find_one(doc! {"_id": id}).await.unwrap_or_else(|err| {
		error!("{:?}", err);
		None
	})
}

pub async fn find_one_by_username(database: &Database, username: &str) -> Option<Asset> {
	database.collection::<Asset>("user").find_one(doc! {"username": username}).await.unwrap_or_else(|err| {
		error!("{:?}", err);
		None
	})
}

pub async fn insert_one(
	database: &Database,
	data: &User,
	user: Option<UserReference>,
) -> Result<InsertOneResult, Error> {
	database.collection::<Document>("user").insert_one(timestamp(data, user, &true, None)).await
}

pub async fn replace_one(
	database: &Database,
	data: &User,
	user: Option<UserReference>,
) -> Result<UpdateResult, Error> {
	database.collection::<Document>("user").replace_one(doc! {"_id": data.id}, timestamp(data, user, &true, None)).await
}

pub async fn log_insert_one(
	database: &Database,
	data: &User,
	user: Option<UserReference>,
) -> Result<InsertOneResult, Error> {
	database.collection::<Document>("user_log_data").insert_one(timestamp(data, user, &true, Some("user_id".to_string()))).await
}
