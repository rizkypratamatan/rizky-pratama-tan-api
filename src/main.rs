use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use dotenvy::from_filename;
use flexi_logger::filter::{LogLineFilter, LogLineWriter};
use flexi_logger::{detailed_format, DeferredNow, FileSpec, Logger, WriteMode};
use log::{error, info, Record};
use mongodb::{Client, Database};
use rizky_pratama_tan_api::core::base::models::BaseResponse;
use rizky_pratama_tan_api::integration::discord;
use rizky_pratama_tan_api::site;
use rizky_pratama_tan_api::trading::data::services::{get_realtime, sync};
use rizky_pratama_tan_api::trading::data::status::enums::Status as DataStatus;
use rizky_pratama_tan_api::trading::data::status::models::{Status, StatusAsset};
use rizky_pratama_tan_api::trading::data::status::repositories::find_by_status;
use rizky_pratama_tan_api::trading::data::status::services::get_asset;
use rizky_pratama_tan_api::{core, trading};
use serenity::all::GatewayIntents;
use std::env;
use std::sync::Arc;
use tera::Tera;
use tokio::task;
use tokio_cron_scheduler::{Job, JobScheduler};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let _ = from_filename(if cfg!(debug_assertions) {
		".env.development"
	} else {
		".env.production"
	});

	let mongodb_client: Option<Client> =
		match Client::with_uri_str(&env::var("DATABASE_CONNECTION_STRING").unwrap_or_default())
			.await
		{
			Ok(client) => Some(client),
			Err(err) => {
				error!("{:?}", err);

				None
			}
		};

	let database: Arc<Database> = Arc::new(
		mongodb_client
			.unwrap()
			.database(&env::var("DATABASE_NAME").unwrap_or_default()),
	);
	let database_arc: Arc<Database> = Arc::clone(&database);

	initial(&database_arc.clone());

	let server_address: String = env::var("SERVER_ADDRESS").unwrap_or_default();
	let tera: Tera = Tera::new("resources/templates/**/*").unwrap();

	HttpServer::new(move || {
		let database_clone: Arc<Database> = Arc::clone(&database_arc);

		App::new()
			.wrap(
				Cors::default()
					.allow_any_origin()
					.allow_any_method()
					.allow_any_header()
					.max_age(3600),
			)
			.wrap(middleware::Logger::default())
			.app_data(web::Data::new(tera.clone()))
			.app_data(web::Data::from(database_clone))
			.service(Files::new("/resources", "./resources").show_files_listing())
			.service(web::scope("/site").configure(site::routes::config))
			.service(web::scope("/trading").configure(trading::routes::config))
			.route("/", web::get().to(core::base::services::index))
	})
		.bind(server_address)?
		.run()
		.await
}

fn initial(database: &Database) {
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

	let database_clone: Database = database.clone();

	task::spawn(async move {
		let scheduler: JobScheduler = JobScheduler::new().await.unwrap();
		let database_clone: Arc<Database> = Arc::new(database_clone);

		scheduler
			.add(
				Job::new_async("0 */1 * * * *", move |_uuid, _l| {
					let database_arc_clone: Arc<Database> = Arc::clone(&database_clone);

					Box::pin(async move {
						let data_statuses: Option<Vec<Status>> =
							find_by_status(&database_arc_clone, &DataStatus::Synchronized).await;

						if !data_statuses.is_none() && !data_statuses.is_some() {
							for data_status in data_statuses.unwrap_or_default() {
								let response: BaseResponse = get_realtime(
									&database_arc_clone,
									&get_asset(&data_status.asset),
									"1m".to_string(),
								)
									.await;

								info!("{}", response.response);
							}

							info!("Trading data get_realtime executed");
						}
					})
				})
					.unwrap(),
			)
			.await
			.unwrap();

		scheduler.start().await.unwrap();
	});

	sync(
		database,
		StatusAsset {
			asset: "XAUUSD".to_string(),
			suffix: Some("FOREX".to_string()),
		},
		"1m".to_string(),
	);
}
