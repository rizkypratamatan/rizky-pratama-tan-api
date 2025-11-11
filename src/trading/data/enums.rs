use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Default, Deserialize, EnumIter, Eq, PartialEq, Serialize)]
pub enum Analysis {
	Bearish,
	Bullish,
	#[default]
	Sideways
}

#[derive(Clone, Debug, Default, Deserialize, EnumIter, Eq, PartialEq, Serialize)]
pub enum Timeframe {
	FifteenMinutes,
	FiveMinutes,
	FourHours,
	#[default]
	OneDay,
	OneHour,
	OneMinute,
	OneMonth,
	ThirtyMinutes,
}
