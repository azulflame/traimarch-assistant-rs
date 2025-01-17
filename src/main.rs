mod models;
mod commands;
mod config;
pub mod common;

use serenity::all::{ClientBuilder, GuildId};
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use crate::commands::ping::ping;
use crate::commands::rewards::rewards;
use crate::commands::timestamp::timestamp;
use crate::common::Data;
use crate::config::{get_config_val, load_config, SecretType};

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    load_config(secret_store.clone());

    // get database connection


    // Get the discord token set in `Secrets.toml`
    let token = get_config_val(SecretType::Token);
    let guild_id = GuildId::new(get_config_val(SecretType::GuildId).parse::<u64>().unwrap());

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(),
            timestamp(),
            rewards()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
