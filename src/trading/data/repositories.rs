use crate::core::database::models::UserReference;
use crate::core::database::services::timestamp;
use crate::trading::data::models::Data;
use bson::{doc, Document};
use log::error;
use mongodb::error::Error;
use mongodb::options::IndexOptions;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{Database, IndexModel};
use serenity::futures::TryStreamExt;

pub async fn create_index(database: &Database, collection: &str) {
	let index_model = IndexModel::builder()
		.keys(doc! {"datetime": 1, "timeframe": 1})
		.options(
			IndexOptions::builder()
				.unique(true)
				.name(Some("datetime_timeframe_unique_".to_string()))
				.build(),
		)
		.build();
	match database
		.collection::<Data>(&("trading_data_".to_string() + collection))
		.create_index(index_model)
		.await
	{
		Ok(_) => {}
		Err(err) => {
			error!("{:?}", err)
		}
	}
}

pub async fn find(database: &Database, collection: &str) -> Option<Vec<Data>> {
	match database
		.collection::<Data>(&("trading_data_".to_string() + collection))
		.find(doc! {})
		.sort(doc! {"created.timestamp": 1})
		.await
	{
		Ok(cursor) => match cursor.try_collect().await {
			Ok(data) => Some(data),
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

pub async fn find_one(database: &Database, collection: &str, sorts: Document) -> Option<Data> {
	database
		.collection::<Data>(&("trading_data_".to_string() + collection))
		.find_one(doc! {})
		.sort(sorts)
		.await
		.unwrap_or_else(|err| {
			error!("{:?}", err);
			None
		})
}

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

pub async fn replace_one(
	database: &Database,
	collection: &str,
	data: &Data,
	user: Option<UserReference>,
) -> Result<UpdateResult, Error> {
	database
		.collection::<Document>(&("trading_data_".to_string() + collection))
		.replace_one(doc! {"_id": data.id}, timestamp(data, user, &true, None))
		.await
}
