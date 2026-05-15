use std::str::FromStr;
use pumpkin_plugin_api::events::{EventHandler, FromIntoEvent, PlayerInteractEvent};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use tracing::info;
use uuid::Uuid;
use crate::services::auth::{AUTH_SERVICE, UNVERIFIED};
use crate::services::freeze::FROZEN;

pub struct InteractEvent;

impl EventHandler<PlayerInteractEvent> for InteractEvent {
    fn handle(&self, server: Server, mut event: <PlayerInteractEvent as FromIntoEvent>::Data) -> <PlayerInteractEvent as FromIntoEvent>::Data {
        let uuid = Uuid::from_str(event.player.get_id().as_str()).unwrap();
        let unverified = UNVERIFIED.get().unwrap();
        let lock = unverified.lock().unwrap();

        if lock.contains(&uuid) {
            event.player.send_system_message(TextComponent::text("You are not verified, please use /staff login2"), true);
            event.cancelled = true
        }

        let frozen = FROZEN.get();
        if frozen.is_none() {
            info!("No frozen players to check for on interact.");
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