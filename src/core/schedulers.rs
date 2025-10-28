use crate::core::base::models::BaseResponse;
use crate::trading::asset::models::Asset;
use crate::trading::asset::repositories::find_by_sync_synchronized;
use crate::trading::data::enums::Timeframe;
use crate::trading::data::services::get_realtime;
use log::info;
use mongodb::Database;
use std::sync::Arc;
use tokio::task;
use tokio_cron_scheduler::{Job, JobScheduler};

pub fn scheduler_eodhd_intraday_1m(database: &Database) {
	let database_clone: Database = database.clone();

	task::spawn(async move {
		let scheduler: JobScheduler = JobScheduler::new().await.unwrap();
		let database_clone: Arc<Database> = Arc::new(database_clone);

		scheduler.add(
			Job::new_async("0 */1 * * * *", move |_uuid, _l| {
				let database_arc_clone: Arc<Database> = Arc::clone(&database_clone);

				Box::pin(async move {
					let assets: Option<Vec<Asset>> = find_by_sync_synchronized(&database_arc_clone, &true).await;

					if !assets.is_none() && !assets.is_some() {
						for asset in assets.unwrap_or_default() {
							let response: BaseResponse = get_realtime(
								&database_arc_clone,
								&asset,
								&Timeframe::OneMinute,
							).await;

							info!("{}", response.response);
						}

						info!("Trading data get_realtime executed");
					}
				})
			}).unwrap(),
		).await.unwrap();

		scheduler.start().await.unwrap();
	});
}
