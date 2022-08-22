use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Common commands")]
pub enum CommonCommands {
    #[command(parse_with = "split", description = "Start bot")]
    Start,
    #[command(parse_with = "split", description = "System command")]
    LetMeIn { username: String, token: String },
}
