use crate::core::database::models::{Timestamp, UserReference};
use bson::{to_document, Bson, Document};
use chrono::Utc;
use mongodb::error::Error;
use serde::{Deserialize, Serialize};

pub fn error_message(err: Error, unique_key: &str) -> String {
	let error: String = format!("{:?}", err.kind.as_ref());

	if error.contains("code: 11000") && error.contains(unique_key) {
		(unique_key[0..1].to_uppercase() + &unique_key[1..] + " already exists.").to_string()
	} else {
		"Internal server error.".to_string()
	}
}

pub fn timestamp<T>(
	data: &T,
	user: Option<UserReference>,
	create: &bool,
	log_key: Option<String>,
) -> Document where
		T: Serialize + for<'de> Deserialize<'de>,
{
	let mut document: Document = to_document(data).unwrap_or_default();

	let mut user_reference: UserReference = UserReference {
		username: "System".to_string(),
		..Default::default()
	};

	if !user.is_none() {
		user_reference = user.unwrap_or_default();
	}

	let timestamp: Document = to_document(&Timestamp {
		timestamp: Utc::now(),
		user: user_reference,
	}).unwrap_or_default();

	if document.get("created").and_then(Bson::as_document).unwrap().get("timestamp").is_some()
	{
		document.insert("created", timestamp.clone());
	}

	document.insert("modified", timestamp);

	if !log_key.is_none() {
		document.insert(
			log_key.unwrap_or_default(),
			document.get("_id").unwrap().clone(),
		);
	}

	if create.clone() {
		document.remove("_id");
	}

	document
}
