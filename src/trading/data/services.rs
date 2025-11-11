use crate::core::base::models::BaseResponse;
use crate::integration::massive::enums::Timespan;
use crate::integration::massive::models::{AggregateTickerRequest, AggregateTickerResponse};
use crate::integration::massive::services::aggregate_ticker;
use crate::trading::asset::models::Asset;
use crate::trading::asset::repositories::{find_one_by_id, replace_one as replace_one_asset};
use crate::trading::data::enums::{Analysis, Timeframe};
use crate::trading::data::models::{Data, DataPrice, DataSyncArc};
use crate::trading::data::repositories::{find, find_one, insert_one, replace_one};
use bson::doc;
use chrono::{DateTime, Duration, Utc};
use log::info;
use mongodb::Database;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use tokio_cron_scheduler::job::JobLocked;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn analyze(database: &Database, asset: &Asset) -> BaseResponse {
	let mut response: BaseResponse = BaseResponse::default();

	let raw_data_vec: Option<Vec<Data>> = find(database, &asset.ticker).await;

	if !raw_data_vec.is_none() {
		let data_vec: Vec<Data> = raw_data_vec.unwrap_or_default();

		for (index, data) in data_vec.iter().enumerate() {
			if index > 0 && index < data_vec.len() - 1 {
				let mut new_data: Data = data.clone();
				new_data.analysis.classification = Analysis::Sideways;

				if new_data.change.amount > 2f64 && (new_data.price.high - new_data.price.close) < 0.25 {
					// Bullish pattern
					if data_vec[index - 1].change.amount > 0f64 {
						new_data.analysis.classification = Analysis::Bullish;

						if data_vec[index + 1].change.amount >= 5f64 {
							new_data.analysis.target = 50f64;
						} else if data_vec[index + 1].change.amount >= 4f64 {
							new_data.analysis.target = 40f64;
						} else if data_vec[index + 1].change.amount >= 3f64 {
							new_data.analysis.target = 30f64;
						} else if data_vec[index + 1].change.amount >= 2f64 {
							new_data.analysis.target = 20f64;
						} else if data_vec[index + 1].change.amount >= 1f64 {
							new_data.analysis.target = 10f64;
						}
					}
				} else if new_data.change.amount < -2f64 && (new_data.price.close - new_data.price.low) < 0.25 {
					// Bearish pattern
					if data_vec[index - 1].change.amount < 0f64 {
						new_data.analysis.classification = Analysis::Bearish;

						if data_vec[index + 1].change.amount <= -5f64 {
							new_data.analysis.target = 50f64;
						} else if data_vec[index + 1].change.amount <= -4f64 {
							new_data.analysis.target = 40f64;
						} else if data_vec[index + 1].change.amount <= -3f64 {
							new_data.analysis.target = 30f64;
						} else if data_vec[index + 1].change.amount <= -2f64 {
							new_data.analysis.target = 20f64;
						} else if data_vec[index + 1].change.amount <= -1f64 {
							new_data.analysis.target = 10f64;
						}
					}
				}

				if new_data.analysis.classification != Analysis::Sideways {
					let _ = replace_one(database, &asset.ticker, &new_data, None).await;

					response.response = "Trading data analyzed successfully.".to_string();
					response.result = true;
				}
			}
		}
	}

	response
}

