use crate::trading::data::status::models::StatusAsset;

pub fn get_asset(symbol: &str) -> StatusAsset {
	let mut asset: StatusAsset = StatusAsset::default();

	let symbols: Vec<&str> = symbol.split(".").collect::<Vec<&str>>();
	asset.asset = symbols[0].to_string();

	if symbols.len() > 1 {
		asset.suffix = Some(symbols[1].to_string());
	}

	asset
}

pub fn get_symbol(asset: &StatusAsset) -> String {
	let mut symbol: String = asset.asset.to_string();

	if !asset.suffix.is_none() {
		symbol += &format!(".{}", asset.suffix.clone().unwrap_or_default());
	}

	symbol
}
