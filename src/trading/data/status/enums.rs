use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Default, Deserialize, EnumIter, Eq, PartialEq, Serialize)]
pub enum Status {
	Pending,
	Synchronized,
	#[default]
	Unsynchronized,
}
