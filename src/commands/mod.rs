use pumpkin_plugin_api::command::Command;
use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::permission::{Permission, PermissionDefault, PermissionLevel};
use tracing::{error, info};
use crate::commands::freeze::freeze_command;
use crate::commands::unfreeze::unfreeze_command;
use crate::commands::vanish::vanish_command;
use crate::PERMISSION_BASE;

pub mod create;
pub mod login;
pub mod freeze;
pub mod unfreeze;
pub mod vanish;

pub fn register_commands(context: &Context) {
    info!("Registering commands...");
    let names = ["staff".to_string()];
    let cmd = Command::new(&names, "");

    create::create_command(&cmd);
    login::login_command(&cmd);
    info!("Registering Staff Permission...");
    let permission = Permission {
        node: PERMISSION_BASE.to_string() + "staff",
        description: "Staff Commands Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };
    match context.register_permission(&permission) {
        Ok(_) => info!("Permission registered successfully"),
        Err(e) => error!("Failed to register permission: {}", e),
    }

    info!("Registering Staff Command...");
    context.register_command(cmd, &permission.node.as_str());
    info!("Staff Commands registered successfully");

    info!("Registering Freeze Permission...");

    let freeze_permission = Permission {
        node: PERMISSION_BASE.to_string() + "freeze",
        description: "Freeze Command Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };
    match context.register_permission(&freeze_permission) {
        Ok(_) => info!("Permission registered successfully"),
        Err(e) => error!("Failed to register permission: {}", e),
    }
    info!("Freeze Permission registered successfully");

    info!("Registering Freeze Command...");

    context.register_command(freeze_command(), &freeze_permission.node.as_str());

    info!("Freeze Command registered successfully");

    info!("Registering Unfreeze Permission...");
    let unfreeze_permission = Permission {
        node: PERMISSION_BASE.to_string() + "unfreeze",
        description: "Unfreeze Command Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };
    match context.register_permission(&unfreeze_permission) {
        Ok(_) => info!("Permission registered successfully"),
        Err(e) => error!("Failed to register permission: {}", e),
    }
    info!("Unfreeze Permission registered successfully");

    info!("Registering Unfreeze Command...");

    context.register_command(unfreeze_command(), &unfreeze_permission.node.as_str());

    info!("Unfreeze Command registered successfully");

    info!("Registering Vanish Permission...");

    let vanish_permission = Permission {
        node: PERMISSION_BASE.to_string() + "vanish",
        description: "Vanish Command Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };
    match context.register_permission(&vanish_permission) {
        Ok(_) => info!("Permission registered successfully"),
        Err(e) => error!("Failed to register permission: {}", e),
    }
    info!("Vanish Permission registered successfully");

    info!("Registering Vanish Command...");
    context.register_command(vanish_command(), &vanish_permission.node.as_str());

    info!("Vanish Command registered successfully");

    info!("Registration completed successfully");
}