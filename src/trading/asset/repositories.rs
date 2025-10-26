use crate::core::database::models::UserReference;
use crate::core::database::services::timestamp;
use crate::trading::asset::models::Asset;
use bson::oid::ObjectId;
use bson::{doc, Document};
use log::error;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use mongodb::Database;

pub async fn find_one_by_id(database: &Database, id: &ObjectId) -> Option<Asset> {
	database
		.collection::<Asset>("trading_asset")
		.find_one(doc! {"_id": id})
		.await
		.unwrap_or_else(|err| {
			error!("{:?}", err);
			None
		})
}

pub async fn insert_one(
	database: &Database,
	data: &Asset,
	user: Option<UserReference>,
) -> Result<InsertOneResult, Error> {
	database
		.collection::<Document>("trading_asset")
		.insert_one(timestamp(data, user, &true, None))
		.await
}
