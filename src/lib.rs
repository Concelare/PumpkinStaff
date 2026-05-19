use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use tracing::*;
use crate::commands::register_commands;
use crate::config::{SecurityMode, CONFIG};

mod config;
mod services;
pub mod commands;
pub mod models;
pub mod events;

const PERMISSION_BASE: &str = "PumpkinStaff:command.";

struct PumpkinStaffPlugin;
impl Plugin for PumpkinStaffPlugin {
    fn new() -> Self {
        PumpkinStaffPlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "PumpkinStaff".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Concelare".into()],
            description: "A staff plugin made by Concelare for PumpkinMC".into(),
            dependencies: vec![],
            permissions: vec!["fs.read.data".to_string(), "fs.write.data".to_string()],
        }
    }

    fn on_load(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Loading PumpkinStaff plugin...");

        let config_path = PathBuf::from(_context.get_data_folder() + "/pumpkinstaff.toml");
        let db_path = PathBuf::from(_context.get_data_folder()).join("pumpkinstaff.db");

        if !config_path.exists()  {
            info!("Config file not found. Creating...");
            let mut file = match File::create(config_path.clone()) {
                Ok(file) => {
                    info!("Created config file.");
                    file
                }
                Err(e) => {
                    error!("Failed to create config file: {}", e);
                    panic!("Failed to create config file");
                }
            };

            let config = config::Config {
                mode: SecurityMode::None,
                redb_path: "redb.db".to_string(),
            };

            file.write_all(toml::to_string(&config).unwrap().as_bytes()).unwrap();
        }

        info!("Loading config file...");
        config::Config::init(&config_path);
        info!("Config file loaded.");

        if CONFIG.mode == SecurityMode::TwoFactor {
            panic!("Two-factor authentication is not supported yet.");
        }

        info!("Loading Database Service...");
        match services::database::DatabaseService::init(db_path.to_str().unwrap()) {
            Ok(_) => info!("Database Service loaded."),
            Err(e) => error!("Failed to load Database Service: {}", e),
        }

        info!("Loading Auth Service...");
        services::auth::AuthService::init();
        info!("Auth Service loaded.");

        info!("Loading Freeze Service...");
        services::freeze::FreezeService::init();
        info!("Freeze Service loaded.");

        info!("Loading Vanish Service...");
        services::vanish::VanishService::init();
        info!("Vanish Service loaded.");

        info!("Loading StaffChat Service...");
        services::staffchat::StaffChatService::init(_context.get_server());
        info!("StaffChat Service loaded.");

        register_commands(&_context);
        
        events::register_events(&_context);

        info!("PumpkinStaff plugin loaded!");

        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("PumpkinStaff plugin unloaded. Goodbye!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(PumpkinStaffPlugin);