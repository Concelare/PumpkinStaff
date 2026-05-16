use pumpkin_plugin_api::events::EventPriority;
use tracing::{error, info};
use crate::events::on_join::OnJoinEvent;

pub mod on_join;
pub mod on_leave;
pub mod interact;
pub mod chat;
pub mod movement;
pub mod block;
pub mod command;

pub fn register_events(context: &pumpkin_plugin_api::Context) {
    info!("Registering events...");
    info!("Registering OnJoinEvent...");
    match context.register_event_handler(OnJoinEvent, EventPriority::Highest, true) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to register OnJoinEvent: {}", e);
        }
    };
    info!("Registered OnJoinEvent.");

    info!("Registering OnLeaveEvent...");
    match context.register_event_handler(on_leave::OnLeaveEvent, EventPriority::Highest, true) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to register OnLeaveEvent: {}", e);
        }
    };
    info!("Registered OnLeaveEvent.");
    // Interact Doesn't Work At The Moment
    // info!("Registering InteractEvent...");
    // match context.register_event_handler(interact::InteractEvent, EventPriority::Highest, true) {
    //     Ok(_) => (),
    //     Err(e) => {
    //         error!("Failed to register InteractEvent: {}", e);
    //     }
    // };
    // info!("Registered InteractEvent.");

    info!("Registering ChatEvent...");
    match context.register_event_handler(chat::ChatEvent, EventPriority::Highest, true) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to register ChatEvent: {}", e);
        }
    };
    info!("Registered ChatEvent.");

    info!("Registering MovementEvent...");
    match context.register_event_handler(movement::MovementEvent, EventPriority::Highest, true) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to register MovementEvent: {}", e);
        }
    };
    info!("Registered MovementEvent.");

    info!("Registering BlockEvent...");
    match context.register_event_handler(block::BreakEvent, EventPriority::Highest, true) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to register BlockEvent: {}", e);
        }
    };
    info!("Registered BlockEvent.");

    info!("Registering BlockPlaceEvent...");
    match context.register_event_handler(block::PlaceBlockEvent, EventPriority::Highest, true) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to register BlockPlaceEvent: {}", e);
        }
    };
    info!("Registered BlockPlaceEvent.");
}