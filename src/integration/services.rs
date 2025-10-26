use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn initialize_params<T>(params: &T) -> String
							where
								T: Serialize + for<'de> Deserialize<'de>,
{
	let mut url: String = "".to_string();

	let json: Value = serde_json::to_value(&params).unwrap();

	if let Value::Object(map) = json {
		for (key, value) in map {
			if !value.is_null() {
				if value.is_string() {
					url += &format!("&{}={}", key, value.as_str().unwrap_or_default());
				} else {
					url += &format!("&{}={}", key, value);
				}
			}
		}
	}

	url
}
