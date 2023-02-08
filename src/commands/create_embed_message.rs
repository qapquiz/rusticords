use serenity::{model::prelude::{interaction::application_command::CommandDataOption, command::CommandOptionType}, builder::CreateApplicationCommand};

pub fn run(_options: &Vec<CommandDataOption>) -> String {
    String::from("Run create_embed_message success")
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("create_embed_message")
        .description("A create_embed_message command to create embed message that will interact with user.")
        .create_option(|option| {
            option
                .name("type")
                .description("a collections of types to create embed")
                .kind(CommandOptionType::String)
                .add_string_choice("verify", "verify")
                .add_string_choice("react role", "react_role")
                .required(true)
        })
}
