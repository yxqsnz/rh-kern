#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]
extern crate alloc;
mod integration;

use rhai::{Engine, Scope};

use uefi::{
    prelude::entry,
    table::{Boot, SystemTable},
    Handle, Status,
};
use uefi_services::println;
mod log;
mod util;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    log::info("Loading Kernel ...");
    let mut engine = Engine::new();
    let mut scope = Scope::new();

    let maybe_kernel = util::read_kernel(system_table.boot_services());
    match maybe_kernel {
        Ok(kernel) => {
            integration::integrate(&mut engine);
            engine.run_with_scope(&mut scope, &kernel).unwrap();
        }
        Err(why) => {
            println!("Failed to load kernel: {why:?}");
            return Status::ABORTED;
        }
    }

    Status::SUCCESS
}
