use crate::core::base::enums::Status;
use crate::integration::discord;
use crate::trading::asset::enums::Provider;
use crate::trading::asset::models::{Asset, AssetSync};
use crate::trading::asset::repositories::{create_index, find_by_sync_synchronized, insert_one};
use crate::trading::data::enums::Timeframe;
use crate::trading::data::repositories::create_index as create_data_index;
use crate::trading::data::services::sync;
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use dotenvy::from_filename;
use flexi_logger::filter::{LogLineFilter, LogLineWriter};
use flexi_logger::{detailed_format, DeferredNow, FileSpec, Logger, WriteMode};
use log::{error, Record};
use mongodb::Database;
use serenity::all::GatewayIntents;
use std::env;
use tokio::task;

struct LogFilter;

impl LogLineFilter for LogFilter {
	fn write(
		&self,
		now: &mut DeferredNow,
		record: &Record,
		writer: &dyn LogLineWriter,
	) -> std::io::Result<()> {
		let modules: Vec<&str> = vec!["serenity"];

		if modules
			.iter()
			.any(|module| record.module_path().unwrap_or_default().starts_with(module))
		{
			Ok(())
		} else {
			writer.write(now, record)
		}
	}
}

pub fn initialize_discord() {
	let discord_token: String = env::var("API_DISCORD_TOKEN").unwrap_or_default();
	let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

	task::spawn(async move {
		match serenity::Client::builder(discord_token, intents)
			.event_handler(discord::services::Handler)
			.await
		{
			Ok(mut client) => {
				match client.start().await {
					Ok(_) => {}
					Err(err) => error!("{:?}", err),
				};
			}
			Err(err) => error!("{:?}", err),
		};
	});
}

pub fn initialize_env() {
	let _ = from_filename(if cfg!(debug_assertions) {
		".env.development"
	} else {
		".env.production"
	});
}

pub fn initialize_log() {
	Logger::try_with_str("info")
		.unwrap()
		.filter(Box::new(LogFilter))
		.write_mode(WriteMode::BufferAndFlush)
		.format(detailed_format)
		.log_to_file(
			FileSpec::default()
				.directory("logs")
				.basename("application"),
		)
		.rotate(
			flexi_logger::Criterion::Age(flexi_logger::Age::Day),
			flexi_logger::Naming::Numbers,
			flexi_logger::Cleanup::KeepLogFiles(7),
		)
		.start()
		.unwrap();
}

pub async fn initialize_task(database: &Database) {
	create_index(database).await;

	let last: DateTime<Utc> = Utc::now() - Duration::days(3650);
	let _ = insert_one(
		database,
		&Asset {
			provider: Provider::EODHD,
			status: Status::Active,
			sync: AssetSync {
				last: Utc.with_ymd_and_hms(last.year(), 1, 1, 0, 0, 0).unwrap(),
				synchronized: false,
				symbol: "C:XAUUSD".to_string(),
			},
			ticker: "XAUUSD".to_string(),
			..Default::default()
		},
		None,
	)
		.await;

	let assets: Option<Vec<Asset>> = find_by_sync_synchronized(database, &false).await;

	if !assets.is_none() {
		for asset in assets.unwrap_or_default() {
			create_data_index(database, &asset.ticker).await;

			sync(database, asset, Timeframe::OneMinute);
		}
	}
}
