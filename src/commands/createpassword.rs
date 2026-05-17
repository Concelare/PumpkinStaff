use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType, StringType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::Server;
use std::str::FromStr;
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use tracing::error;
use uuid::Uuid;
use crate::config::{SecurityMode, CONFIG};
use crate::services::auth::AUTH_SERVICE;
use crate::services::database::DATABASE_SERVICE;

pub fn create_command() -> Command {
    let names = ["createpassword".to_string()];
    let cmd = Command::new(&names, "Create a new account").execute(CreateCommandExecutor);

    cmd.then(CommandNode::argument("password", &ArgumentType::String(StringType::SingleWord)).execute(CreateCommandExecutor));

    cmd
}

pub struct CreateCommandExecutor;

impl CommandHandler for CreateCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {

        let uuid = match Uuid::from_str(sender.as_player().unwrap().get_id().as_str()) {
            Ok(uuid) => uuid,
            Err(_) => return {
                let error_msg = TextComponent::text("Failed to parse player UUID");
                error_msg.color_named(NamedColor::DarkRed);
                error_msg.bold(true);
                sender.send_message(error_msg);
                Ok(0)
            },
        };

        if let Arg::Simple(password) = args.get_value("password") {
            if CONFIG.mode == SecurityMode::Password {
                if DATABASE_SERVICE.exists(uuid) {
                    let error_msg = TextComponent::text("You already have a password");
                    error_msg.color_named(NamedColor::DarkRed);
                    error_msg.bold(true);
                    sender.send_message(error_msg);
                    return Ok(0);
                }

                if !AUTH_SERVICE.create_user_password(uuid, password.as_str()) {
                    let error_msg = TextComponent::text("Failed to create password");
                    error_msg.color_named(NamedColor::DarkRed);
                    error_msg.bold(true);
                    sender.send_message(error_msg);
                    return Ok(0);
                }

                let success_msg = TextComponent::text("Password created successfully");
                success_msg.color_named(NamedColor::Green);
                success_msg.bold(true);
                sender.send_message(success_msg);
            }
            else if CONFIG.mode == SecurityMode::TwoFactor {
                AUTH_SERVICE.generate_new_totp(uuid).map(|qr_code| {
                    let success_msg = TextComponent::text("TOTP QR Code generated successfully");
                    success_msg.color_named(NamedColor::Green);
                    success_msg.bold(true);
                    sender.send_message(success_msg);
                    let qr_message = TextComponent::text(format!("Scan the QR code with your authenticator app:\n{}", qr_code).as_str());
                    qr_message.color_named(NamedColor::Yellow);
                    qr_message.bold(true);
                    sender.send_message(qr_message);
                }).unwrap_or_else(|err| {
                    let error_msg = TextComponent::text("Failed to generate TOTP QR Code".to_string().as_str());
                    error_msg.color_named(NamedColor::DarkRed);
                    error_msg.bold(true);
                    sender.send_message(error_msg);
                    error!("Failed to generate TOTP QR Code: {}", err);
                })
            }
            else {
                let error_msg = TextComponent::text("Security Mode is not enabled.");
                error_msg.color_named(NamedColor::DarkRed);
                error_msg.bold(true);
                sender.send_message(error_msg);
                return Ok(0);
            }

            return Ok(1);
        }

        let error_msg = TextComponent::text("Missing argument: 'Password'");
        error_msg.color_named(NamedColor::DarkRed);
        error_msg.bold(true);
        sender.send_message(error_msg);

        Ok(0)
    }
}