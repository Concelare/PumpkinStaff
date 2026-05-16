use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType, Number};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;

pub fn speed_command() -> Command {
    let names = ["speed".to_string(), "spd".to_string()];
    let cmd = Command::new(&names, "Speed Command").execute(SpeedCommandExecutor);

    cmd.then(
        {
            let node = CommandNode::argument("speed", &ArgumentType::Float((Some(0f32), Some(10f32))));

            node.then(CommandNode::argument("player", &ArgumentType::Players).execute(SpeedCommandExecutor));

            node
        }
    );

    cmd.then(CommandNode::argument("speed", &ArgumentType::Float((Some(0f32), Some(10f32)))).execute(SpeedCommandExecutor));


    cmd
}

struct SpeedCommandExecutor;

impl CommandHandler for SpeedCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {

        let speed_res = match args.get_value("speed") {
            Arg::Num(amount) => amount,
            _ => return Ok(0),
        };

        let speed = match speed_res {
            Ok(Number::Float32(amount)) => amount,
            _ => return Ok(0),
        };

        if let Arg::Players(players) = args.get_value("player") {
            for player in players {
                let mut abilities = player.get_abilities();
                abilities.walk_speed = speed;
                abilities.fly_speed = speed;
                player.set_abilities(abilities);

                player.send_system_message(TextComponent::text(format!("Your speed has been set to {}", speed).as_str()), true);
            }

            if let Some(player) = sender.as_player() {
                player.send_system_message(TextComponent::text("Speed set successfully."), true);
            }
            else {
                sender.send_message(TextComponent::text("Speed set successfully."));
            }

            return Ok(1);
        }

        let player = match sender.as_player() {
            Some(player) => player,
            None => return Ok(1),
        };

        let mut abilities = player.get_abilities();
        abilities.walk_speed = speed;
        abilities.fly_speed = speed;
        player.set_abilities(abilities);

        player.send_system_message(TextComponent::text(format!("Speed set to {}", speed).as_str()), true);
        Ok(1)
    }
}