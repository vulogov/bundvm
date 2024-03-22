pub mod cmd;
pub mod vm;
pub mod vm_json;
pub mod stdlib;
pub mod vm_shell;

fn main() {
    cmd::init();
}
