#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]
extern crate alloc;

use alloc::{string::String, vec};
use rhai::{Engine, Scope};
use uefi::{
    prelude::*,
    proto::media::{
        file::{File, FileAttribute, FileInfo, FileMode},
        fs::SimpleFileSystem,
    },
};
use uefi_services::println;

fn read_kernel(services: &BootServices) -> String {
    let handle = services
        .get_handle_for_protocol::<SimpleFileSystem>()
        .unwrap();
    let mut sfs = services
        .open_protocol_exclusive::<SimpleFileSystem>(handle)
        .expect("failed to open SimpleFileSystem protocol");

    let mut directory = sfs.open_volume().unwrap();
    let kernel = directory
        .open(
            cstr16!("kernel.rhai"),
            FileMode::Read,
            FileAttribute::empty(),
        )
        .unwrap();
    let mut buffer = vec![0; 128];
    let mut file = kernel.into_regular_file().unwrap();
    let file_info = file.get_info::<FileInfo>(&mut buffer).unwrap();
    let file_size = file_info.file_size() as usize;
    let mut file_buffer = vec![0; file_size];

    file.read(&mut file_buffer).unwrap();
    let string = String::from_utf8(file_buffer).unwrap();

    string
}

fn uefi_puts(content: &str) {
    println!("{content}")
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    println!("Loading Kernel ...");
    let mut engine = Engine::new();
    let mut scope = Scope::new();
    let kernel = read_kernel(system_table.boot_services());
    engine.register_fn("uefi_puts", uefi_puts);
    engine.run_with_scope(&mut scope, &kernel).unwrap();
    Status::SUCCESS
}
