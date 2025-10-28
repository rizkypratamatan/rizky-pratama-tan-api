use crate::user::log::models::UserLog;
use bson::doc;
use log::error;
use mongodb::Database;

pub async fn find_one_by_authentication(
	database: &Database,
	authentication: &str,
) -> Option<UserLog> {
	database.collection::<UserLog>("user_log").find_one(doc! {"authentication": authentication}).await.unwrap_or_else(|error| {
		error!("{:?}", error);

		None
	})
}