pub async fn copy(
	database: &Database,
	asset: &Asset,
	timeframe: &Timeframe,
	from: Option<DateTime<Utc>>,
	to: Option<DateTime<Utc>>,
) -> BaseResponse {
	let mut result: BaseResponse = BaseResponse::default();

	let mut params: AggregateTickerRequest = AggregateTickerRequest {
		ticker: asset.sync.symbol.clone(),
		multiplier: get_multiplier(timeframe),
		timespan: get_timespan(timeframe),
		..Default::default()
	};

	if !from.is_none() {
		params.from = from.unwrap_or_default().format("%Y-%m-%d").to_string();
	}

	if !to.is_none() {
		params.to = to.unwrap_or_default().format("%Y-%m-%d").to_string();
	}

	let response: Option<AggregateTickerResponse> = aggregate_ticker(&params).await;

	if !response.is_none() {
		for result in response.unwrap_or_default().results {
			let mut data: Data = Data {
				datetime: result.timestamp,
				price: DataPrice {
					close: result.close,
					high: result.high,
					low: result.low,
					open: result.open,
				},
				timeframe: timeframe.clone(),
				volume: result.volume,
				..Default::default()
			};
			data.change.amount = data.price.close - data.price.open;
			data.change.percentage = data.change.amount / data.price.open * 100f64;

			let _ = insert_one(database, &asset.ticker.clone(), &data, None).await;
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
		let mut new_asset: Asset = asset.clone();
		new_asset.sync.last = to;
		let _ = replace_one_asset(database, &new_asset, None).await;
	}

	response
}

pub fn get_interval(timeframe: &Timeframe) -> String {
	match timeframe {
		Timeframe::OneMinute => "1m".to_string(),
		Timeframe::FiveMinutes => "5m".to_string(),
		Timeframe::FifteenMinutes => "15m".to_string(),
		Timeframe::ThirtyMinutes => "30m".to_string(),
		Timeframe::OneHour => "1h".to_string(),
		Timeframe::FourHours => "4h".to_string(),
		Timeframe::OneDay => "1d".to_string(),
		Timeframe::OneMonth => "1M".to_string(),
	}
}

pub fn get_multiplier(timeframe: &Timeframe) -> i64 {
	match timeframe {
		Timeframe::OneMinute => 1,
		Timeframe::FiveMinutes => 5,
		Timeframe::FifteenMinutes => 15,
		Timeframe::ThirtyMinutes => 30,
		Timeframe::OneHour => 1,
		Timeframe::FourHours => 4,
		Timeframe::OneDay => 1,
		Timeframe::OneMonth => 1,
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

pub fn get_timespan(timeframe: &Timeframe) -> Timespan {
	match timeframe {
		Timeframe::OneMinute => Timespan::Minute,
		Timeframe::FiveMinutes => Timespan::Minute,
		Timeframe::FifteenMinutes => Timespan::Minute,
		Timeframe::ThirtyMinutes => Timespan::Minute,
		Timeframe::OneHour => Timespan::Hour,
		Timeframe::FourHours => Timespan::Hour,
		Timeframe::OneDay => Timespan::Day,
		Timeframe::OneMonth => Timespan::Month,
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
		let scheduler: Arc<Mutex<JobScheduler>> =
			Arc::new(Mutex::new(JobScheduler::new().await.unwrap()));
		let scheduler_arc: Arc<Mutex<JobScheduler>> = Arc::clone(&scheduler);
		let data_task: Arc<DataSyncArc> = Arc::clone(&data_task);

		let job: JobLocked = Job::new_async("0 */1 * * * *", move |_uuid, _l| {
			let scheduler_arc: Arc<Mutex<JobScheduler>> = Arc::clone(&scheduler_arc);
			let data_task: Arc<DataSyncArc> = Arc::clone(&data_task);

			Box::pin(async move {
				let data_task: Arc<DataSyncArc> = Arc::clone(&data_task);

				let asset: Option<Asset> =
					find_one_by_id(&data_task.database, &data_task.asset.id).await;

				if !asset.is_none() {
					let asset: Asset = asset.unwrap_or_default();

					let from: DateTime<Utc> = asset.sync.last;
					let to: DateTime<Utc> = from + Duration::days(3);
					let response: BaseResponse = copy(
						&data_task.database,
						&asset,
						&data_task.timeframe,
						Some(from),
						Some(to),
					)
						.await;

					info!("{}", response.response);

					update_last(&data_task.database, &asset, &to).await;

					if to > Utc::now() {
						analyze(&data_task.database, &asset).await;

						let _ = scheduler_arc.lock().await.shutdown().await;
					}
				}
			})
		})
			.unwrap();

		scheduler.lock().await.add(job).await.unwrap();
		scheduler.lock().await.start().await.unwrap();
	});

	response
}

async fn update_last(database: &Database, asset: &Asset, last: &DateTime<Utc>) {
	let mut new_asset: Asset = asset.clone();
	new_asset.sync.last = last.clone();

	if last.clone() > Utc::now() {
		let data: Option<Data> =
			find_one(database, &new_asset.ticker, doc! {"created.timestamp": -1}).await;

		if !data.is_none() {
			new_asset.sync.last = data.unwrap_or_default().datetime;
			new_asset.sync.synchronized = true;
		}
	}

	let _ = replace_one_asset(database, &new_asset, None).await;
}
