use serde::{Deserialize, Serialize};
use crate::trading::data::enums::{Analysis, Timeframe};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PredictRequest {
	pub ticker: String,
	pub timeframe: Timeframe,
	pub token: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PredictResponse {
	pub prediction: Analysis,
	pub response: String,
	pub result: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TrainRequest {
	pub ticker: String,
	pub token: String,
}
