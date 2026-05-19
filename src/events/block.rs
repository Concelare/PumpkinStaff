use pumpkin_plugin_api::events::{BlockBreakEvent, BlockPlaceEvent, EventHandler, FromIntoEvent};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use tracing::info;
use uuid::Uuid;
use crate::services::auth::UNVERIFIED;
use crate::services::freeze::FROZEN;

pub struct PlaceBlockEvent;

impl EventHandler<BlockPlaceEvent> for PlaceBlockEvent {
    fn handle(&self, _server: Server, mut event: <BlockPlaceEvent as FromIntoEvent>::Data) -> <BlockPlaceEvent as FromIntoEvent>::Data {
        let uuid = Uuid::from_u64_pair(event.player.get_id().high, event.player.get_id().low);
        let unverified = UNVERIFIED.get().unwrap();
        let lock = unverified.lock().unwrap();

        if lock.contains(&uuid) {
            event.player.send_system_message(TextComponent::text("You are not verified, please use /login"), true);
            event.cancelled = true
        }

        let frozen = FROZEN.get();
        if frozen.is_none() {
            info!("No frozen players to check for on block place.");
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


pub struct BreakEvent;

impl EventHandler<BlockBreakEvent> for BreakEvent {
    fn handle(&self, _server: Server, mut event: <BlockBreakEvent as FromIntoEvent>::Data) -> <BlockBreakEvent as FromIntoEvent>::Data {
        let player = match event.player {
            Some(ref player) => player,
            None => return event
        };

        let uuid = Uuid::from_u64_pair(player.get_id().high, player.get_id().low);
        let unverified = UNVERIFIED.get().unwrap();
        let lock = unverified.lock().unwrap();

        if lock.contains(&uuid) {
            player.send_system_message(TextComponent::text("You are not verified, please use /staff login"), true);
            event.cancelled = true
        }

        let frozen = FROZEN.get().unwrap();
        let lock = frozen.lock().unwrap();

        if lock.contains(&uuid) {
            player.send_system_message(TextComponent::text("You are frozen, please contact staff"), true);
            event.cancelled = true
        }

        event
    }
}