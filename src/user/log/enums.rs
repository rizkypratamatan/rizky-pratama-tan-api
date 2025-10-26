use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Default, Deserialize, EnumIter, Eq, PartialEq, Serialize)]
pub enum Platform {
	#[default]
	Desktop,
	Mobile,
}

#[derive(Clone, Debug, Default, Deserialize, EnumIter, Eq, PartialEq, Serialize)]
pub enum Type {
	Login,
	#[default]
	Logout,
}
