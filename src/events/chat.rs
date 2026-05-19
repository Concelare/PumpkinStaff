use std::str::FromStr;
use pumpkin_plugin_api::events::{EventHandler, FromIntoEvent, PlayerChatEvent};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use tracing::info;
use uuid::Uuid;
use crate::services::auth::UNVERIFIED;
use crate::services::freeze::FROZEN;
use crate::services::staffchat::STAFFCHAT_SERVICE;

pub struct ChatEvent;

impl EventHandler<PlayerChatEvent> for ChatEvent {
    fn handle(&self, _server: Server, mut event: <PlayerChatEvent as FromIntoEvent>::Data) -> <PlayerChatEvent as FromIntoEvent>::Data {
        let uuid = Uuid::from_u64_pair(event.player.get_id().high, event.player.get_id().low);
        let unverified = UNVERIFIED.get().unwrap();
        let lock = unverified.lock().unwrap();

        if lock.contains(&uuid) {
            event.player.send_system_message(TextComponent::text("You are unverified, please use /login"), true);
            event.cancelled = true
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

        if STAFFCHAT_SERVICE.in_staff_mode(&uuid) && !event.message.contains("[StaffChat]") {
            STAFFCHAT_SERVICE.send_chat(&event.player, event.message.as_str());
            event.cancelled = true;
        }

        Uuid::from_u64_pair(event.player.get_id().high, event.player.get_id().low);

        event
    }
}