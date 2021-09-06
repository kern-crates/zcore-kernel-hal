#![allow(dead_code)]

use alloc::vec::Vec;

#[macro_use]
pub mod serial;

mod consts;
mod plic;
mod sbi;
mod trap;
mod uart;

pub mod context;
pub mod cpu;
pub mod interrupt;
pub mod mem;
pub mod special;
pub mod timer;
pub mod vm;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GraphicInfo {
    /// Graphic mode
    //pub mode: ModeInfo,
    pub mode: u64,
    /// Framebuffer base physical address
    pub fb_addr: u64,
    /// Framebuffer size
    pub fb_size: u64,
}

/// This structure represents the information that the bootloader passes to the kernel.
#[repr(C)]
#[derive(Debug)]
pub struct BootInfo {
    pub memory_map: Vec<u64>,
    //pub memory_map: Vec<&'static MemoryDescriptor>,
    /// The offset into the virtual address space where the physical memory is mapped.
    pub physical_memory_offset: u64,
    /// The graphic output information
    pub graphic_info: GraphicInfo,

    /// Physical address of ACPI2 RSDP, 启动的系统信息表的入口指针
    //pub acpi2_rsdp_addr: u64,
    /// Physical address of SMBIOS, 产品管理信息的结构表
    //pub smbios_addr: u64,
    pub hartid: u64,
    pub dtb_addr: u64,

    /// The start physical address of initramfs
    pub initramfs_addr: u64,
    /// The size of initramfs
    pub initramfs_size: u64,
    /// Kernel command line
    pub cmdline: &'static str,
}

pub struct HalConfig {
    pub mconfig: u64,
    pub dtb: usize,
}

pub fn init(config: HalConfig) {
    trace!("hal dtb: {:#x}", config.dtb);

    interrupt::init();
    timer::init();

    unsafe { asm!("ebreak") };

    #[cfg(feature = "board_qemu")]
    {
        // TODO
        // sbi_println!("Setup virtio @devicetree {:#x}", config.dtb);
        // drivers::virtio::device_tree::init(config.dtb);
    }
}
