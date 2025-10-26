use crate::site::services::{generate_rsa_key, send_message};
use actix_web::{web, HttpResponse};

pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(
			web::resource("/send-message")
				.route(web::post().to(send_message))
				.route(web::head().to(HttpResponse::MethodNotAllowed)),
		)
		.service(
			web::resource("/send-message/")
				.route(web::post().to(send_message))
				.route(web::head().to(HttpResponse::MethodNotAllowed)),
		)
		.service(
			web::resource("/generate-rsa-key")
				.route(web::post().to(generate_rsa_key))
				.route(web::head().to(HttpResponse::MethodNotAllowed)),
		)
		.service(
			web::resource("/generate-rsa-key/")
				.route(web::post().to(generate_rsa_key))
				.route(web::head().to(HttpResponse::MethodNotAllowed)),
		);
}
