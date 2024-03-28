extern crate log;
use crate::vm::{BUNDCore, BundApplicative};
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
                        res = res.push(value);
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
                            res = res.push(value);
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
                            res = res.push(value);
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
        if self.have_applicative(&fun.clone()) {
            log::debug!("BUND VM: applicative found: {}", &fun);
            match self.call_applicative(fun.clone()) {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => {
                    self.display_message(format!("BUND VM: applicative call of {} is lead to an error: {:?}", fun, err), "N/A".to_string());
                    return Err(err);
                }
            }

        } else if self.have_functor(&fun.clone()) {
            log::debug!("BUND VM: functor found: {}", &fun);
            match self.call_functor(fun.clone()) {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => {
                    self.display_message(format!("BUND VM: functor call of {} is lead to an error: {:?}", fun, err), "N/A".to_string());
                    return Err(err);
                }
            }
        } else {
            self.display_message(format!("BUND VM: applicative/functor/lambda {} do not exists", fun), "N/A".to_string());
            bail!("{}", format!("BUND VM: applicative/functor/lambda {} do not exists", fun));
        }
    }
    pub fn call_bundapplicative(&mut self, app: BundApplicative) -> Result<(), Error> {
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
                ensure!(self.stack.stack_len() >= 1, "VM: Stack is not deep enough for {} call. len()=1 required", &app.name);
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
                ensure!(self.stack.stack_len() >= 2, "VM: Stack is not deep enough for {} call. len()=2 required", &app.name);
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
                ensure!(self.stack.stack_len() > 0, "VM: Stack is not deep enough for {} call", &app.name);
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
    pub fn call_applicative(&mut self, fun: String) -> Result<(), Error> {
        log::debug!("BUND VM: calling applicative: {}", &fun);
        match self.resolve(fun.as_str()) {
            Some(app) => {
                return self.call_bundapplicative(app);
            }
            None => {
                bail!("VM: applicative call failed, so such applicative registered");
            }
        }
    }
    pub fn call_functor(&mut self, fun: String) -> Result<(), Error> {
        log::debug!("BUND VM: calling functor: {}", &fun);
        match self.get_functor(fun.as_str()) {
            Some(app) => {
                return self.call_bundapplicative(app);
            }
            None => {
                bail!("VM: functor call failed, so such functor registered");
            }
        }
    }
}
