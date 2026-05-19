use std::str::FromStr;
use pumpkin_plugin_api::events::{EventHandler, FromIntoEvent, PlayerMoveEvent};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use tracing::info;
use uuid::Uuid;
use crate::services::auth::UNVERIFIED;
use crate::services::freeze::FROZEN;

pub struct MovementEvent;

impl EventHandler<PlayerMoveEvent> for MovementEvent {
    fn handle(&self, _server: Server, mut event: <PlayerMoveEvent as FromIntoEvent>::Data) -> <PlayerMoveEvent as FromIntoEvent>::Data {
        let uuid = Uuid::from_u64_pair(event.player.get_id().high, event.player.get_id().low);
        let unverified = UNVERIFIED.get().unwrap();
        let lock = unverified.lock().unwrap();

        if lock.contains(&uuid) {
            event.player.send_system_message(TextComponent::text("You are not verified, please use /login"), true);
            event.cancelled = true
        }

        let frozen = FROZEN.get();
        if frozen.is_none() {
            info!("No frozen players to check for on movement.");
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