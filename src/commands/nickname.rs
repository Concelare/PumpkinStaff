use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType, StringType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;

pub fn nickname_command() -> Command {
    let names = vec!["nickname".to_string(), "nick".to_string()];
    let cmd = Command::new(&names, "Set your nickname").execute(NicknameCommandExecutor);

    cmd.then({
        let node = CommandNode::argument("nickname", &ArgumentType::String(StringType::SingleWord));

        node.then(CommandNode::argument("player", &ArgumentType::Players).execute(NicknameCommandExecutor));

        node
    });

    cmd.then(CommandNode::argument("nickname", &ArgumentType::String(StringType::SingleWord)).execute(NicknameCommandExecutor));

    cmd
}

struct NicknameCommandExecutor;

impl CommandHandler for NicknameCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if let Arg::Simple(nickname) = args.get_value("nickname") {
            if let Arg::Players(players) = args.get_value("player") {
                for player in players {
                    let nickname = nickname.clone();
                    let display_name = TextComponent::text(nickname.as_str());
                    player.set_display_name(display_name);
                    player.send_system_message(TextComponent::text(format!("Your nickname has been set to {}", nickname).as_str()), false);
                }

                return Ok(0);
            }

            let player = match sender.as_player() {
                Some(player) => player,
                None => {
                    sender.send_message(TextComponent::text("You must be a player to use this command"));
                    return Ok(0);
                }
            };

            let nickname = nickname.clone();
            let display_name = TextComponent::text(nickname.as_str());
            player.set_display_name(display_name);
            player.send_system_message(TextComponent::text(format!("Your nickname has been set to {}", nickname).as_str()), false);

            return Ok(1);
        }

        sender.send_message(TextComponent::text("Missing player argument"));
        Ok(0)
    }
}