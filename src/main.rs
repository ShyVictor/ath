mod commands;

use serenity::all::CreateInteractionResponse;
use serenity::all::CreateInteractionResponseMessage;
use serenity::Client;
use serenity::all::EventHandler;
use std::fs;
use serenity::all::Context;
use serenity::async_trait;
use serenity::model::application::{Command, Interaction};
use serenity::model::channel::Message;
use serenity::model::gateway::{GatewayIntents, Ready};

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Escutando: {}", ready.user.name);
        Command::create_global_command(&ctx.http, commands::bunny::register()).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("{command:#?}");
            let content = match command.data.name.as_str() {
                "bunny" => Some(commands::bunny::run(&command.data.options())),
                _ => Some("Not implemented".to_string())
            };
            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

fn token() -> Result<String, std::io::Error> {
    let file = fs::read_to_string("src/athy.config")?;
    Ok(file.trim().to_string())
}
#[tokio::main]
async fn main() {
    let token:String = token().expect("Erro no arquivo de configuração");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;
    let mut discord_client = Client::builder(&token, intents).event_handler(Handler).
        await.expect("Erro ao criar o cliente");

    if let Err(why) = discord_client.start().await {
        println!("Client error: {why:?}");
    }
}
