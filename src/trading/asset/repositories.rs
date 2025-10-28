use crate::core::database::models::UserReference;
use crate::core::database::services::timestamp;
use crate::trading::asset::models::Asset;
use bson::oid::ObjectId;
use bson::{doc, Document};
use log::error;
use mongodb::error::Error;
use mongodb::options::IndexOptions;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{Database, IndexModel};
use serenity::futures::TryStreamExt;

pub async fn create_index(database: &Database) {
	let index_model = IndexModel::builder()
		.keys(doc! {"name": 1})
		.options(
			IndexOptions::builder()
				.unique(true)
				.name(Some("name_unique_".to_string()))
				.build(),
		)
		.build();
	match database
		.collection::<Asset>("trading_asset")
		.create_index(index_model)
		.await
	{
		Ok(_) => {}
		Err(err) => {
			error!("{:?}", err)
		}
	}
}

pub async fn find_by_sync_synchronized(
	database: &Database,
	sync_synchronized: &bool,
) -> Option<Vec<Asset>> {
	match database
		.collection::<Asset>("trading_asset")
		.find(doc! {"sync.synchronized": sync_synchronized})
		.await
	{
		Ok(cursor) => match cursor.try_collect().await {
			Ok(asset) => Some(asset),
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

pub async fn find_one_by_ticker(database: &Database, ticker: &str) -> Option<Asset> {
	database
		.collection::<Asset>("trading_asset")
		.find_one(doc! {"ticker": ticker})
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

pub async fn replace_one(
	database: &Database,
	data: &Asset,
	user: Option<UserReference>,
) -> Result<UpdateResult, Error> {
	database
		.collection::<Document>("trading_asset")
		.replace_one(doc! {"_id": data.id}, timestamp(data, user, &true, None))
		.await
}

pub async fn log_insert_one(
	database: &Database,
	data: &Asset,
	user: Option<UserReference>,
) -> Result<InsertOneResult, Error> {
	database
		.collection::<Document>("trading_asset_log_data")
		.insert_one(timestamp(
			data,
			user,
			&true,
			Some("trading_asset_id".to_string()),
		))
		.await
}
