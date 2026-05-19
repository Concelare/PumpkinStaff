use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType, StringType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use uuid::Uuid;
use crate::config::{SecurityMode, CONFIG};
use crate::services::auth::AUTH_SERVICE;
use crate::services::database::DATABASE_SERVICE;

pub fn login_command() -> Command {
    let names = ["login".to_string()];
    let cmd = Command::new(&names, "Login to the server").execute(LoginCommandExecutor);
    
    cmd.then(CommandNode::argument("password", &ArgumentType::String(StringType::SingleWord)).execute(LoginCommandExecutor));

    cmd
}

pub struct LoginCommandExecutor;

impl CommandHandler for LoginCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {

        let player = match sender.as_player() {
            Some(player) => player,
            None => return {
                let error_msg = TextComponent::text("Command can only be used by players");
                error_msg.color_named(NamedColor::DarkRed);
                error_msg.bold(true);
                sender.send_message(error_msg);
                Ok(0)
            },
        };

        let uuid = Uuid::from_u64_pair(player.get_id().high, player.get_id().low);

        if let Arg::Simple(password) = args.get_value("password") {
            return if CONFIG.mode == SecurityMode::Password {
                if !DATABASE_SERVICE.exists(uuid) {
                    let error_msg = TextComponent::text("You don't have a password");
                    error_msg.color_named(NamedColor::DarkRed);
                    error_msg.bold(true);
                    sender.send_message(error_msg);
                    return Ok(1);
                }

                if !AUTH_SERVICE.check_password(uuid, password.as_str()) {
                    let error_msg = TextComponent::text("Invalid password. Please try again.");
                    error_msg.color_named(NamedColor::DarkRed);
                    error_msg.bold(true);
                    sender.send_message(error_msg);
                    return Ok(1);
                }

                let success_msg = TextComponent::text("Password verified successfully");
                success_msg.color_named(NamedColor::Green);
                success_msg.bold(true);
                sender.send_message(success_msg);
                AUTH_SERVICE.verify(uuid);
                Ok(1)
            } else if CONFIG.mode == SecurityMode::TwoFactor {
                match AUTH_SERVICE.verify_totp(uuid, password.as_str()) {
                    true => {
                        let success_msg = TextComponent::text("TOTP verified successfully");
                        success_msg.color_named(NamedColor::Green);
                        success_msg.bold(true);
                        sender.send_message(success_msg);
                        AUTH_SERVICE.verify(uuid);
                        Ok(1)
                    }
                    false => {
                        let error_msg = TextComponent::text("Invalid TOTP code. Please try again.");
                        error_msg.color_named(NamedColor::DarkRed);
                        error_msg.bold(true);
                        sender.send_message(error_msg);
                        Ok(1)
                    }
                }
            } else {
                Ok(0)
            }
        }

        let error_msg = TextComponent::text("Missing argument: 'Password'");
        error_msg.color_named(NamedColor::DarkRed);
        error_msg.bold(true);
        sender.send_message(error_msg);

        Ok(0)
    }
}