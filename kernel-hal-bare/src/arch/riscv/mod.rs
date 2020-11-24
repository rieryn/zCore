// use self::riscv::*;
// use kernel_hal::{PageTableTrait, PhysAddr, VirtAddr};
// use crate::riscv::addr::{Page, frame};
// use crate::riscv::paging::{PageTableFlags as PTF, *};
// use crate::riscv::register::satp;

use core::time::Duration;
use alloc::boxed::Box;
use riscv;

mod io;
pub mod sbi;
pub mod timer;

pub use io::*;

#[export_name = "hal_timer_now"]
pub fn timer_now() -> Duration {
    timer::timer_now()
}

pub struct Config {
    pub dtb: usize,
}

pub fn init(_config: Config) {
    timer::init();
}

pub unsafe fn set_page_table(_vmtoken: usize) {
    unimplemented!();
}

#[export_name = "hal_serial_write"]
pub fn serial_write(s: &str) {
    for c in s.as_bytes() {
        putchar(*c);
    }
    // putfmt(format_args!("{}", s));
}

#[export_name = "hal_serial_read"]
pub fn serial_read(_buf: &mut [u8]) -> usize {
    unimplemented!();
}

#[export_name = "hal_serial_set_callback"]
pub fn serial_set_callback(_callback: Box<dyn Fn() -> bool + Send + Sync>) {
    unimplemented!()
}

pub fn wait_for_interrupt() {
    unsafe {
        // enable interrupt and disable
        let sie = riscv::register::sstatus::read().sie();
        riscv::register::sstatus::set_sie();
        riscv::asm::wfi();
        if !sie {
            riscv::register::sstatus::clear_sie();
        }
    }
}

#[export_name = "hal_apic_local_id"]
pub fn apic_local_id() -> u8 {
    0
}
