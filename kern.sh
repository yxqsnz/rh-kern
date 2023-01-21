export EFI_PATH="target/x86_64-unknown-uefi/debug/rh-kern.efi"
export CARGO_TARGET_DIR=target
export VM_PATH="target/virtual-machine"
export OVMF_PATH="/usr/share/edk2-ovmf/x64"

echo "--> Building Kernel"
cargo build --target x86_64-unknown-uefi || exit


echo "--> Creating Fake filesystem"
mkdir -p $VM_PATH/efi/boot
cp $EFI_PATH $VM_PATH/efi/boot/bootx64.efi
cp kernel.rhai $VM_PATH

echo "--> Running Virtual Machine"
exec qemu-system-x86_64 \
    -drive if=pflash,format=raw,readonly=on,file=$OVMF_PATH/OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=$OVMF_PATH/OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:$VM_PATH

