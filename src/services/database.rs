use std::ops::Deref;
use std::sync::OnceLock;
use redb::{Database, ReadableDatabase, TableDefinition};
use tracing::error;
use uuid::Uuid;
use crate::models::password::Password;

pub static DATABASE_SERVICE: Ref = Ref(OnceLock::new());

const PASSWORDS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("passwords");

pub struct DatabaseService {
    db: Database,
}

impl DatabaseService {
    pub fn init(db_path: &str) -> Result<(), String> {

        let db_con = Database::create(db_path).map_err(|e| e.to_string())?;

        {
            let tx = db_con.begin_write().map_err(|e| e.to_string())?;
            {
                let _ = tx
                    .open_table(PASSWORDS_TABLE)
                    .map_err(|e| e.to_string())?;
            }
            tx.commit().map_err(|e| e.to_string())?;
        }


        let database = DatabaseService {
            db: db_con,
        };

        assert!(DATABASE_SERVICE.0.set(database).is_ok());
        Ok(())
    }

    pub fn save_password(&self, user: Uuid, password: Password) {
        let user = user.to_string();

        // Convert Password Record To JSON
        let json = match serde_json::to_string(&password).map_err(|e| e.to_string()) {
            Ok(json) => json,
            Err(e) => {
                error!("Failed to serialize password: {}", e);
                return;
            }
        };

        // Begin Transaction
        let tx = match self.db.begin_write().map_err(|e| e.to_string()) {
            Ok(tx) => tx,
            Err(e) => {
                error!("Failed to begin transaction: {}", e);
                return;
            }
        };

        {
            // Open Passwords Table
            let mut table = match tx.open_table(PASSWORDS_TABLE).map_err(|e| e.to_string()) {
                Ok(table) => table,
                Err(e) => {
                    error!("Failed to open passwords table: {}", e);
                    return;
                }
            };

            // Insert Password Record
            if let Err(e) = table.insert(user.as_str(), json.as_str()).map_err(|e| e.to_string()) {
                error!("Failed to insert password: {}", e);
                return;
            }
        }

        // Commit Transaction
        match tx.commit().map_err(|e| e.to_string()) {
            Ok(_) => (),
            Err(e) => error!("Failed to commit transaction: {}", e),
        }
    }

    pub fn get_password(&self, user: Uuid) -> Option<Password> {
        let user = user.to_string();
        let password: Option<Password>;

        let tx = match self.db.begin_read().map_err(|e| e.to_string()).ok() {
            Some(tx) => tx,
            None => return None,
        };

        {
            let table = match tx.open_table(PASSWORDS_TABLE).map_err(|e| e.to_string()).ok() {
                Some(table) => table,
                None => return None,
            };

            let password_record_str = match table.get(user.as_str()).map_err(|e| e.to_string()).ok() {
                Some(Some(password_record_str)) => password_record_str,
                Some(None) => return None,
                None => return None,
            };

            let password_record: Password = match serde_json::from_str(password_record_str.value().to_string().as_str()) {
                Ok(password_record) => password_record,
                Err(e) => {
                    error!("Failed to deserialize password record: {}", e);
                    return None;
                }
            };

            password = Some(password_record);
        }

        tx.close().map_err(|e| e.to_string()).ok();

        password
    }

    pub fn exists(&self, user: Uuid) -> bool {
        self.get_password(user).is_some()
    }


}

pub struct Ref(OnceLock<DatabaseService>);

impl Deref for Ref {
    type Target = DatabaseService;
    fn deref(&self) -> &Self::Target {
        self.0.get().unwrap()
    }
}

