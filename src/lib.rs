#![no_std]
#![cfg_attr(target_os = "uefi", no_main)]

extern crate alloc;

mod exfat;
mod protocol;

pub use exfat::*;
pub use protocol::*;

// Global allocator for UEFI
#[cfg(target_os = "uefi")]
use uefi::allocator::Allocator;

#[cfg(target_os = "uefi")]
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

#[cfg(target_os = "uefi")]
#[no_mangle]
pub extern "efiapi" fn efi_main(_image_handle: uefi::Handle, _system_table: *const ()) -> uefi::Status {
    // Initialize UEFI environment
    // The global allocator is available for use
    
    // Driver initialization would happen here
    // In a real implementation, this would register protocol handlers
    
    uefi::Status::SUCCESS
}

#[cfg(target_os = "uefi")]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

