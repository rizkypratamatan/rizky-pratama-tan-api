use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use log::error;
use mongodb::{Client, Database};
use rizky_pratama_tan_api::core::schedulers::scheduler_eodhd_intraday_1m;
use rizky_pratama_tan_api::core::services::{initialize_discord, initialize_env, initialize_log, initialize_task};
use rizky_pratama_tan_api::site;
use rizky_pratama_tan_api::{core, trading};
use std::env;
use std::sync::Arc;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	initialize_env();
	initialize_log();
	initialize_discord();

	let mongodb_client: Option<Client> = match Client::with_uri_str(&env::var("DATABASE_CONNECTION_STRING").unwrap_or_default()).await {
		Ok(client) => Some(client),
		Err(err) => {
			error!("{:?}", err);

			None
		}
	};

	let database: Arc<Database> = Arc::new(
		mongodb_client.unwrap().database(&env::var("DATABASE_NAME").unwrap_or_default()),
	);
	let database_arc: Arc<Database> = Arc::clone(&database);

	scheduler_eodhd_intraday_1m(&database_arc.clone());

	initialize_task(&database_arc.clone()).await;

	let server_address: String = env::var("SERVER_ADDRESS").unwrap_or_default();
	let tera: Tera = Tera::new("resources/templates/**/*").unwrap();

	HttpServer::new(move || {
		let database_clone: Arc<Database> = Arc::clone(&database_arc);

		App::new().wrap(
			Cors::default().allow_any_origin().allow_any_method().allow_any_header().max_age(3600),
		).wrap(middleware::Logger::default()).app_data(web::Data::new(tera.clone())).app_data(web::Data::from(database_clone)).service(Files::new("/resources", "./resources").show_files_listing()).service(web::scope("/site").configure(site::routes::config)).service(web::scope("/trading").configure(trading::routes::config)).route("/", web::get().to(core::base::services::index))
	}).bind(server_address)?.run().await
}
