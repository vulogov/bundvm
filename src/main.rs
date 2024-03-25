pub mod cmd;
pub mod stdlib;
pub mod vm;
pub mod vm_json;
pub mod vm_shell;
pub mod vm_stdlib;

fn main() {
    cmd::init();
}
