#[cfg(target_os="windows")]
mod hack;
#[cfg(target_os="windows")]
mod process;
#[cfg(target_os="windows")]
mod exec;

#[cfg(target_os="windows")]
use crate::exec::exec;

#[cfg(target_os="windows")]
fn main() {
    exec();
}

#[cfg(target_os="linux")]
fn main() {
    println!("只有windows平台才需要编译")
}