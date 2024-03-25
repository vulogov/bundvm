extern crate log;
use std::collections::VecDeque;
use crate::vm::BUNDCore;
use crate::vm_stdlib;
use rust_dynamic::ctx::{Context, CtxApplicative};
use rust_dynamic::value::{Value};

impl Context for BUNDCore {
    fn new() -> BUNDCore {
        let mut res = BUNDCore::init();
        vm_stdlib::init_stdlib(&mut res);
        res
    }
    fn resolve(&self, name: &str) -> Option<CtxApplicative> {
        if self.applicatives.contains_key(&name.to_string()) {
            match self.applicatives.get(&name.to_string()) {
                Some(app) => return Some(app.back().unwrap().clone()),
                None => return None,
            }
        }
        None
    }
    fn register(&mut self, name: &str, f: CtxApplicative) -> bool {
        if ! self.applicatives.contains_key(&name.to_string()) {
            let mut q: VecDeque<CtxApplicative> = VecDeque::new();
            q.push_back(f);
            self.applicatives.insert(name.to_string(), q);
        } else {
            let q = self.applicatives.get_mut(&name.to_string());
            q.expect("Applicative queue expected").push_back(f);
        }
        true
    }
    fn get_association(&self, _name: &str) -> Option<Value> {
        None
    }
    fn register_association(&mut self, _name: &str, _v: Value) -> bool {
        true
    }
    fn eval(&mut self, _app: Option<CtxApplicative>, _value: Value)  -> Option<Value> {
        None
    }
}
