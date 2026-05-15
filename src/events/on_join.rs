use std::str::FromStr;
use pumpkin_plugin_api::events::{EventHandler, FromIntoEvent, PlayerJoinEvent};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use tracing::info;
use uuid::Uuid;
use crate::services::auth::AUTH_SERVICE;
use crate::services::database::DATABASE_SERVICE;
use crate::services::freeze::FROZEN;
use crate::services::vanish::VANISH_SERVICE;

pub struct OnJoinEvent;

impl EventHandler<PlayerJoinEvent> for OnJoinEvent {
    fn handle(&self, _server: Server, mut event: <PlayerJoinEvent as FromIntoEvent>::Data) -> <PlayerJoinEvent as FromIntoEvent>::Data {

        let uuid = Uuid::from_str(event.player.get_id().as_str()).unwrap();

        if DATABASE_SERVICE.exists(uuid) {
            AUTH_SERVICE.add_unverified(uuid);
            event.player.send_system_message(TextComponent::text("You are not verified, please use /staff login"), true);
        }

        let frozen = FROZEN.get();
        if frozen.is_none() {
            info!("No frozen players to check for on join.");
            return event;
        }
        let lock = frozen.unwrap().lock().unwrap();

        if lock.contains(&uuid) {
            event.player.send_system_message(TextComponent::text("You are frozen, please contact staff"), true);
        }

        if VANISH_SERVICE.is_vanished(uuid) {
            event.player.send_system_message(TextComponent::text("Silently Joined, You are vanished"), true);
            event.player.as_entity().set_invisible(true);
            event.cancelled = true;
        }

        event
    }
}