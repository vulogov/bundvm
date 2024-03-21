use rust_twostack::ts::TS;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Clone)]
pub struct BUNDCore {
    pub version:        String,
    pub stack:          TS,
}

impl BUNDCore {
    pub fn init() -> Self {
        Self {
            version:        env!("CARGO_PKG_VERSION").to_string(),
            stack:          TS::new(),
        }
    }
}

lazy_static! {
    pub static ref BUND: Mutex<BUNDCore> = {
        let vm: Mutex<BUNDCore> = Mutex::new(BUNDCore::init());
        vm
    };
}
