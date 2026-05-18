use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::permission::{Permission, PermissionDefault, PermissionLevel};
use tracing::{error, info};
use crate::commands::createpassword::create_command;
use crate::commands::fly::fly_command;
use crate::commands::freeze::freeze_command;
use crate::commands::god::god_command;
use crate::commands::login::login_command;
use crate::commands::removepassword::remove_command;
use crate::commands::speed::speed_command;
use crate::commands::unfreeze::unfreeze_command;
use crate::commands::vanish::vanish_command;
use crate::PERMISSION_BASE;

pub mod createpassword;
pub mod login;
pub mod freeze;
pub mod unfreeze;
pub mod vanish;
pub mod removepassword;
pub mod fly;
pub mod speed;
pub mod god;

pub fn register_commands(context: &Context) {
    info!("Registering commands...");

    info!("Registering Login Permission...");
    let permission = Permission {
        node: PERMISSION_BASE.to_string() + "login",
        description: "Login Command Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };
    match context.register_permission(&permission) {
        Ok(_) => info!("Permission registered successfully"),
        Err(e) => error!("Failed to register permission: {}", e),
    }

    info!("Login Permission registered successfully.");

    info!("Registering Login Command...");
    context.register_command(login_command(), &permission.node.as_str());
    info!("Login Command registered successfully");

    info!("Registering Create Permission...");
    let create_permission = Permission {
        node: PERMISSION_BASE.to_string() + "create",
        description: "Create Command Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };
    match context.register_permission(&create_permission) {
        Ok(_) => info!("Permission registered successfully"),
        Err(e) => error!("Failed to register permission: {}", e),
    }
    info!("Create Permission registered successfully");

    info!("Registering Create Permission...");
    context.register_command(create_command(), &create_permission.node.as_str());
    info!("Create Command registered successfully");

    info!("Registering Remove Permission...");
    let remove_permission = Permission {
        node: PERMISSION_BASE.to_string() + "remove",
        description: "Remove Command Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };
    match context.register_permission(&remove_permission) {
        Ok(_) => info!("Permission registered successfully"),
        Err(e) => error!("Failed to register permission: {}", e),
    }
    info!("Remove Permission registered successfully");

    info!("Registering Remove Command...");
    context.register_command(remove_command(), &remove_permission.node.as_str());
    info!("Remove Command registered successfully");

    info!("Registering Fly Permission...");
    let fly_permission = Permission {
        node: PERMISSION_BASE.to_string() + "fly",
        description: "Fly Command Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };
    match context.register_permission(&fly_permission) {
        Ok(_) => info!("Permission registered successfully"),
        Err(e) => error!("Failed to register permission: {}", e),
    }
    info!("Fly Permission registered successfully");

    info!("Registering Fly Command...");

    context.register_command(fly_command(), &fly_permission.node.as_str());

    info!("Fly Command registered successfully");

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

    info!("Registering Speed Permission...");
    let speed_permission = Permission {
        node: PERMISSION_BASE.to_string() + "speed",
        description: "Speed Command Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };
    match context.register_permission(&speed_permission) {
        Ok(_) => info!("Permission registered successfully"),
        Err(e) => error!("Failed to register permission: {}", e),
    }
    info!("Speed Permission registered successfully");

    info!("Registering Speed Command...");
    context.register_command(speed_command(), &speed_permission.node.as_str());
    info!("Speed Command registered successfully");

    info!("Registering God Permission...");
    let god_permission = Permission {
        node: PERMISSION_BASE.to_string() + "god",
        description: "God Command Permission".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: vec![],
    };
    match context.register_permission(&god_permission) {
        Ok(_) => info!("Permission registered successfully"),
        Err(e) => error!("Failed to register permission: {}", e),
    }
    info!("God Permission registered successfully");

    info!("Registering God Command...");
    context.register_command(god_command(), &god_permission.node.as_str());
    info!("God Command registered successfully");

    info!("Registration completed successfully");
}