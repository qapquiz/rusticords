mod commands;

use std::env;

use serenity::{prelude::{GatewayIntents, EventHandler, Context}, Client, model::prelude::{Message, Ready, interaction::{Interaction, InteractionResponseType}}, async_trait};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.content == "!ping" {
            if let Err(why) = message.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // slash command call or button pressed
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            // doing the command
            let content = match command.data.name.as_str() {
                "create_verify_message" => commands::create_verify_message::run(&command.data.options),
                _ => "Not implemented :(".to_string(),
            };

            // send command response
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                }).await
            {
                println!("Cannot response to the slash command: {}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Get DISCORD_TOKEN
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set Intent
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new client
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
