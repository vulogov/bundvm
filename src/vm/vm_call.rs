extern crate log;
use crate::vm::BUNDCore;
use crate::vm::vm_applicatives::{NOEXTRA, JUSTONE, JUSTTWO, TAKEALL};
use rust_dynamic::value::{Value};
use easy_error::{bail, ensure, Error};

impl BUNDCore {
    pub fn call(&mut self, fun: String) -> Result<(), Error> {
        log::debug!("BUND VM: calling functor: {}", &fun);
        if self.have_applicative(&fun.clone()) {
            match self.call_applicative(fun.clone()) {
                Ok(_) => {},
                Err(err) => {
                    self.display_message(format!("BUND VM: applicative call of {} is lead to an error: {:?}", fun, err), "N/A".to_string());
                    return Err(err);
                }
            }

        } else if self.have_functor(&fun.clone()) {
            match self.call_functor(fun.clone()) {
                Ok(_) => {},
                Err(err) => {
                    self.display_message(format!("BUND VM: functor call of {} is lead to an error: {:?}", fun, err), "N/A".to_string());
                    return Err(err);
                }
            }
        }
        Ok(())
    }
    pub fn call_applicative(&mut self, fun: String) -> Result<(), Error> {
        log::debug!("BUND VM: calling applicative: {}", &fun);
        match self.resolve(fun.as_str()) {
            Some(app) => {
                log::debug!("BUND VM: applicative {} has been found", &fun);
                match app.extra {
                    NOEXTRA => {
                        let param = Value::list();
                        match (app.f)(self, &app.name, param) {
                            Ok(res) => {
                                match res {
                                    Some(value) => {
                                        self.stack.push(value);
                                    }
                                    None => {},
                                }
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    }
                    JUSTONE => {
                        ensure!(self.stack.stack_len() >= 1, "VM: Stack is not deep enough for {} call. len()=1 required", &fun);

                    }
                    JUSTTWO => {
                        ensure!(self.stack.stack_len() >= 2, "VM: Stack is not deep enough for {} call. len()=2 required", &fun);
                    }
                    TAKEALL => {
                        ensure!(self.stack.stack_len() > 0, "VM: Stack is not deep enough for {} call", &fun);

                    }
                    _ => bail!("VM: illegal operant for applicative"),
                };
                return Ok(());
            }
            None => {
                bail!("VM: applicative call failed, so such applicative registered");
            }
        }
    }
    pub fn call_functor(&mut self, _fun: String) -> Result<(), Error> {
        bail!("VM: functor call failed, not implemented");
    }
}
