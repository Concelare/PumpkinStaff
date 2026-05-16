use std::str::FromStr;
use pumpkin_plugin_api::events::{EventHandler, FromIntoEvent, PlayerCommandSendEvent};
use pumpkin_plugin_api::Server;
use uuid::Uuid;
use crate::services::auth::{AUTH_SERVICE, UNVERIFIED};

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
            event.cancelled = true;
        }
        
        event
    }
}