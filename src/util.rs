use alloc::{string::String, vec};
use uefi::{
    prelude::{cstr16, BootServices},
    proto::media::{
        file::{File, FileAttribute, FileInfo, FileMode},
        fs::SimpleFileSystem,
    },
};

use crate::log;

pub fn read_kernel(services: &BootServices) -> uefi::Result<String> {
    log::debug("Getting protocols ...");
    let handle = services.get_handle_for_protocol::<SimpleFileSystem>()?;
    let mut sfs = services.open_protocol_exclusive::<SimpleFileSystem>(handle)?;
    log::debug("Opening file ...");
    let mut directory = sfs.open_volume()?;
    let kernel = directory.open(
        cstr16!("kernel.rhai"),
        FileMode::Read,
        FileAttribute::empty(),
    )?;

    log::debug("Reading main rhai file ...");
    let mut buffer = vec![0; 128];
    let mut file = kernel.into_regular_file().unwrap();
    let file_info = file.get_info::<FileInfo>(&mut buffer).unwrap();
    let file_size = file_info.file_size() as usize;
    let mut file_buffer = vec![0; file_size];

    file.read(&mut file_buffer).unwrap();
    let data = String::from_utf8(file_buffer).expect("Failed to parse kernel file");

    Ok(data)
}
