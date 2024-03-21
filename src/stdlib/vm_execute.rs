extern crate log;

pub fn instructions(inst: String) {
    if inst.len() == 0 {
        log::error!("No instructions has been passed");
    }
    for i in inst.lines() {
        log::debug!("Instruction received. len={}", &i.len());
    }
}
