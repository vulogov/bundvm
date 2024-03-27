extern crate log;
use easy_error::{Error};

pub fn run_json_instructions(inst: String) -> Result<(), Error> {
    for instruction in inst.lines() {
        println!("{}", &instruction);
    }
    Ok(())
}
