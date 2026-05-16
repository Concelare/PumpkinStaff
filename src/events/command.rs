use std::str::FromStr;
use pumpkin_plugin_api::events::{EventHandler, FromIntoEvent, PlayerCommandSendEvent};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use tracing::info;
use uuid::Uuid;
use crate::services::auth::{AUTH_SERVICE, UNVERIFIED};
use crate::services::freeze::FROZEN;

pub struct CommandEvent;

impl EventHandler<PlayerCommandSendEvent> for CommandEvent {
    fn handle(&self, server: Server, mut event: <PlayerCommandSendEvent as FromIntoEvent>::Data) -> <PlayerCommandSendEvent as FromIntoEvent>::Data {
        let player = &event.player;

        let unverified = match UNVERIFIED.get() {
            Some(unverified) => unverified,
            None => return event,
        };

        let lock = match unverified.lock() {
            Ok(lock) => lock,
            Err(_) => return event,
        };

        let uuid = match Uuid::from_str(player.get_id().as_str()) {
            Ok(uuid) => uuid,
            Err(_) => return event,
        };

        if lock.contains(&uuid) {
            event.player.send_system_message(TextComponent::text("You are not verified! You cannot use commands. Use /staff login!"), true);
            event.cancelled = true;
        }

        let frozen = FROZEN.get();
        if frozen.is_none() {
            info!("No frozen players to check for on chat.");
            return event;
        }
        let lock = frozen.unwrap().lock().unwrap();

        if lock.contains(&uuid) {
            event.player.send_system_message(TextComponent::text("You are frozen, please contact staff"), true);
            event.cancelled = true
        }

        event
    }
}