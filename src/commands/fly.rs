use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::gui::TextComponent;
use pumpkin_plugin_api::player::PlayerAbilities;
use pumpkin_plugin_api::Server;

pub fn fly_command() -> Command {
    let names = ["fly".to_string()];
    
    let cmd = Command::new(&names, "Toggles flying mode").execute(FlyCommandExecutor);
    
    cmd.then(
        {
            let node = CommandNode::argument("player", &ArgumentType::Players);
            
            node.execute(FlyCommandExecutor)
        });
    
    cmd
}

struct FlyCommandExecutor;

impl CommandHandler for FlyCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
                if player.is_flying() {
                    player.set_flying(false);
                    let abilities = toggle_flight(player.get_abilities());
                    player.set_abilities(abilities);
                    player.send_system_message(TextComponent::text("Flying mode disabled"), false);
                    continue;
                }
                
                player.set_flying(true);
                let abilities = toggle_flight(player.get_abilities());
                player.set_abilities(abilities);
                player.send_system_message(TextComponent::text("Flying mode enabled"), false);
            }

            return match sender.as_player() {
                Some(player) => {
                    player.send_system_message(TextComponent::text("Flying mode toggled for all players"), false);
                    Ok(1)
                },
                None => {
                    sender.send_message(TextComponent::text("Flying mode toggled for all players"));
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
        
        if player.is_flying() {
            player.set_flying(false);
            let abilities = toggle_flight(player.get_abilities());
            player.set_abilities(abilities);
            player.send_system_message(TextComponent::text("Flying mode disabled"), false);
            return Ok(1);
        }
        
        player.set_flying(true);
        let abilities = toggle_flight(player.get_abilities());
        player.set_abilities(abilities);
        player.send_system_message(TextComponent::text("Flying mode enabled"), false);
        
        Ok(1)
    }
    
}

fn toggle_flight(mut player_abilities: PlayerAbilities) -> PlayerAbilities {
    player_abilities.allow_flying = !player_abilities.allow_flying;
    player_abilities
}