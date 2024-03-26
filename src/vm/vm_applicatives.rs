extern crate log;
use easy_error::{Error};
use std::collections::VecDeque;
use crate::vm::BUNDCore;
use crate::vm_stdlib;
use rust_dynamic::value::{Value};

//
// Number of required operands for the Applicatives
//
pub const NOEXTRA: u16         = 0;
pub const JUSTONE: u16         = 1;
pub const JUSTTWO: u16         = 2;
pub const TAKEALL: u16         = 3;

pub type BundCtxAppFn  = fn(&mut BUNDCore,&str,Value) -> Result<Option<Value>, Error>;

#[derive(Clone)]
pub struct BundApplicative {
    pub name:   String,
    pub extra:  u16,
    pub f:      BundCtxAppFn,
}

impl BundApplicative {
    pub fn new<N: AsRef<str> + std::fmt::Display>(name: N, e: u16, f: BundCtxAppFn) -> Self {
        Self {
            name:   name.as_ref().to_string(),
            extra:  e,
            f:      f,
        }
    }
}


impl BUNDCore {
    pub fn new() -> BUNDCore {
        let mut res = BUNDCore::init();
        vm_stdlib::init_stdlib(&mut res);
        res
    }
    pub fn resolve(&self, name: &str) -> Option<BundApplicative> {
        if self.applicatives.contains_key(&name.to_string()) {
            match self.applicatives.get(&name.to_string()) {
                Some(app) => return Some(app.back().unwrap().clone()),
                None => return None,
            }
        }
        None
    }
    pub fn have_applicative(&self, name: &str) -> bool {
        self.applicatives.contains_key(&name.to_string())
    }
    pub fn register(&mut self, name: &str, f: BundApplicative) -> bool {
        if ! self.applicatives.contains_key(&name.to_string()) {
            let mut q: VecDeque<BundApplicative> = VecDeque::new();
            q.push_back(f);
            self.applicatives.insert(name.to_string(), q);
        } else {
            let q = self.applicatives.get_mut(&name.to_string());
            q.expect("Applicative queue expected").push_back(f);
        }
        true
    }
    pub fn get_functor(&self, name: &str) -> Option<BundApplicative> {
        if self.functors.contains_key(&name.to_string()) {
            match self.functors.get(&name.to_string()) {
                Some(app) => return Some(app.back().unwrap().clone()),
                None => return None,
            }
        }
        None
    }
    pub fn register_functor(&mut self, name: &str, f: BundApplicative) -> bool {
        if ! self.functors.contains_key(&name.to_string()) {
            let mut q: VecDeque<BundApplicative> = VecDeque::new();
            q.push_back(f);
            self.functors.insert(name.to_string(), q);
        } else {
            let q = self.functors.get_mut(&name.to_string());
            q.expect("Functors queue expected").push_back(f);
        }
        true
    }
    pub fn have_functor(&self, name: &str) -> bool {
        self.functors.contains_key(&name.to_string())
    }
    pub fn eval(&mut self, _app: Option<BundApplicative>, _value: Value)  -> Option<Value> {
        None
    }
    pub fn bund(&self) -> &BUNDCore {
        self
    }
}
