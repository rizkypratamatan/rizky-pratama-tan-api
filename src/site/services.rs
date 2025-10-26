use crate::core::authentication::enums::Access;
use crate::core::authentication::services::authenticate;
use crate::core::base::models::BaseResponse;
use crate::integration::discord::services::send_visitor;
use crate::site::models::{GenerateRsaKeyRequest, SiteSendMessageRequest};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::env;

pub async fn generate_rsa_key(
	request: HttpRequest,
	data: web::Json<GenerateRsaKeyRequest>,
) -> impl Responder {
	let mut response: BaseResponse = BaseResponse::default();

	let authentication: BaseResponse = authenticate(request, &Access::Public, &data.clone());

	if authentication.result {
		let valid_size: Vec<u16> = vec![1024, 2048, 4096];

		if valid_size.contains(&data.size) {
			response.response = "RSA Key Generated".to_string();
			response.result = true;
		}
	} else {
		return HttpResponse::Unauthorized().json(authentication);
	}

	HttpResponse::Ok().json(response)
}

pub async fn send_message(
	request: HttpRequest,
	data: web::Json<SiteSendMessageRequest>,
) -> impl Responder {
	let mut response: BaseResponse = BaseResponse::default();

	let authentication: BaseResponse = authenticate(request, &Access::Public, &data.clone());

	if authentication.result {
		let application_name: String = env::var("APPLICATION_NAME").unwrap_or_default();
		let body: String = format!(
			"Sender Name : {}\nSender Email : {}\n\n\n{}",
			data.name.clone(),
			data.email.clone(),
			data.message.clone()
		);
		send_visitor(&format!("[{}]\n\n\n{}", application_name, body)).await;

		response.response = "Message sent successfully".to_string();
		response.result = true;
	} else {
		return HttpResponse::Unauthorized().json(authentication);
	}

	HttpResponse::Ok().json(response)
}
