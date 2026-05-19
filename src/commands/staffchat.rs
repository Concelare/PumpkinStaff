use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType, StringType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use uuid::Uuid;
use crate::services::staffchat::STAFFCHAT_SERVICE;

pub fn staffchat_command() -> Command {
    let names = ["staffchat".to_string(), "sc".to_string()];
    let cmd = Command::new(&names, "Staff Chat Command").execute(StaffChatExecutor);

    cmd.then({
        let node = CommandNode::argument("message", &ArgumentType::String(StringType::Greedy));

        node.execute(StaffChatExecutor)
    });
    
    cmd
}

struct StaffChatExecutor;

impl CommandHandler for StaffChatExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if let Arg::Simple(msg) = args.get_value("message") {
            if msg.is_empty() {
                sender.send_message(TextComponent::text("Cannot send empty message"));
                return Ok(0);
            }

            let player = match sender.as_player() {
                Some(player) => player,
                None => {
                    sender.send_message(TextComponent::text("Only players can use staff chat"));
                    return Ok(0);
                }
            };

            STAFFCHAT_SERVICE.send_chat(&player, msg.as_str());

            return Ok(0);
        }

        let player = match sender.as_player() {
            Some(player) => player,
            None => {
                sender.send_message(TextComponent::text("Only players can use staff chat"));
                return Ok(1);
            }
        };

        let uuid = Uuid::from_u64_pair(player.get_id().high, player.get_id().low);

        STAFFCHAT_SERVICE.toggle_staff_mode(&uuid);

        if STAFFCHAT_SERVICE.in_staff_mode(&uuid) {
            sender.send_message(TextComponent::text("Staff Chat Mode Enabled"));
        } else {
            sender.send_message(TextComponent::text("Staff Chat Mode Disabled"));
        }

        Ok(1)
    }
}