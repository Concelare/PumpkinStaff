use std::str::FromStr;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use tracing::info;
use uuid::Uuid;
use crate::services::freeze::FREEZE_SERVICE;

pub fn unfreeze_command() -> Command {
    let names = ["unfreeze".to_string()];
    let cmd = Command::new(&names, "Unfreeze a frozen account").execute(UnfreezeCommandExecutor);
    cmd.then({
        let node = CommandNode::argument("player", &ArgumentType::Players);
        node.execute(UnfreezeCommandExecutor)
    });

    cmd
}

struct UnfreezeCommandExecutor;

impl CommandHandler for UnfreezeCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
                let uuid = Uuid::from_u64_pair(player.get_id().high, player.get_id().low);
                FREEZE_SERVICE.unfreeze(uuid);
                let msg = TextComponent::text("You have been unfrozen!");
                msg.bold(true);
                msg.underlined(true);
                msg.color_named(NamedColor::Green);
                player.send_system_message(msg, true);
            }

            if sender.is_player() {
                sender.as_player().unwrap().send_system_message(TextComponent::text("Player has been unfrozen!"), true);
            }
            else {
                info!("Player has been unfrozen by console command.")
            }

            return Ok(1);
        }

        if sender.is_player() {
            let missing_msg = TextComponent::text("Missing Argument: 'Player'");
            missing_msg.color_named(NamedColor::DarkRed);
            missing_msg.bold(true);
            sender.as_player().unwrap().send_system_message(missing_msg, true);
        }
        else {
            info!("Missing argument 'Player' provided by console command.")
        }

        Ok(1)
    }
}