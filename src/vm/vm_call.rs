extern crate log;
use crate::vm::BUNDCore;
use crate::vm::vm_applicatives::{NOEXTRA, JUSTONE, JUSTTWO, TAKEALL};
use rust_dynamic::types::{NONE};
use rust_dynamic::value::{Value};
use easy_error::{bail, ensure, Error};

impl BUNDCore {
    pub fn stack_take_one(&mut self) -> Result<Value, Error> {
        let mut res = Value::list();
        match self.stack.pull() {
            Some(value) => {
                match value.dt {
                    NONE => {
                        bail!("Separator found during stack reading");
                    }
                    _ => {
                        res.push(value);
                    }
                }
            }
            None => {
                bail!("Stack is not deep enough for the call requested");
            }
        }
        return Ok(res);
    }
    pub fn stack_take_two(&mut self) -> Result<Value, Error> {
        let mut res = Value::list();
        for _ in 0..2 {
            match self.stack.pull() {
                Some(value) => {
                    match value.dt {
                        NONE => {
                            bail!("Separator found during stack reading");
                        }
                        _ => {
                            res.push(value);
                        }
                    }
                }
                None => {
                    bail!("Stack is not deep enough for the call requested");
                }
            }
        }
        return Ok(res);
    }
    pub fn stack_take_all(&mut self) -> Result<Value, Error> {
        let mut res = Value::list();
        loop {
            match self.stack.pull() {
                Some(value) => {
                    match value.dt {
                        NONE => {
                            bail!("Separator found during stack reading");
                        }
                        _ => {
                            res.push(value);
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        return Ok(res);
    }
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
                        match self.stack_take_one() {
                            Ok(param) => {
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
                            Err(err) => {
                                bail!("{}", err);
                            }
                        }
                    }
                    JUSTTWO => {
                        ensure!(self.stack.stack_len() >= 2, "VM: Stack is not deep enough for {} call. len()=2 required", &fun);
                        match self.stack_take_two() {
                            Ok(param) => {
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
                            Err(err) => {
                                bail!("{}", err);
                            }
                        }
                    }
                    TAKEALL => {
                        ensure!(self.stack.stack_len() > 0, "VM: Stack is not deep enough for {} call", &fun);
                        match self.stack_take_all() {
                            Ok(param) => {
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
                            Err(err) => {
                                bail!("{}", err);
                            }
                        }
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
