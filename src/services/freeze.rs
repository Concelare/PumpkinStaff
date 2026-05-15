use std::ops::Deref;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

pub static FREEZE_SERVICE: Ref = Ref(OnceLock::new());

pub static FROZEN: OnceLock<Mutex<Vec<Uuid>>> = OnceLock::new();

pub struct FreezeService;

impl FreezeService {
    pub fn init() {
        FREEZE_SERVICE.0.get_or_init(|| FreezeService);

        FROZEN.set(Mutex::new(Vec::new())).ok();

        assert!(FREEZE_SERVICE.0.get().is_some());
    }

    pub fn freeze(&self, id: Uuid) {
        let mut frozen = FROZEN.get().unwrap().lock().unwrap();
        frozen.push(id);
    }

    pub fn unfreeze(&self, id: Uuid) {
        let mut frozen = FROZEN.get().unwrap().lock().unwrap();
        frozen.retain(|&i| i != id);
    }
}


pub struct Ref(OnceLock<FreezeService>);

impl Deref for Ref {
    type Target = FreezeService;
    fn deref(&self) -> &Self::Target {
        self.0.get().unwrap()
    }
}