use chrono::{DateTime, Utc};
use log::{error, info};
use serenity::all::MessageId;
use serenity::async_trait;
use serenity::builder::GetMessages;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::Timestamp;
use serenity::prelude::{Context, EventHandler};
use std::env;
use std::sync::{OnceLock, RwLock};
use std::time::Duration;
use tokio::task;
use tokio::time::{interval, sleep, Interval};

pub struct Discord {
	pub channel_id: RwLock<Option<u64>>,
	pub message: RwLock<Option<String>>,
}

impl Discord {
	fn new() -> Self {
		Discord {
			channel_id: RwLock::new(None),
			message: RwLock::new(None),
		}
	}

	pub fn instance() -> &'static Self {
		static INSTANCE: OnceLock<Discord> = OnceLock::new();
		INSTANCE.get_or_init(Discord::new)
	}

	pub fn get_channel_id(&self) -> Option<u64> {
		self.channel_id.write().ok()?.clone()
	}

	pub fn get_message(&self) -> Option<String> {
		self.message.write().ok()?.take()
	}

	pub fn set_channel_id(&self, channel_id: &u64) {
		match self.channel_id.write() {
			Ok(mut _u64) => {
				*_u64 = Some(*channel_id);
			}
			Err(err) => error!("{}", err),
		}
	}

	pub fn set_message(&self, message: &str) {
		match self.message.write() {
			Ok(mut string) => {
				*string = Some(message.to_string());
			}
			Err(err) => error!("{}", err),
		}
	}
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, context: Context, message: Message) {
		if message.content == "!delete" {
			match message.channel_id.say(&context.http, "Pong!").await {
				Ok(_) => {}
				Err(err) => error!("{:?}", err),
			}
		}
	}

	async fn ready(&self, context: Context, ready: Ready) {
		info!("{} discord bot is ready.", ready.user.name.clone());

		let discord: &Discord = Discord::instance();

		let channel_id: u64 = env::var("API_DISCORD_CHANNEL_ID").unwrap_or_default().parse::<u64>().unwrap_or_default();
		discord.set_channel_id(&channel_id);

		let context_send: Context = context.clone();

		task::spawn(async move {
			loop {
				let message: String = discord.get_message().unwrap_or_default().clone();

				if !message.is_empty() {
					let discord_channel_id: ChannelId = ChannelId::from(discord.get_channel_id().unwrap_or_default());

					match discord_channel_id.say(&context_send.http, message).await {
						Ok(_) => {}
						Err(err) => error!("{:?}", err),
					}
				}

				sleep(Duration::from_millis(500)).await;
			}
		});

		let context_delete: Context = context.clone();

		task::spawn(async move {
			let mut delay: Interval = interval(Duration::from_secs(30 * 24 * 60 * 60));

			if channel_id == discord.get_channel_id().unwrap_or_default() {
				delay = interval(Duration::from_secs(24 * 60 * 60));
			}

			loop {
				delay.tick().await;

				let discord_channel_id: ChannelId = ChannelId::from(discord.get_channel_id().unwrap_or_default());
				delete_message(&context_delete, &discord_channel_id).await;
			}
		});
	}
}

async fn delete_message(context: &Context, channel_id: &ChannelId) {
	let limit: DateTime<Utc> = Utc::now() - chrono::Duration::hours(1);

	match channel_id.messages(context, GetMessages::new().limit(50)).await {
		Ok(messages) => {
			let old_messages: Vec<MessageId> = messages.iter().filter(|message| {
				message.timestamp < Timestamp::from_unix_timestamp(limit.timestamp()).unwrap()
			}).map(|message| message.id).collect();

			if !old_messages.is_empty() {
				match channel_id.delete_messages(context, old_messages).await {
					Ok(_) => {}
					Err(err) => error!("{:?}", err),
				}
			}
		}
		Err(err) => error!("{:?}", err),
	}
}

pub async fn send_visitor(message: &str) {
	let discord: &Discord = Discord::instance();
	let channel_id: u64 = env::var("API_DISCORD_CHANNEL_ID_VISITOR_MESSAGE").unwrap_or_default().parse::<u64>().unwrap_or_default();
	discord.set_channel_id(&channel_id);
	discord.set_message(message);
}
