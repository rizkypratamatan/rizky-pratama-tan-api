use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};
use std::env;
use tera::{Context, Tera};

pub async fn index(template: web::Data<Tera>) -> impl Responder {
	let mut context: Context = Context::new();

	let application_name: String = env::var("APPLICATION_NAME").unwrap_or_default();
	context.insert("title", &application_name);

	let body: String = template.render("index.html", &context).unwrap();

	/*get_data(IntradayRequest {
		symbol: "XAUUSD.FOREX".to_string(),
		interval: Some("1m".to_string()),
		from: None,
		to: None,
	})
		.await;*/

	HttpResponse::Ok().content_type(ContentType::html()).body(body)
}
