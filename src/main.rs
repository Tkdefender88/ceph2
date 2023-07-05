mod commands;

use anyhow::anyhow;
use poise::serenity_prelude as serenity;
use shuttle_secrets::SecretStore;

use std::{collections::HashMap, sync::Mutex};

pub struct Data {
    votes: Mutex<HashMap<String, u32>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[shuttle_runtime::main]
async fn poise(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_poise::ShuttlePoise<Data, Error> {
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("DISCORD_TOKEN was not found").into());
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::age(),
                commands::help(),
                commands::vote(),
                commands::getvotes(),
            ],
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { 
                    votes: Mutex::new(HashMap::new()),
                })
            })
        }).build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

        Ok(framework.into())
}
