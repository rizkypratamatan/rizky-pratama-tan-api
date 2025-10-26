use crate::core::authentication::enums::Access;
use crate::core::base::models::{BaseResponse, Client};
use crate::core::encryption::rsa::decrypt;
use actix_web::dev::ConnectionInfo;
use actix_web::HttpRequest;
use chrono::{DateTime, TimeDelta, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};
use std::cell::Ref;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn authenticate<T>(request: HttpRequest, access: &Access, data: &T) -> BaseResponse
					   where
						   T: Serialize + for<'de> Deserialize<'de>,
{
	let mut response: BaseResponse = BaseResponse::default();

	info!(
        "Request to {} Body: {}",
        &request.path().to_string(),
        serde_json::to_string(data).unwrap()
    );

	let client: Client = get_client_detail(&get_client_key(request.clone()));

	if !client.key.is_empty() {
		if access == &Access::Public && client.security.access == Access::Public {
			if authenticate_ip(request.clone(), client.clone()) {
				let tokens: Vec<String> = get_token(
					client.clone(),
					from_value(to_value(data).unwrap_or_default()).unwrap_or_default(),
				);

				if tokens.len() == 2 {
					if authenticate_path(request.clone(), tokens.clone()) {
						if authenticate_timestamp(client.clone(), tokens.clone()) {
							response.response = "Request authenticated".to_string();
							response.result = true;
						} else {
							response.response = "Token expired".to_string();
						}
					} else {
						response.response = "Invalid path".to_string();
					}
				} else {
					response.response = "Invalid token".to_string();
				}
			} else {
				response.response =
					format!("Unauthorized IP address {}", get_client_ip(request.clone()));
			}
		} else if client.security.access == Access::Private {
			if authenticate_ip(request.clone(), client.clone()) {
				response.response = "Request authenticated".to_string();
				response.result = true;
			} else {
				response.response =
					format!("Unauthorized IP address {}", get_client_ip(request.clone()));
			}
		} else {
			response.response = "Access denied".to_string();
		}
	} else {
		response.response = format!("Unauthorized PLD key {}", get_client_key(request.clone()));
	}

	response
}

fn authenticate_ip(request: HttpRequest, client: Client) -> bool {
	client
		.ip
		.contains(&get_client_ip(request.clone()).to_string())
		|| client.ip.contains(&"*".to_string())
}

fn authenticate_path(request: HttpRequest, tokens: Vec<String>) -> bool {
	tokens.get(0).unwrap() == &request.path().to_string()
}

fn authenticate_timestamp(client: Client, tokens: Vec<String>) -> bool {
	let current_timestamp: DateTime<Utc> = Utc::now();

	if client.security.timestamp {
		let difference: TimeDelta = DateTime::parse_from_rfc3339(tokens.get(1).unwrap())
			.ok()
			.unwrap_or_default()
			.to_utc()
			.signed_duration_since(current_timestamp);

		difference.num_seconds() <= 5
	} else {
		true
	}
}

fn get_client_ip(request: HttpRequest) -> String {
	let connection_info: Ref<ConnectionInfo> = request.connection_info();

	connection_info
		.realip_remote_addr()
		.unwrap_or_default()
		.to_string()
}

fn get_client_key(request: HttpRequest) -> String {
	request
		.headers()
		.get("pld-key")
		.unwrap()
		.to_str()
		.unwrap_or_default()
		.to_string()
}

fn get_client_detail(key: &str) -> Client {
	match read_to_string(&format!("clients/{}/client.pld", key)) {
		Ok(file) => serde_json::from_str(&file).unwrap_or_default(),
		Err(_) => Client::default(),
	}
}

fn get_token(client: Client, map: HashMap<String, Value>) -> Vec<String> {
	decrypt(
		&map.get("token").unwrap().as_str().unwrap(),
		&client.key.to_string(),
	)
		.split("~")
		.map(String::from)
		.collect()
}
