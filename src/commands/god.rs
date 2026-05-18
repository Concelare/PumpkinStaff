use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::gui::TextComponent;
use pumpkin_plugin_api::player::PlayerAbilities;
use pumpkin_plugin_api::Server;

pub fn god_command() -> Command {
    let names = ["god".to_string()];

    let cmd = Command::new(&names, "Toggle god mode").execute(GodCommandExecutor);

    cmd.then({
        let player_argument = CommandNode::argument("player", &ArgumentType::Players);
        player_argument.execute(GodCommandExecutor)
    });

    cmd
}

struct GodCommandExecutor;

impl CommandHandler for GodCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
                if player.get_abilities().invulnerable {
                    player.set_flying(false);
                    let abilities = toggle_god(player.get_abilities());
                    player.set_abilities(abilities);
                    player.send_system_message(TextComponent::text("God mode disabled"), true);
                    continue;
                }

                player.set_flying(true);
                let abilities = toggle_god(player.get_abilities());
                player.set_abilities(abilities);
                player.send_system_message(TextComponent::text("God mode enabled"), true);
            }

            return match sender.as_player() {
                Some(player) => {
                    player.send_system_message(TextComponent::text("God mode toggled for all players"), true);
                    Ok(1)
                },
                None => {
                    sender.send_message(TextComponent::text("God mode toggled for all players"));
                    Ok(1)
                },
            }
        }

        let player = match sender.as_player() {
            Some(player) => player,
            None => {
                sender.send_message(TextComponent::text("Missing Argument: 'Player'"));
                return Ok(0);
            },
        };

        if player.get_abilities().invulnerable {
            player.set_flying(false);
            let abilities = toggle_god(player.get_abilities());
            player.set_abilities(abilities);
            player.send_system_message(TextComponent::text("God mode disabled"), true);
            return Ok(1);
        }

        player.set_flying(true);
        let abilities =  toggle_god(player.get_abilities());
        player.set_abilities(abilities);
        player.send_system_message(TextComponent::text("God mode enabled"), true);

        Ok(1)
    }
}


fn toggle_god(mut player_abilities: PlayerAbilities) -> PlayerAbilities {
    player_abilities.invulnerable = true;
    player_abilities
}