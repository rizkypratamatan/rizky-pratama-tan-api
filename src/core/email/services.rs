use crate::core::base::models::BaseResponse;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::env;

pub async fn send(from: &str, to: &str, subject: &str, body: &str) {
	let mut email: BaseResponse = BaseResponse::default();

	let smtp_server: String = env::var("EMAIL_SMTP_SERVER").unwrap_or_default();
	let smtp_port: u16 = env::var("EMAIL_SMTP_PORT")
		.unwrap_or_default()
		.parse()
		.unwrap_or_default();
	let user: String = env::var("EMAIL_USER").unwrap_or_default();
	let password: String = env::var("EMAIL_PASSWORD").unwrap_or_default();

	let message: Message = Message::builder()
		.from(from.parse::<Mailbox>().unwrap())
		.to(to.parse::<Mailbox>().unwrap())
		.subject(subject)
		.body(String::from(body))
		.unwrap();
	let mailer: AsyncSmtpTransport<Tokio1Executor> =
		AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)
			.unwrap()
			.port(smtp_port)
			.tls(Tls::Required(
				TlsParameters::new(smtp_server.to_string()).unwrap(),
			))
			.credentials(Credentials::new(user.clone(), password))
			.build();

	match mailer.send(message).await {
		Ok(mailer_response) => {
			email.response = format!("{:?}", mailer_response);
			email.result = true;
		}
		Err(err) => email.response = format!("{:?}", err),
	}
}
