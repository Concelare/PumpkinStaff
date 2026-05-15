use std::str::FromStr;
use pumpkin_plugin_api::events::{EventHandler, FromIntoEvent, PlayerLeaveEvent};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use tracing::info;
use uuid::Uuid;
use crate::services::auth::{AUTH_SERVICE, VERIFIED};
use crate::services::freeze::FROZEN;

pub struct OnLeaveEvent;

impl EventHandler<PlayerLeaveEvent> for OnLeaveEvent {
    fn handle(&self, server: Server, event: <PlayerLeaveEvent as FromIntoEvent>::Data) -> <PlayerLeaveEvent as FromIntoEvent>::Data {
        let uuid = Uuid::from_str(&event.player.get_id().as_str()).unwrap();
        AUTH_SERVICE.on_leave(uuid);

        let frozen = FROZEN.get().unwrap();
        let lock = frozen.lock().unwrap();

        if lock.contains(&uuid) {
            let staff = VERIFIED.get();
            if staff.is_none() {
                info!("No staff members online to notify of player leaving while frozen.");
                return event;
            }

            staff.unwrap().lock().unwrap().iter().for_each(|uuid| {
                match server.get_player_by_uuid(uuid.to_string().as_str()) {
                    Some(staff) => {
                        staff.send_system_message(TextComponent::text(format!("{} has left the server while frozen", event.player.get_name()).as_str()), true);
                    }
                    None => {}
                }
            });

            event.player.ban(Some(TextComponent::text("Left while frozen")));
        }

        event
    }
}
