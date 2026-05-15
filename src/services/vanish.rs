use std::ops::Deref;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;


pub static VANISH_SERVICE: Ref = Ref(OnceLock::new());

pub static VANISHED: OnceLock<Mutex<Vec<Uuid>>> = OnceLock::new();

pub struct VanishService;

impl VanishService {

    pub fn init() {
        VANISHED.get_or_init(|| Mutex::new(Vec::new()));

        VANISH_SERVICE.0.get_or_init(|| VanishService);

        assert!(VANISH_SERVICE.0.get().is_some(), "Vanish service initialization failed");
    }

    pub fn vanish(&self, uuid: Uuid) {
        let mut vanished = VANISHED.get().unwrap().lock().unwrap();
        vanished.push(uuid);
    }

    pub fn unvanish(&self, uuid: Uuid) {
        let mut vanished = VANISHED.get().unwrap().lock().unwrap();
        vanished.retain(|&u| u != uuid);
    }
    
    pub fn is_vanished(&self, uuid: Uuid) -> bool {
        let vanished = VANISHED.get().unwrap().lock().unwrap();
        vanished.contains(&uuid)
    }
}

pub struct Ref(OnceLock<VanishService>);

impl Deref for Ref {
    type Target = VanishService;
    fn deref(&self) -> &Self::Target {
        self.0.get().unwrap()
    }
}