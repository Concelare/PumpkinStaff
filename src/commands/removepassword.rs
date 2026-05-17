use crate::services::auth::AUTH_SERVICE;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::Server;
use std::str::FromStr;
use uuid::Uuid;

pub fn remove_command() -> Command {
    let names = ["removepassword".to_string()];
    let cmd = Command::new(&names, "Remove staff password");
    cmd.then(CommandNode::argument("player", &ArgumentType::Players).execute(RemoveCommandExecutor));

    cmd
}

struct RemoveCommandExecutor;

impl CommandHandler for RemoveCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
                let uuid = Uuid::from_str(player.get_id().as_str()).unwrap();

                AUTH_SERVICE.delete(uuid);

                player.send_system_message(TextComponent::text("Your staff password has been removed."), true);

            }
        }

        sender.send_message(TextComponent::text("Staff password removed for specified players."));

        Ok(1)
    }
}