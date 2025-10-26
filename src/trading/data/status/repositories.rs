use crate::core::database::models::UserReference;
use crate::core::database::services::timestamp;
use crate::trading::data::status::enums::Status as DataStatus;
use crate::trading::data::status::models::Status;
use bson::{doc, Document};
use log::error;
use mongodb::error::Error;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::Database;
use serenity::futures::TryStreamExt;

pub async fn find_one_by_asset(database: &Database, asset: &str) -> Option<Status> {
	database
		.collection::<Status>("trading_data_status")
		.find_one(doc! {"asset": asset})
		.await
		.unwrap_or_else(|err| {
			error!("{:?}", err);
			None
		})
}

pub async fn find_by_status(database: &Database, status: &DataStatus) -> Option<Vec<Status>> {
	match database
		.collection::<Status>("trading_data_status")
		.find(doc! {"status": format!("{:?}", status)})
		.await
	{
		Ok(cursor) => match cursor.try_collect().await {
			Ok(status) => Some(status),
			Err(err) => {
				error!("{:?}", err);
				None
			}
		},
		Err(err) => {
			error!("{:?}", err);
			None
		}
	}
}

pub async fn insert_one(
	database: &Database,
	data: &Status,
	user: Option<UserReference>,
) -> Result<InsertOneResult, Error> {
	database
		.collection::<Document>("trading_data_status")
		.insert_one(timestamp(data, user, &true, None))
		.await
}

pub async fn replace_one(
	database: &Database,
	data: &Status,
	user: Option<UserReference>,
) -> Result<UpdateResult, Error> {
	database
		.collection::<Document>("trading_data_status")
		.replace_one(doc! {"_id": data.id}, timestamp(data, user, &true, None))
		.await
}
