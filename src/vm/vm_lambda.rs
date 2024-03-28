extern crate log;
use easy_error::{bail, Error};
use crate::vm::BUNDCore;
use rust_dynamic::value::Value;

impl BUNDCore {
    pub fn lambda_scaffolding(&mut self) -> bool {
        self.scaffold.len() > 0
    }
    pub fn lambda_scaffolding_push(&mut self, v: Value) -> Result<(), Error> {
        match self.scaffold.pop_back() {
            Some(mut l) => {
                l = l.push(v);
                self.scaffold.push_back(l);
            }
            None => {
                bail!("VM: Scaffolding stack is empty");
            }
        }
        Ok(())
    }
    pub fn lambda_finish_scaffolding(&mut self, param: Value) -> Result<(), Error> {
        match param.cast_list() {
            Ok(p) => {
                if p.len() > 0 {
                    let name = &p[0];
                    match name.cast_string() {
                        Ok(l_name) => {
                            match self.scaffold.pop_back() {
                                Some(l) => {
                                    self.lambdas.insert(l_name, l);
                                }
                                None => {
                                    bail!("VM: Scaffolding stack is empty");
                                }
                            }
                        }
                        Err(err) => {
                            bail!("{}", err);
                        }
                    }
                } else {
                    bail!("VM: parfameters passed for lambda termination are too shallow");
                }
            }
            Err(err) => {
                bail!("{}", err);
            }
        }
        Ok(())
    }
}
