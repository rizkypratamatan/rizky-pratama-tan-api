use crate::core::base::models::BaseResponse;
use crate::integration::eodhd::models::{IntradayRequest, IntradayResponse};
use crate::integration::eodhd::services::intraday;
use crate::trading::data::enums::Timeframe;
use crate::trading::data::models::Data;
use crate::trading::data::repositories::insert_one;
use crate::trading::data::status::enums::Status as DataStatus;
use crate::trading::data::status::models::{Status, StatusAsset};
use crate::trading::data::status::repositories::insert_one as insert_one_data_status;
use crate::trading::data::status::repositories::{find_one_by_asset, replace_one};
use crate::trading::data::status::services::get_symbol;
use chrono::{DateTime, Duration, TimeZone, Utc};
use log::info;
use mongodb::Database;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use tokio_cron_scheduler::job::JobLocked;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn copy(database: &Database, params: IntradayRequest) -> BaseResponse {
	let mut result: BaseResponse = BaseResponse::default();

	let response: Option<Vec<IntradayResponse>> = intraday(params.clone()).await;

	if !response.is_none() {
		for data in response.unwrap_or_default() {
			let _ = insert_one(
				database,
				&params.asset.clone(),
				&Data {
					close: data.close,
					datetime: data.datetime,
					high: data.high,
					low: data.low,
					open: data.open,
					timeframe: timeframe(&params.interval.clone().unwrap_or_default()),
					volume: data.volume,
					..Default::default()
				},
				None,
			)
				.await;
		}

		result.response = "Trading data has been copied successfully.".to_string();
		result.result = true;
	} else {
		result.response = "Failed to retrieve trading data.".to_string();
	}

	result
}

pub async fn get_realtime(database: &Database, asset: &StatusAsset, interval: String) -> BaseResponse {
	let to: DateTime<Utc> = Utc::now();
	let from: DateTime<Utc> = to - Duration::hours(6);
	let response: BaseResponse = copy(database, IntradayRequest {
		from: Some(from.timestamp()),
		interval: Some(interval),
		suffix: asset.suffix.clone(),
		asset: asset.asset.clone(),
		to: Some(to.timestamp()),
		..Default::default()
	}).await;

	let status: Option<Status> =
		find_one_by_asset(database, &get_symbol(&asset)).await;

	if !status.is_none() {
		let mut new_status: Status = status.unwrap();
		new_status.last = to;
		let _ = replace_one(database, &new_status, None).await;
	}

	response
}

async fn get_status(database: &Database, asset: &StatusAsset, interval: &str) -> Option<Status> {
	let status: Option<Status> =
		find_one_by_asset(database, &get_symbol(&asset)).await;

	if status.is_none() {
		let _ = insert_one_data_status(database, &Status {
			asset: get_symbol(&asset),
			interval: interval.to_string(),
			status: DataStatus::Unsynchronized,
			..Default::default()
		}, None).await;
	}

	status
}

pub fn sync(database: &Database, asset: StatusAsset, interval: String) -> BaseResponse {
	let response: BaseResponse = BaseResponse::default();

	let database_clone: Arc<Database> = Arc::new(database.clone());

	task::spawn(async move {
		let scheduler: Arc<Mutex<JobScheduler>> =
			Arc::new(Mutex::new(JobScheduler::new().await.unwrap()));
		let scheduler_arc: Arc<Mutex<JobScheduler>> = Arc::clone(&scheduler);
		let database_arc: Arc<Database> = Arc::clone(&database_clone);

		let job: JobLocked = Job::new_async("0 */5 * * * *", move |_uuid, _l| {
			let scheduler_arc_clone: Arc<Mutex<JobScheduler>> = Arc::clone(&scheduler_arc);
			let database_arc_clone: Arc<Database> = Arc::clone(&database_arc);
			let asset_clone: StatusAsset = asset.clone();
			let interval_clone: String = interval.clone();

			Box::pin(async move {
				let database_arc_box: Arc<Database> = Arc::clone(&database_arc_clone);
				let mut from: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
				let mut status: Option<Status> = get_status(&database_arc_box, &asset_clone, &interval_clone).await;

				if !status.is_none() {
					from = status.clone().unwrap_or_default().last;
				} else {
					status = find_one_by_asset(&database_arc_box, &get_symbol(&asset_clone)).await;
				}

				let to: DateTime<Utc> = from + Duration::days(30);
				let response: BaseResponse = copy(&database_arc_box, IntradayRequest {
					from: Some(from.timestamp()),
					interval: Some(interval_clone),
					suffix: asset_clone.suffix,
					asset: asset_clone.asset,
					to: Some(to.timestamp()),
					..Default::default()
				}).await;

				info!("{}", response.response);

				update_last(&database_arc_box, &mut status.clone().unwrap_or_default(), &to).await;

				if to > Utc::now() {
					update_last(&database_arc_box, &mut status.unwrap_or_default(), &Utc::now()).await;

					let _ = scheduler_arc_clone.lock().await.shutdown().await;
				}
			})
		})
			.unwrap();

		scheduler.lock().await.add(job).await.unwrap();
		scheduler.lock().await.start().await.unwrap();
	});

	response
}

pub fn timeframe(interval: &str) -> Timeframe {
	match interval {
		"1m" => Timeframe::OneMinute,
		"5m" => Timeframe::FiveMinutes,
		"15m" => Timeframe::FifteenMinutes,
		"30m" => Timeframe::ThirtyMinutes,
		"1h" => Timeframe::OneHour,
		"4h" => Timeframe::FourHours,
		"1d" => Timeframe::OneDay,
		"1M" => Timeframe::OneMonth,
		_ => Timeframe::default(),
	}
}

async fn update_last(database: &Database, status: &mut Status, last: &DateTime<Utc>) {
	status.last = *last;

	if *last > Utc::now() {
		status.status = DataStatus::Synchronized;
	}

	let _ = replace_one(database, status, None).await;
}
