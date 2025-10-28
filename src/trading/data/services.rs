use crate::core::base::models::BaseResponse;
use crate::integration::eodhd::models::{IntradayRequest, IntradayResponse};
use crate::integration::eodhd::services::intraday;
use crate::trading::asset::models::Asset;
use crate::trading::asset::repositories::{find_one_by_id, replace_one};
use crate::trading::data::enums::Timeframe;
use crate::trading::data::models::{Data, DataSyncArc};
use crate::trading::data::repositories::insert_one;
use chrono::{DateTime, Duration, Utc};
use log::info;
use mongodb::Database;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use tokio_cron_scheduler::job::JobLocked;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn copy(
	database: &Database,
	asset: &Asset,
	timeframe: &Timeframe,
	from: Option<DateTime<Utc>>,
	to: Option<DateTime<Utc>>,
) -> BaseResponse {
	let mut result: BaseResponse = BaseResponse::default();

	let mut params: IntradayRequest = IntradayRequest {
		interval: Some(get_interval(timeframe).to_string()),
		symbol: asset.sync.symbol.clone(),
		..Default::default()
	};

	if !from.is_none() {
		params.from = Some(from.unwrap_or_default().timestamp());
	}

	if !to.is_none() {
		params.to = Some(to.unwrap_or_default().timestamp());
	}

	let response: Option<Vec<IntradayResponse>> = intraday(params.clone()).await;

	if !response.is_none() {
		for data in response.unwrap_or_default() {
			let _ = insert_one(
				database,
				&asset.ticker.clone(),
				&Data {
					close: data.close,
					datetime: data.datetime,
					high: data.high,
					low: data.low,
					open: data.open,
					timeframe: get_timeframe(&params.interval.clone().unwrap_or_default()),
					volume: data.volume,
					..Default::default()
				},
				None,
			).await;
		}

		result.response = "Trading data has been copied successfully.".to_string();
		result.result = true;
	} else {
		result.response = "Failed to retrieve trading data.".to_string();
	}

	result
}

pub async fn get_realtime(
	database: &Database,
	asset: &Asset,
	timeframe: &Timeframe,
) -> BaseResponse {
	let to: DateTime<Utc> = Utc::now();
	let from: DateTime<Utc> = to - Duration::hours(6);
	let response: BaseResponse = copy(database, asset, timeframe, Some(from), Some(to)).await;

	if response.result {
		let mut asset_new: Asset = asset.clone();
		asset_new.sync.last = to;
		let _ = replace_one(database, &asset_new, None).await;
	}

	response
}

pub fn get_interval(timeframe: &Timeframe) -> &str {
	match timeframe {
		Timeframe::OneMinute => "1m",
		Timeframe::FiveMinutes => "5m",
		Timeframe::FifteenMinutes => "15m",
		Timeframe::ThirtyMinutes => "30m",
		Timeframe::OneHour => "1h",
		Timeframe::FourHours => "4h",
		Timeframe::OneDay => "1d",
		Timeframe::OneMonth => "1M",
	}
}

pub fn get_timeframe(interval: &str) -> Timeframe {
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

pub fn sync(database: &Database, asset: Asset, timeframe: Timeframe) -> BaseResponse {
	let response: BaseResponse = BaseResponse::default();

	let data_task: Arc<DataSyncArc> = Arc::new(DataSyncArc {
		asset,
		database: database.clone(),
		timeframe,
	});

	task::spawn(async move {
		let scheduler: Arc<Mutex<JobScheduler>> = Arc::new(Mutex::new(JobScheduler::new().await.unwrap()));
		let scheduler_arc: Arc<Mutex<JobScheduler>> = Arc::clone(&scheduler);
		let data_task: Arc<DataSyncArc> = Arc::clone(&data_task);

		let job: JobLocked = Job::new_async("0 */1 * * * *", move |_uuid, _l| {
			let scheduler_arc: Arc<Mutex<JobScheduler>> = Arc::clone(&scheduler_arc);
			let data_task: Arc<DataSyncArc> = Arc::clone(&data_task);

			Box::pin(async move {
				let data_task: Arc<DataSyncArc> = Arc::clone(&data_task);

				let asset: Option<Asset> = find_one_by_id(&data_task.database, &data_task.asset.id).await;

				if !asset.is_none() {
					let asset: Asset = asset.unwrap_or_default();

					let from: DateTime<Utc> = asset.sync.last;
					let to: DateTime<Utc> = from + Duration::days(30);
					let response: BaseResponse = copy(
						&data_task.database,
						&asset,
						&data_task.timeframe,
						Some(from),
						Some(to),
					).await;

					info!("{}", response.response);

					update_last(&data_task.database, &asset, &to).await;

					if to > Utc::now() {
						update_last(&data_task.database, &asset, &Utc::now()).await;

						let _ = scheduler_arc.lock().await.shutdown().await;
					}
				}
			})
		}).unwrap();

		scheduler.lock().await.add(job).await.unwrap();
		scheduler.lock().await.start().await.unwrap();
	});

	response
}

async fn update_last(database: &Database, asset: &Asset, last: &DateTime<Utc>) {
	let mut asset_new: Asset = asset.clone();
	asset_new.sync.last = last.clone();

	if last.clone() > Utc::now() {
		asset_new.sync.last = Utc::now();
		asset_new.sync.synchronized = true;
	}
	println!("{:?}", asset_new);
	let _ = replace_one(database, &asset_new, None).await;
}
