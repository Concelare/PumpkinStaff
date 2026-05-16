use std::str::FromStr;
use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use tracing::info;
use uuid::Uuid;
use crate::services::vanish::VANISH_SERVICE;

pub fn vanish_command() -> Command {
    let names = ["vanish".to_string(), "v".to_string(), "invisible".to_string()];
    let cmd = Command::new(&names, "Vanish yourself from the server").execute(VanishCommandExecutor);

    cmd.then({
        let node = CommandNode::argument("Player", &ArgumentType::Players);
        node.execute(VanishCommandExecutor)
    });

    cmd
}

struct VanishCommandExecutor;

impl CommandHandler for VanishCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
                let uuid = Uuid::from_str(player.get_id().as_str()).unwrap();
                if VANISH_SERVICE.is_vanished(uuid) {
                    VANISH_SERVICE.unvanish(uuid);
                    player.as_entity().set_invisible(false);
                    let msg = TextComponent::text("You have been unvanished!");
                    msg.bold(true);
                    msg.underlined(true);
                    msg.color_named(NamedColor::Green);
                    player.send_system_message(msg, true);

                    if sender.is_player() {
                        sender.as_player().unwrap().send_system_message(TextComponent::text("Player has been unvanished!"), true);
                    }
                    else {
                        info!("Player has been unvanished by console command.")
                    }

                    continue;
                }

                VANISH_SERVICE.vanish(uuid);
                player.as_entity().set_invisible(true);
                let msg = TextComponent::text("You have been vanished!");
                msg.bold(true);
                msg.underlined(true);
                msg.color_named(NamedColor::Green);
                player.send_system_message(msg, true);

                if sender.is_player() {
                    sender.as_player().unwrap().send_system_message(TextComponent::text("Player has been vanished!"), true);
                }
                else {
                    info!("Player has been vanished by console command.")
                }
            }
            return Ok(1);
        }


        if sender.is_console() {
            sender.send_message(TextComponent::text("Missing Argument: 'Player'"));
            return Ok(1);
        }

        let player = match sender.as_player() {
            Some(player) => player,
            None => {
                sender.send_message(TextComponent::text("Sender is not a player"));
                return Ok(1);
            }
        };

        let uuid= match Uuid::from_str(player.get_id().as_str()) {
            Ok(uuid) => uuid,
            Err(_) => {
                sender.send_message(TextComponent::text("Invalid player UUID"));
                return Ok(1);
            }
        };

        if VANISH_SERVICE.is_vanished(uuid) {
            VANISH_SERVICE.unvanish(uuid);
            player.as_entity().set_invisible(false);
            player.set_tab_list_listed(true);
            sender.as_player().unwrap().send_system_message(TextComponent::text("You are unvanished!"), true);
        }
        else {
            VANISH_SERVICE.vanish(uuid);
            player.as_entity().set_invisible(true);
            player.set_tab_list_listed(false);
            sender.as_player().unwrap().send_system_message(TextComponent::text("You are vanished!"), true);
        }


        
        Ok(1)
    }
}