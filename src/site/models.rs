use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GenerateRsaKeyRequest {
	pub size: u16,
	pub token: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SiteSendMessageRequest {
	pub email: String,
	pub message: String,
	pub name: String,
	pub token: String,
}
