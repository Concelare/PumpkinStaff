use std::str::FromStr;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use tracing::info;
use uuid::Uuid;
use crate::services::freeze::FREEZE_SERVICE;

pub fn freeze_command() -> Command {
    let names = ["freeze".to_string()];

    let cmd = Command::new(&names, "Freeze the current state of the project").execute(FreezeCommandExecutor);

    cmd.then({
        let player_argument = CommandNode::argument("player", &ArgumentType::Players);
        player_argument.execute(FreezeCommandExecutor)
    });

    cmd
}

struct FreezeCommandExecutor;

impl CommandHandler for FreezeCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
                let uuid = match Uuid::from_str(player.get_id().as_str()) {
                    Ok(uuid) => uuid,
                    Err(_) => {
                        sender.send_message(TextComponent::text("Invalid player UUID format."));
                        return Ok(1);
                    }
                };
                FREEZE_SERVICE.freeze(uuid);
                let msg = TextComponent::text("You have been frozen! Contact Staff!");
                msg.bold(true);
                msg.underlined(true);
                msg.color_named(NamedColor::Red);
                player.send_system_message(msg, true);
            }

            if sender.is_player() {
                sender.as_player().unwrap().send_system_message(TextComponent::text("Player has been frozen!"), true);
            }
            else {
                info!("Player has been frozen by console command.")
            }

            return Ok(1);
        }

        let missing_msg = TextComponent::text("Missing Argument: 'Player'");
        missing_msg.color_named(NamedColor::DarkRed);
        missing_msg.bold(true);
        sender.as_player().unwrap().send_system_message(missing_msg, true);

        Ok(1)
    }
}