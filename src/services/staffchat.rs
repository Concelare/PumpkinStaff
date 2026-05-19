use std::ops::Deref;
use std::sync::{Mutex, OnceLock};
use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::player;
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::server::Server;
use pumpkin_plugin_api::text::TextComponent;
use uuid::Uuid;

pub static STAFFCHAT_SERVICE: Ref = Ref(OnceLock::new());

pub static ONLINE_STAFF: OnceLock<Mutex<Vec<Uuid>>> = OnceLock::new();

pub static STAFFCHAT_MODE: OnceLock<Mutex<Vec<Uuid>>> = OnceLock::new();

pub struct StaffChatService {
    server: Server,
}
impl StaffChatService {
    pub fn init(server: Server) {
        STAFFCHAT_SERVICE.0.get_or_init(|| StaffChatService { server });
        
        ONLINE_STAFF.get_or_init(|| Mutex::new(Vec::new()));
        STAFFCHAT_MODE.get_or_init(|| Mutex::new(Vec::new()));

        assert!(STAFFCHAT_SERVICE.0.get().is_some(), "StaffChatService initialization failed");
    }

    pub fn add_staff_member(&self, uuid: Uuid) {
        let mut online_staff = ONLINE_STAFF.get().unwrap().lock().unwrap();
        online_staff.push(uuid);
    }

    pub fn remove_staff_member(&self, uuid: &Uuid) {
        let mut online_staff = ONLINE_STAFF.get().unwrap().lock().unwrap();
        if let Some(index) = online_staff.iter().position(|u| u == uuid) {
            online_staff.remove(index);
        }
    }

    pub fn send_chat(&self, staff: &Player, message: &str) {
        let online_staff = ONLINE_STAFF.get().unwrap().lock().unwrap();

        for staff_member in online_staff.iter() {
            let uuid_pair = staff_member.as_u64_pair();
            let uuid = player::Uuid { high: uuid_pair.0, low: uuid_pair.1 };

            let player = self.server.get_player_by_uuid(uuid).unwrap();

            let msg = TextComponent::text("[StaffChat] ");
            msg.color_named(NamedColor::DarkGreen);
            msg.bold(true);
            let name = staff.get_display_name();
            name.color_named(NamedColor::DarkRed);
            msg.add_child(name);
            let message = format!(" {}", message);
            let message_component = TextComponent::text(message.as_str());
            message_component.color_named(NamedColor::White);
            msg.add_child(message_component);

            player.send_system_message(msg, false);
        }
    }

    pub fn toggle_staff_mode(&self, uuid: &Uuid) {
        let mut staff_mode = STAFFCHAT_MODE.get().unwrap().lock().unwrap();
        if staff_mode.contains(uuid) {
            staff_mode.retain(|u| u != uuid);
        } else {
            staff_mode.push(uuid.clone());
        }
    }

    pub fn in_staff_mode(&self, uuid: &Uuid) -> bool {
        let staff_mode = STAFFCHAT_MODE.get().unwrap().lock().unwrap();
        staff_mode.contains(uuid)
    }
}


pub struct Ref(OnceLock<StaffChatService>);

impl Deref for Ref {
    type Target = StaffChatService;
    fn deref(&self) -> &Self::Target {
        self.0.get().unwrap()
    }
}