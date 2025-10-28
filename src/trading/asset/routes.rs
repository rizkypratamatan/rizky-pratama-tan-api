use crate::trading::asset;
use actix_web::{web, HttpResponse};

pub fn config(config: &mut web::ServiceConfig) {
	config.service(
		web::resource("/").route(web::post().to(asset::services::create)).route(web::head().to(HttpResponse::MethodNotAllowed)),
	);
}
