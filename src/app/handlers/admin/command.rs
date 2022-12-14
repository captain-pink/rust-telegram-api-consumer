use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Admin commands")]
pub enum AdminCommands {
    #[command(parse_with = "split", description = "Check consumers list")]
    Consumers,
    #[command(parse_with = "split", description = "Request vehicles from server")]
    Vehicles { limit: String, offset: String },
}
