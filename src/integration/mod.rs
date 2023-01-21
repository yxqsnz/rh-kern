use rhai::Engine;
use uefi_services::println;

fn uefi_puts(content: &str) {
    println!("{content}")
}

pub fn integrate(engine: &mut Engine) {
    engine.register_fn("uefi_puts", uefi_puts);
}
