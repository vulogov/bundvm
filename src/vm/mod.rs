use std::collections::HashMap;
use std::collections::VecDeque;
use rust_twostack::ts::TS;
use rust_dynamic::value::{Value};
use lazy_static::lazy_static;
use std::sync::Mutex;
use zenoh::config::{Config};
use crate::vm::vm_applicatives::{BundApplicative};

pub mod vm_applicatives;
pub mod vm_call;
pub mod vm_callstack;
pub mod vm_error;
pub mod vm_push;

#[derive(Clone)]
pub struct BUNDCore {
    pub version:        String,
    pub stack:          TS,
    pub call_stack:     VecDeque<Value>,
    pub scaffold:       VecDeque<Value>,
    pub applicatives:   HashMap<String, VecDeque<BundApplicative>>,
    pub functors:       HashMap<String, VecDeque<BundApplicative>>,
    pub lambdas:        HashMap<String, Value>,
    // bus
    pub zc:             Config,
    // vm behavioral
    pub shell_if_error: bool,
}

impl BUNDCore {
    pub fn init() -> Self {
        Self {
            version:        env!("CARGO_PKG_VERSION").to_string(),
            stack:          TS::new(),
            call_stack:     VecDeque::new(),
            applicatives:   HashMap::new(),
            functors:       HashMap::new(),
            scaffold:       VecDeque::new(),
            lambdas:        HashMap::new(),
            zc:             Config::default(),
            shell_if_error: false,
        }
    }
}

lazy_static! {
    pub static ref BUND: Mutex<BUNDCore> = {
        let vm: Mutex<BUNDCore> = Mutex::new(BUNDCore::new());
        vm
    };
}
