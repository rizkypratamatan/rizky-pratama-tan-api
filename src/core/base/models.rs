use crate::core::authentication::enums::Access;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BaseResponse {
	pub response: String,
	pub result: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Client {
	pub key: String,
	pub ip: Vec<String>,
	pub security: ClientSecurity,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClientSecurity {
	pub access: Access,
	pub timestamp: bool,
}
