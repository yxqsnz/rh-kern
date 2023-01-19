#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

use uefi::prelude::*;
use uefi_services::println;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    println!("Hello world!");
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}

