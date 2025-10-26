use crate::trading;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
	config.service(web::scope("/asset").configure(trading::asset::routes::config));
}
