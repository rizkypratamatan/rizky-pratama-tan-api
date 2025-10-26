use crate::core::database::models::UserReference;
use crate::core::database::services::timestamp;
use crate::trading::data::models::Data;
use bson::Document;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use mongodb::Database;

pub async fn insert_one(
	database: &Database,
	collection: &str,
	data: &Data,
	user: Option<UserReference>,
) -> Result<InsertOneResult, Error> {
	database
		.collection::<Document>(&("trading_data_".to_string() + collection))
		.insert_one(timestamp(data, user, &true, None))
		.await
}
