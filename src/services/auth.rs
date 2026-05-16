use std::ops::Deref;
use std::sync::{Mutex, OnceLock};
use argon2::password_hash::{
    SaltString
};
use password_hash::{PasswordHasher, Salt};
use totp_rs::{Algorithm, Secret, TOTP};
use uuid::Uuid;
use crate::models::password::Password;
use password_hash::rand_core::OsRng;
use tracing::{error, info};
use crate::services::database::DATABASE_SERVICE;

pub static AUTH_SERVICE: Ref = Ref(OnceLock::new());

pub static VERIFIED: OnceLock<Mutex<Vec<Uuid>>> = OnceLock::new();
pub static UNVERIFIED: OnceLock<Mutex<Vec<Uuid>>> = OnceLock::new();

pub struct AuthService;

impl AuthService {
    
    pub fn init() {
        let auth = AuthService;

        UNVERIFIED.set(Mutex::new(Vec::new())).ok();

        VERIFIED.set(Mutex::new(Vec::new())).ok();

        assert!(AUTH_SERVICE.0.set(auth).is_ok());
    }

    pub fn create_user_password(&self, user: Uuid, password: &str) -> bool {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = argon2::Argon2::default();

        let hash = argon2.hash_password(password.as_bytes(), &salt);

        let password_hash = match hash {
            Ok(hash) => {
                hash
            },
            Err(e) => {
                error!("Error hashing password: {}", e);
                return false;
            }
        };

        let password_record = Password {
            user,
            password: password_hash.hash.unwrap().to_string(),
            salt: salt.to_string(),
        };

        DATABASE_SERVICE.save_password(user, password_record);

        true
    }

    pub fn check_password(&self, user: Uuid, password: &str) -> bool {
        let password_record = match DATABASE_SERVICE.get_password(user) {
            Some(record) => record,
            None => return false,
        };

        let argon2 = argon2::Argon2::default();

        let salt = Salt::from_b64(password_record.salt.as_str()).unwrap();

        let inputpassword = argon2.hash_password(password.as_bytes(), salt).unwrap();

        password_record.password == inputpassword.hash.unwrap().to_string()
    }

    pub fn generate_new_totp(&self, user: Uuid) -> Result<String, String> {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            Secret::Raw(user.as_bytes().to_vec()).to_bytes().unwrap(),
            Some("Minecraft".to_string()),
            "MinecraftServer".to_string()
        ).map_err(|e| format!("Failed to create TOTP: {}", e))?;

        totp.get_qr_base64().map_err(|e| format!("Failed to generate QR code: {}", e))
    }

    pub fn verify_totp(&self, user: Uuid, code: &str) -> bool {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            Secret::Raw(user.as_bytes().to_vec()).to_bytes().unwrap(),
            Some("Minecraft".to_string()),
            "MinecraftServer".to_string()
        );

        match totp.unwrap().generate_current() {
            Ok(generated_code) => {
                if generated_code != code {
                    return false;
                }
            },
            Err(e) => {
                error!("Failed to generate TOTP: {}", e);
                return false;
            }
        }

        true
    }

    pub fn add_unverified(&self, uuid: Uuid) {
        let vec = UNVERIFIED.get().expect("UNVERIFIED not initialized");
        let mut guard = vec.lock().unwrap();
        guard.push(uuid);
    }

    pub fn verify(&self,uuid: Uuid) {
        // Remove from unverified
        let vec = UNVERIFIED.get().expect("UNVERIFIED not initialized");
        let mut guard = vec.lock().unwrap();
        guard.retain(|u| *u != uuid);

        // Add to verified
        let vec = VERIFIED.get().expect("VERIFIED not initialized");
        let mut guard = vec.lock().unwrap();
        guard.push(uuid)
    }

    pub fn on_leave(&self, uuid: Uuid) {
        let vec = VERIFIED.get().expect("VERIFIED not initialized");
        let mut guard = vec.lock().unwrap();
        guard.retain(|u| *u != uuid);

        let vec = UNVERIFIED.get().expect("UNVERIFIED not initialized");
        let mut guard = vec.lock().unwrap();
        guard.retain(|u| *u != uuid);
        info!("{} has left the server", uuid);
    }
    
    pub fn delete(&self, uuid: Uuid) {
        // Remove from verified
        let vec = VERIFIED.get().expect("VERIFIED not initialized");
        let mut guard = vec.lock().unwrap();
        guard.retain(|u| *u != uuid);
        drop(guard);

        // Remove from unverified
        let vec = UNVERIFIED.get().expect("UNVERIFIED not initialized");
        let mut guard = vec.lock().unwrap();
        guard.retain(|u| *u != uuid);
        drop(guard);

        // Delete password record from database
        DATABASE_SERVICE.delete(uuid);
    }
}

pub struct Ref(OnceLock<AuthService>);

impl Deref for Ref {
    type Target = AuthService;
    fn deref(&self) -> &Self::Target {
        self.0.get().unwrap()
    }
}