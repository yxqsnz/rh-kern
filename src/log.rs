use uefi_services::println;

pub fn info(text: &str) {
    println!("(Information): {text}")
}

pub fn debug(text: &str) {
    println!("(Debug): {text}")
}
