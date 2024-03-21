use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Eu amo a Athyeine".to_string()
}
pub fn register() -> CreateCommand {
    CreateCommand::new("bunny").description("Default command")
}